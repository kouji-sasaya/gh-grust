use std::any::Any;
use std::error::Error;
use std::io;
use std::panic::{self, AssertUnwindSafe};
use std::sync::{Arc, Mutex};

/// GUI フレームワーク内部の panic を通常のエラーへ変換し、
/// 不足ライブラリ時に原因が分かるメッセージを返す。
pub fn run_gui_command<F>(command_name: &str, launcher: F) -> Result<(), Box<dyn Error>>
where
    F: FnOnce() -> Result<(), Box<dyn Error>>,
{
    let captured_message = Arc::new(Mutex::new(None));
    let panic_hook_message = Arc::clone(&captured_message);
    let previous_hook = panic::take_hook();

    panic::set_hook(Box::new(move |panic_info| {
        let mut message = panic_hook_message
            .lock()
            .expect("failed to lock panic hook message");
        *message = Some(format!("{panic_info}"));
    }));

    let result = panic::catch_unwind(AssertUnwindSafe(launcher));
    panic::set_hook(previous_hook);

    match result {
        Ok(result) => result,
        Err(payload) => {
            let captured_message = captured_message
                .lock()
                .expect("failed to lock captured panic message")
                .clone();
            Err(io::Error::other(format_panic_message(
                command_name,
                payload,
                captured_message,
            ))
            .into())
        }
    }
}

fn format_panic_message(
    command_name: &str,
    payload: Box<dyn Any + Send>,
    captured_message: Option<String>,
) -> String {
    let panic_message = captured_message.unwrap_or_else(|| panic_payload_to_string(&payload));

    if panic_message.contains("libxkbcommon-x11.so") {
        return format!(
            "[{command_name}] GUI の起動に必要な共有ライブラリ libxkbcommon-x11.so が見つかりません。Linux では libxkbcommon-x11 系パッケージ（Ubuntu なら libxkbcommon-x11-0）をインストールしてから再実行してください。"
        );
    }

    format!("[{command_name}] GUI 初期化中に panic が発生しました: {panic_message}")
}

fn panic_payload_to_string(payload: &(dyn Any + Send)) -> String {
    if let Some(message) = payload.downcast_ref::<&'static str>() {
        return (*message).to_string();
    }

    if let Some(message) = payload.downcast_ref::<String>() {
        return message.clone();
    }

    "詳細不明の panic".to_string()
}
