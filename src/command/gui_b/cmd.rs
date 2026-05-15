// src/command/gui_b/cmd.rs
// GuiBCommand: iced（Elm アーキテクチャ / リアクティブ）を使ったウィンドウサンプル

use crate::common::commandnode::CommandNode;
use clap::ArgMatches;
use std::error::Error;

/// GuiBCommand: iced による Elm インスパイア型リアクティブ GUI
/// アーキテクチャ: Model-Update-View パターン（Elm アーキテクチャ）
/// 特徴: 関数型プログラミングスタイル。状態とビューが明確に分離される
pub struct GuiBCommand;

impl GuiBCommand {
    pub fn new() -> Self {
        GuiBCommand
    }
}

impl CommandNode for GuiBCommand {
    fn name(&self) -> &'static str {
        "gui-b"
    }

    fn about(&self) -> &'static str {
        "GUI サンプル B: iced（Elm アーキテクチャ / リアクティブ）"
    }

    fn command(&self) -> clap::Command {
        // 完全修飾で clap::Command を記述
        clap::Command::new(self.name())
            .about(self.about())
            .disable_help_subcommand(true)
    }

    fn execute(&self, _matches: &ArgMatches) -> Result<(), Box<dyn Error>> {
        println!("[gui-b] アーキテクチャ: iced（Elm アーキテクチャ / リアクティブ）");
        println!("  - Model（状態）/ Message（イベント）/ Update（状態遷移）/ View（描画）の4要素");
        println!("  - 状態変化はメッセージとして表現し、副作用を分離できる");
        println!("  - TODO: iced::application() でウィンドウを起動する");
        // TODO: 実装例
        // iced::application("gui-b: iced サンプル", GuiBApp::update, GuiBApp::view)
        //     .run()?;
        Ok(())
    }
}
