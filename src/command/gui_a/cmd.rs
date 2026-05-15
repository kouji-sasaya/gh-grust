// src/command/gui_a/cmd.rs
// GuiACommand: egui (Immediate Mode) window sample

use crate::common::commandnode::CommandNode;
use clap::ArgMatches;
use std::error::Error;

/// GuiACommand: egui / eframe Immediate Mode GUI
/// Architecture: Redraws entire UI every frame
/// Features: Simple state management. Good for games and development tools
pub struct GuiACommand;

impl GuiACommand {
    pub fn new() -> Self {
        GuiACommand
    }
}

impl CommandNode for GuiACommand {
    fn name(&self) -> &'static str {
        "gui-a"
    }

    fn about(&self) -> &'static str {
        "GUI Sample A: egui (Immediate Mode)"
    }

    fn command(&self) -> clap::Command {
        // 完全修飾で clap::Command を記述
        clap::Command::new(self.name())
            .about(self.about())
            .disable_help_subcommand(true)
    }

    fn execute(&self, _matches: &ArgMatches) -> Result<(), Box<dyn Error>> {
        // Set initial window size and title
        let options = eframe::NativeOptions {
            viewport: eframe::egui::ViewportBuilder::default()
                .with_title("gui-a: egui Immediate Mode Sample")
                .with_inner_size([520.0, 360.0]),
            ..Default::default()
        };

        // Launch window with eframe::run_native
        // Closure creates and returns App instance
        eframe::run_native(
            "com.example.gui-a",
            options,
            Box::new(|_cc| Ok(Box::new(super::app::GuiAApp::default()))),
        )?;

        Ok(())
    }
}
