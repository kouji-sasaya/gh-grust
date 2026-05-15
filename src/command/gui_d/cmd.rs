// src/command/gui_d/cmd.rs
// GuiDCommand: winit（低レベルウィンドウ管理）を使ったウィンドウサンプル

use crate::common::commandnode::CommandNode;
use clap::ArgMatches;
use std::error::Error;

/// GuiDCommand: winit + softbuffer による低レベルウィンドウ
/// アーキテクチャ: OS のウィンドウシステムへの薄いラッパー（イベントループ直接制御）
/// 特徴: GUI フレームワークを使わず、ピクセルバッファを直接操作して描画する最小構成
pub struct GuiDCommand;

impl GuiDCommand {
    pub fn new() -> Self {
        GuiDCommand
    }
}

impl CommandNode for GuiDCommand {
    fn name(&self) -> &'static str {
        "gui-d"
    }

    fn about(&self) -> &'static str {
        "GUI サンプル D: winit（低レベルウィンドウ管理）"
    }

    fn command(&self) -> clap::Command {
        // 完全修飾で clap::Command を記述
        clap::Command::new(self.name())
            .about(self.about())
            .disable_help_subcommand(true)
    }

    fn execute(&self, _matches: &ArgMatches) -> Result<(), Box<dyn Error>> {
        println!("[gui-d] アーキテクチャ: winit（低レベルウィンドウ管理）");
        println!("  - OS のウィンドウシステムへの薄いラッパー");
        println!("  - EventLoop でキーボード／マウス／リサイズなどのイベントを直接処理する");
        println!("  - softbuffer と組み合わせてピクセルバッファを直接書き込んで描画する");
        println!("  - TODO: EventLoop::new() → Window::new() → softbuffer でピクセル描画");
        // TODO: 実装例
        // let event_loop = winit::event_loop::EventLoop::new()?;
        // let window = winit::window::WindowBuilder::new()
        //     .with_title("gui-d: winit サンプル")
        //     .build(&event_loop)?;
        // event_loop.run(move |event, target| { ... })?;
        Ok(())
    }
}
