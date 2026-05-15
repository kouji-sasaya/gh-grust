// src/command/gui_b/cmd.rs
// GuiBCommand: iced (Elm Architecture / Reactive) window sample

use crate::common::commandnode::CommandNode;
use clap::ArgMatches;
use std::error::Error;

/// GuiBCommand: iced Elm Architecture GUI
/// Architecture: Model-Message-Update-View pattern (Elm inspired)
/// Features: Functional programming style. State and view clearly separated
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
        "GUI Sample B: iced (Elm Architecture / Reactive)"
    }

    fn command(&self) -> clap::Command {
        // Complete qualifier for clap::Command
        clap::Command::new(self.name())
            .about(self.about())
            .disable_help_subcommand(true)
    }

    fn execute(&self, _matches: &ArgMatches) -> Result<(), Box<dyn Error>> {
        // iced アプリケーションを起動（iced 0.13 の関数型 API を使用）
        iced::application(
            "gui-b: iced Elm Architecture",
            super::app::GuiBApp::update,
            super::app::GuiBApp::view,
        )
        .window(iced::window::Settings {
            size: iced::Size::new(520.0, 360.0),
            ..Default::default()
        })
        .run()?;

        Ok(())
    }
}
