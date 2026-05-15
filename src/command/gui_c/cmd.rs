// src/command/gui_c/cmd.rs
// GuiCCommand: slint（宣言型 DSL）を使ったウィンドウサンプル

use crate::common::commandnode::CommandNode;
use clap::ArgMatches;
use std::error::Error;

/// GuiCCommand: slint による宣言型 UI
/// アーキテクチャ: 独自の宣言型 DSL（.slint ファイル）でレイアウトを定義
/// 特徴: マークアップ言語でUIを記述し、Rust コードと双方向バインディングが可能
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
        "GUI サンプル C: slint（宣言型 DSL）"
    }

    fn command(&self) -> clap::Command {
        // 完全修飾で clap::Command を記述
        clap::Command::new(self.name())
            .about(self.about())
            .disable_help_subcommand(true)
    }

    fn execute(&self, _matches: &ArgMatches) -> Result<(), Box<dyn Error>> {
        println!("[gui-c] アーキテクチャ: slint（宣言型 DSL）");
        println!("  - .slint ファイルに UI レイアウトをマークアップで記述する");
        println!("  - Rust との双方向データバインディングをコンパイル時に型安全に実現");
        println!("  - TODO: slint::include_macros! で .slint ファイルを読み込みウィンドウを起動する");
        // TODO: 実装例（build.rs で slint ファイルをコンパイルする必要がある）
        // slint::include_macros!("src/command/gui_c/main.slint");
        // let ui = AppWindow::new()?;
        // ui.run()?;
        Ok(())
    }
}
