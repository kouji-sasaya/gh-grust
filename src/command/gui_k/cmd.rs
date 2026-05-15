// src/command/gui_k/cmd.rs
// GuiKCommand: ratatui（TUI / ターミナル UI）を使ったウィンドウサンプル

use crate::common::commandnode::CommandNode;
use clap::ArgMatches;
use std::error::Error;

/// GuiKCommand: ratatui によるターミナル UI（TUI）
/// アーキテクチャ: ターミナル上に即時描画でウィジェットを配置するイベントループ型
/// 特徴: ウィンドウシステム不要。SSH 接続先やサーバー環境でも動作する。
///       crossterm でキー入力・端末サイズ変更などのイベントを処理する
pub struct GuiKCommand;

impl GuiKCommand {
    pub fn new() -> Self {
        GuiKCommand
    }
}

impl CommandNode for GuiKCommand {
    fn name(&self) -> &'static str {
        "gui-k"
    }

    fn about(&self) -> &'static str {
        "GUI サンプル K: ratatui（TUI / ターミナル UI・ウィンドウ不要）"
    }

    fn command(&self) -> clap::Command {
        // 完全修飾で clap::Command を記述
        clap::Command::new(self.name())
            .about(self.about())
            .disable_help_subcommand(true)
    }

    fn execute(&self, _matches: &ArgMatches) -> Result<(), Box<dyn Error>> {
        println!("[gui-k] アーキテクチャ: ratatui（TUI / ターミナル UI）");
        println!("  - ウィンドウシステム不要。ターミナル上に文字ベースの UI を描画する");
        println!("  - crossterm でキー入力・端末リサイズ等のイベントを処理するイベントループ型");
        println!("  - Block / Paragraph / Table / Chart など豊富なウィジェットを持つ");
        println!("  - SSH 接続先・CI 環境・サーバー上でも動作するのが最大の特徴");
        println!("  - TODO: Terminal::new() → terminal.draw(|f| {{ ... }}) でレンダリングループを起動する");
        // TODO: 実装例
        // let mut terminal = ratatui::init();
        // loop {
        //     terminal.draw(|frame| {
        //         let area = frame.area();
        //         frame.render_widget(
        //             ratatui::widgets::Paragraph::new("gui-k: ratatui サンプル"),
        //             area,
        //         );
        //     })?;
        //     if matches!(crossterm::event::read()?, crossterm::event::Event::Key(_)) {
        //         break;
        //     }
        // }
        // ratatui::restore();
        Ok(())
    }
}
