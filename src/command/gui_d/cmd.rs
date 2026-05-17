// src/command/gui_d/cmd.rs
// GuiDCommand: winit（低レベルウィンドウ管理）を使ったウィンドウサンプル

use crate::common::commandnode::CommandNode;
use crate::common::gui_runtime;
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
        gui_runtime::run_gui_command(self.name(), super::app::run)
    }
}
