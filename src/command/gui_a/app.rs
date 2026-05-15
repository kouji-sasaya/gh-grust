// src/command/gui_a/app.rs
// GuiAApp: egui Immediate Mode demo app

use eframe::egui;

/// GuiAApp: Immediate Mode Demo
/// Redraws entire UI every frame in update()
pub struct GuiAApp {
    /// Text input field value
    label: String,
    /// Slider value
    value: f32,
    /// Button click count
    count: u32,
    /// Background color
    bg_color: egui::Color32,
}

impl Default for GuiAApp {
    fn default() -> Self {
        Self {
            label: "Hello, egui!".to_owned(),
            value: 0.5,
            count: 0,
            bg_color: egui::Color32::from_rgb(30, 30, 30),  // Dark gray default
        }
    }
}

impl eframe::App for GuiAApp {
    /// Rendered every frame (core of Immediate Mode)
    /// State (self) is referenced and UI widgets are placed on the fly
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default()
            .frame(egui::Frame::default().fill(self.bg_color))
            .show(ctx, |ui| {
            // Heading
            ui.heading("gui-a: egui Immediate Mode Sample");
            ui.separator();

            ui.label(
                "In Immediate Mode, the entire UI is reconstructed every frame.\n\
                 The state is held by the App structure, and the UI is built frame by frame in update().",
            );
            ui.add_space(12.0);

            // Text input
            ui.horizontal(|ui| {
                ui.label("Text:");
                ui.text_edit_singleline(&mut self.label);
            });

            // Slider
            ui.add(egui::Slider::new(&mut self.value, 0.0..=1.0).text("Slider"));

            ui.add_space(8.0);

            // Button (click counter)
            if ui
                .button(format!("Click (count: {})", self.count))
                .clicked()
            {
                self.count += 1;
                self.label = format!("Clicked {} times!", self.count);
            }

            ui.add_space(12.0);
            ui.separator();

            // Background color preset buttons
            ui.group(|ui| {
                ui.label("Background Color:");
                ui.horizontal(|ui| {
                    if ui.button("Red").clicked() {
                        self.bg_color = egui::Color32::from_rgb(60, 20, 20);
                    }
                    if ui.button("Green").clicked() {
                        self.bg_color = egui::Color32::from_rgb(20, 60, 20);
                    }
                    if ui.button("Blue").clicked() {
                        self.bg_color = egui::Color32::from_rgb(20, 20, 60);
                    }
                    if ui.button("Reset").clicked() {
                        self.bg_color = egui::Color32::from_rgb(30, 30, 30);
                    }
                });
            });

            ui.add_space(12.0);
            ui.separator();

            // Display current state
            ui.label(format!("Label: {}", self.label));
            ui.label(format!("Slider Value: {:.2}", self.value));
        });
    }
}
