// src/command/gui_b/app.rs
// GuiBApp: iced Elm Architecture (Model-Message-Update-View) demo

use iced::widget::{button, column, container, row, slider, text};
use iced::{Color, Element, Length, Task};

/// GuiBApp: Elm Architecture Demo (Model-Update-View)
/// Message patterns decouple state transitions from rendering
pub struct GuiBApp {
    /// Button click count
    count: u32,
    /// Text input value
    label: String,
    /// Slider value
    value: f32,
    /// Background color (RGB components)
    bg_color: (f32, f32, f32),
}

/// Message: Events triggered by user interactions
#[derive(Debug, Clone, Copy)]
pub enum Message {
    /// Increment button clicked
    IncrementPressed,
    /// Decrement button clicked
    DecrementPressed,
    /// Red color button clicked
    RedPressed,
    /// Green color button clicked
    GreenPressed,
    /// Blue color button clicked
    BluePressed,
    /// Reset color button clicked
    ResetPressed,
    /// Slider value changed
    SliderChanged(f32),
}

// Text input message would need String, which doesn't implement Copy
// For simplicity, we'll skip text input for now

impl Default for GuiBApp {
    fn default() -> Self {
        GuiBApp {
            count: 0,
            label: "Elm Architecture Demo".to_string(),
            value: 0.5,
            bg_color: (0.12, 0.12, 0.12), // ダークグレー
        }
    }
}

impl GuiBApp {
    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::IncrementPressed => {
                self.count += 1;
                self.label = format!("Count: {}", self.count);
            }
            Message::DecrementPressed => {
                if self.count > 0 {
                    self.count -= 1;
                    self.label = format!("Count: {}", self.count);
                }
            }
            Message::SliderChanged(value) => {
                self.value = value;
            }
            Message::RedPressed => {
                self.bg_color = (0.24, 0.08, 0.08);
            }
            Message::GreenPressed => {
                self.bg_color = (0.08, 0.24, 0.08);
            }
            Message::BluePressed => {
                self.bg_color = (0.08, 0.08, 0.24);
            }
            Message::ResetPressed => {
                self.bg_color = (0.12, 0.12, 0.12);
            }
        }
        Task::none()
    }

    pub fn view(&self) -> Element<'_, Message> {
        let bg_color = Color::from_rgb(self.bg_color.0, self.bg_color.1, self.bg_color.2);

        let content = column![
            text("gui-b: iced Elm Architecture Sample").size(32),
            text("Model-Message-Update-View Pattern"),
            text("State changes are decoupled from rendering").size(14),
            row![
                button("Increment").on_press(Message::IncrementPressed),
                button("Decrement").on_press(Message::DecrementPressed),
                text(format!("Count: {}", self.count)).width(Length::Fixed(100.0)),
            ]
            .spacing(10),
            slider(0.0..=1.0, self.value, Message::SliderChanged).width(Length::Fixed(300.0)),
            text(format!("Slider: {:.2}", self.value)),
            row![
                button("Red").on_press(Message::RedPressed),
                button("Green").on_press(Message::GreenPressed),
                button("Blue").on_press(Message::BluePressed),
                button("Reset").on_press(Message::ResetPressed),
            ]
            .spacing(10),
            text(format!("Label: {}", self.label)),
        ]
        .spacing(20)
        .padding(20)
        .width(Length::Fill)
        .height(Length::Fill);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .style(move |_| iced::widget::container::Style {
                background: Some(iced::Background::Color(bg_color)),
                ..Default::default()
            })
            .into()
    }
}
