// src/command/gui_h/cmd.rs
// GuiHCommand: Dioxus（React インスパイア型 / 仮想 DOM）を使ったウィンドウサンプル

use crate::common::commandnode::CommandNode;
use clap::ArgMatches;
use std::error::Error;

/// GuiHCommand: Dioxus による React インスパイア型 GUI
/// アーキテクチャ: 仮想 DOM + コンポーネントベース（React / JSX に相当する RSX マクロ）
/// 特徴: デスクトップ・Web（WASM）・モバイルを同一コードで対応できるクロスプラットフォーム設計
pub struct GuiHCommand;

impl GuiHCommand {
    pub fn new() -> Self {
        GuiHCommand
    }
}

impl CommandNode for GuiHCommand {
    fn name(&self) -> &'static str {
        "gui-h"
    }

    fn about(&self) -> &'static str {
        "GUI サンプル H: Dioxus（React インスパイア型 / 仮想 DOM・コンポーネント）"
    }

    fn command(&self) -> clap::Command {
        // 完全修飾で clap::Command を記述
        clap::Command::new(self.name())
            .about(self.about())
            .disable_help_subcommand(true)
    }

    fn execute(&self, _matches: &ArgMatches) -> Result<(), Box<dyn Error>> {
        println!("[gui-h] アーキテクチャ: Dioxus（React インスパイア型 / 仮想 DOM）");
        println!("  - rsx! マクロで JSX 風にコンポーネントツリーを宣言的に記述する");
        println!("  - use_signal / use_state などのフックで状態管理（React Hooks 相当）");
        println!("  - desktop / web(WASM) / mobile を同一コードで対応できる");
        println!("  - TODO: dioxus::launch() でデスクトップウィンドウを起動する");
        // TODO: 実装例
        // fn app() -> dioxus::prelude::Element {
        //     let mut count = dioxus::prelude::use_signal(|| 0);
        //     rsx! {
        //         h1 { "gui-h: Dioxus サンプル" }
        //         button { onclick: move |_| count += 1, "カウント: {count}" }
        //     }
        // }
        // dioxus::launch(app);
        Ok(())
    }
}
