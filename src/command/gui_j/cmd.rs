// src/command/gui_j/cmd.rs
// GuiJCommand: Relm4（GTK4 + Elm アーキテクチャ）を使ったウィンドウサンプル

use crate::common::commandnode::CommandNode;
use clap::ArgMatches;
use std::error::Error;

/// GuiJCommand: Relm4 による GTK4 + Elm アーキテクチャ
/// アーキテクチャ: GTK4 をバックエンドとしながら Elm ライクな Model-Update-View を提供
/// 特徴: gui-e（生 GTK4）と異なり、状態管理とメッセージパッシングが型安全かつ構造化されている
pub struct GuiJCommand;

impl GuiJCommand {
    pub fn new() -> Self {
        GuiJCommand
    }
}

impl CommandNode for GuiJCommand {
    fn name(&self) -> &'static str {
        "gui-j"
    }

    fn about(&self) -> &'static str {
        "GUI サンプル J: Relm4（GTK4 + Elm アーキテクチャ / 型安全メッセージパッシング）"
    }

    fn command(&self) -> clap::Command {
        // 完全修飾で clap::Command を記述
        clap::Command::new(self.name())
            .about(self.about())
            .disable_help_subcommand(true)
    }

    fn execute(&self, _matches: &ArgMatches) -> Result<(), Box<dyn Error>> {
        println!("[gui-j] アーキテクチャ: Relm4（GTK4 + Elm アーキテクチャ）");
        println!("  - GTK4 の描画エンジンを使いつつ、Elm ライクな構造でコードを整理する");
        println!("  - Component トレイトで Model / Update / View を一箇所に集約する");
        println!("  - gui-e（生 GTK4）より型安全で、状態管理が明確になる");
        println!("  - TODO: RelmApp::new() → MainWindow::builder() でウィンドウを起動する");
        // TODO: 実装例（要: libgtk-4-dev パッケージ）
        // struct AppModel { count: u8 }
        // #[derive(Debug)] enum AppMsg { Increment, Decrement }
        // impl SimpleComponent for AppModel { ... }
        //
        // let app = RelmApp::new("com.example.gui-j");
        // app.run::<AppModel>(0);
        Ok(())
    }
}
