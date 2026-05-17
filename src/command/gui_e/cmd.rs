// src/command/gui_e/cmd.rs
// GuiECommand: gtk4（ネイティブウィジェット）を使ったウィンドウサンプル

use crate::common::commandnode::CommandNode;
use crate::common::gui_runtime;
use clap::ArgMatches;
use std::error::Error;

/// GuiECommand: gtk4-rs によるネイティブ GTK4 ウィジェット
/// アーキテクチャ: シグナル＆スロット型のウィジェットツリー（オブジェクト指向）
/// 特徴: OS ネイティブのウィジェットを使い、プラットフォーム標準の見た目になる
pub struct GuiECommand;

impl GuiECommand {
    pub fn new() -> Self {
        GuiECommand
    }
}

impl CommandNode for GuiECommand {
    fn name(&self) -> &'static str {
        "gui-e"
    }

    fn about(&self) -> &'static str {
        "GUI サンプル E: gtk4（ネイティブウィジェット / シグナル＆スロット）"
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
