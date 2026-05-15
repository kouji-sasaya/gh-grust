// src/command/gui_i/cmd.rs
// GuiICommand: Vizia（リアクティブ + CSS スタイリング）を使ったウィンドウサンプル

use crate::common::commandnode::CommandNode;
use clap::ArgMatches;
use std::error::Error;

/// GuiICommand: Vizia によるリアクティブ + CSS スタイリング
/// アーキテクチャ: レンズ（Lens）パターンによるデータバインディング + CSS ライクなスタイリング
/// 特徴: CSS でウィジェットのスタイルを定義でき、データモデルの変化が自動的に UI に反映される
pub struct GuiICommand;

impl GuiICommand {
    pub fn new() -> Self {
        GuiICommand
    }
}

impl CommandNode for GuiICommand {
    fn name(&self) -> &'static str {
        "gui-i"
    }

    fn about(&self) -> &'static str {
        "GUI サンプル I: Vizia（リアクティブ + CSS スタイリング / Lens パターン）"
    }

    fn command(&self) -> clap::Command {
        // 完全修飾で clap::Command を記述
        clap::Command::new(self.name())
            .about(self.about())
            .disable_help_subcommand(true)
    }

    fn execute(&self, _matches: &ArgMatches) -> Result<(), Box<dyn Error>> {
        println!("[gui-i] アーキテクチャ: Vizia（リアクティブ + CSS スタイリング）");
        println!("  - Lens パターンでデータモデルの一部を UI にバインドし自動更新する");
        println!("  - CSS ファイルでウィジェットの見た目をスタイリングできる");
        println!("  - Model / View / Lens の3層構造で関心を分離する");
        println!("  - TODO: Application::new() → cx.add_stylesheet() でウィンドウを起動する");
        // TODO: 実装例
        // #[derive(Lens)]
        // struct AppData { count: i32 }
        // impl Model for AppData {}
        //
        // vizia::prelude::Application::new(|cx| {
        //     AppData { count: 0 }.build(cx);
        //     Label::new(cx, AppData::count);
        // })
        // .title("gui-i: Vizia サンプル")
        // .run();
        Ok(())
    }
}
