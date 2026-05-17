// src/command/gui_f/cmd.rs
// GuiFCommand: fltk（FLTK ウィジェットツールキット）を使ったウィンドウサンプル

use crate::common::commandnode::CommandNode;
use crate::common::gui_runtime;
use clap::ArgMatches;
use std::error::Error;

/// GuiFCommand: fltk-rs による FLTK ウィジェットツールキット
/// アーキテクチャ: コールバック型ウィジェット（手続き型スタイル）
/// 特徴: 軽量・高速で依存が少ない。組み込み系やクロスプラットフォーム用途に向く
pub struct GuiFCommand;

impl GuiFCommand {
    pub fn new() -> Self {
        GuiFCommand
    }
}

impl CommandNode for GuiFCommand {
    fn name(&self) -> &'static str {
        "gui-f"
    }

    fn about(&self) -> &'static str {
        "GUI サンプル F: fltk（軽量ウィジェット / コールバック型）"
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
