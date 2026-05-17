// src/command/gui_g/cmd.rs
// GuiGCommand: Tauri（Webview ベース）を使ったウィンドウサンプル

use crate::common::commandnode::CommandNode;
use clap::ArgMatches;
use std::error::Error;

/// GuiGCommand: Tauri による Webview ベース GUI
/// アーキテクチャ: HTML/CSS/JS をフロントエンドとして使い、Rust をバックエンドとする Web 技術型
/// 特徴: Web 技術で UI を構築しつつ、Rust でシステムアクセス・ロジックを担当する
///       コマンドシステムで Rust 側の関数を JS から呼び出せる（IPC）
pub struct GuiGCommand;

impl GuiGCommand {
    pub fn new() -> Self {
        GuiGCommand
    }
}

impl CommandNode for GuiGCommand {
    fn name(&self) -> &'static str {
        "gui-g"
    }

    fn about(&self) -> &'static str {
        "GUI サンプル G: Tauri（Webview / HTML・CSS・JS フロントエンド + Rust バックエンド）"
    }

    fn command(&self) -> clap::Command {
        // 完全修飾で clap::Command を記述
        clap::Command::new(self.name())
            .about(self.about())
            .disable_help_subcommand(true)
    }

    fn execute(&self, _matches: &ArgMatches) -> Result<(), Box<dyn Error>> {
        println!("[gui-g] アーキテクチャ: Tauri（Webview ベース）");
        println!("  - フロントエンド: HTML / CSS / JS（任意の Web フレームワーク可）");
        println!(
            "  - バックエンド: Rust（#[tauri::command] でコマンドを定義し JS から IPC 呼び出し）"
        );
        println!("  - OS のネイティブ Webview を使うためバイナリサイズが小さい");
        println!("  - TODO: tauri::Builder::default().run() でアプリを起動する");
        // TODO: 実装例（Tauri はプロジェクト構造が特殊なため cargo-tauri で初期化が必要）
        // #[tauri::command]
        // fn greet(name: &str) -> String { format!("Hello, {}!", name) }
        //
        // tauri::Builder::default()
        //     .invoke_handler(tauri::generate_handler![greet])
        //     .run(tauri::generate_context!())
        //     .expect("Tauri アプリの起動に失敗しました");
        Ok(())
    }
}
