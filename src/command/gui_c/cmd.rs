// src/command/gui_c/cmd.rs
// GuiCCommand: Slint（宣言型 DSL）を使ったウィンドウサンプル

use crate::common::commandnode::CommandNode;
use crate::common::gui_runtime;
use clap::ArgMatches;
use std::error::Error;

// build.rs で生成された .slint の Rust バインディングをこのモジュールへ読み込む。
slint::include_modules!();

/// GuiCCommand: Slint による宣言型 UI
/// アーキテクチャ: 独自の宣言型 DSL（.slint ファイル）でレイアウトを定義
/// 特徴: マークアップ言語で UI を記述し、Rust コードと双方向バインディングが可能
pub struct GuiCCommand;

impl GuiCCommand {
    pub fn new() -> Self {
        GuiCCommand
    }
}

impl CommandNode for GuiCCommand {
    fn name(&self) -> &'static str {
        "gui-c"
    }

    fn about(&self) -> &'static str {
        "GUI Sample C: Slint (Declarative DSL)"
    }

    fn command(&self) -> clap::Command {
        // clap::Command は規約どおり完全修飾で組み立てる。
        clap::Command::new(self.name())
            .about(self.about())
            .disable_help_subcommand(true)
    }

    fn execute(&self, _matches: &ArgMatches) -> Result<(), Box<dyn Error>> {
        gui_runtime::run_gui_command(self.name(), || {
            let ui = GuiCWindow::new()?;
            ui.run()?;
            Ok(())
        })
    }
}
