use fltk::{
    app,
    button::Button,
    enums::{Align, CallbackTrigger, Color, FrameType},
    frame::Frame,
    group::Flex,
    input::Input,
    misc::Progress,
    prelude::*,
    valuator::HorNiceSlider,
    window::Window,
};
use std::cell::RefCell;
use std::error::Error;
use std::rc::Rc;

const DEFAULT_TEXT: &str = "Hello, FLTK!";

pub fn run() -> Result<(), Box<dyn Error>> {
    app::set_scheme(app::Scheme::Gtk);
    app::background(245, 245, 245);
    app::foreground(32, 32, 32);
    app::set_font_size(16);

    let app = app::App::default();
    let state = Rc::new(RefCell::new(GuiFState::default()));

    let mut window = Window::default()
        .with_size(520, 360)
        .with_label("gui-f: FLTK Callback Sample");

    let mut root = Flex::default_fill().column();
    root.set_margin(20);
    root.set_pad(12);

    let mut title = Frame::default().with_label("gui-f: FLTK Callback Sample");
    title.set_label_size(24);
    title.set_align(Align::Inside | Align::Left);
    root.fixed(&title, 36);

    let mut description = Frame::default()
        .with_label("FLTK の軽量ウィジェットをコールバックでつなぐ GUI サンプルです。");
    description.set_align(Align::Inside | Align::Left | Align::Wrap);
    root.fixed(&description, 40);

    let mut input = Input::default();
    input.set_value(DEFAULT_TEXT);
    input.set_trigger(CallbackTrigger::Changed);
    root.fixed(&input, 32);

    let mut preview = Frame::default().with_label(DEFAULT_TEXT);
    preview.set_frame(FrameType::DownBox);
    preview.set_align(Align::Inside | Align::Left | Align::Wrap);
    root.fixed(&preview, 44);

    let mut slider = HorNiceSlider::default();
    slider.set_range(0.0, 1.0);
    slider.set_value(0.5);
    slider.set_color(Color::from_rgb(220, 220, 220));
    root.fixed(&slider, 32);

    let mut progress_label = Frame::default().with_label("Progress: 50%");
    progress_label.set_align(Align::Inside | Align::Left);
    root.fixed(&progress_label, 24);

    let mut progress = Progress::default();
    progress.set_minimum(0.0);
    progress.set_maximum(1.0);
    progress.set_value(0.5);
    progress.set_selection_color(Color::from_rgb(80, 140, 240));
    root.fixed(&progress, 28);

    let mut count_label = Frame::default().with_label("Clicked: 0");
    count_label.set_align(Align::Inside | Align::Left);
    root.fixed(&count_label, 24);

    let mut status_label = Frame::default().with_label("Status: Ready");
    status_label.set_align(Align::Inside | Align::Left);
    root.fixed(&status_label, 24);

    let mut button_row = Flex::default().row();
    button_row.set_pad(8);
    let mut apply_button = Button::default().with_label("Apply");
    let mut increment_button = Button::default().with_label("Increment");
    let mut reset_button = Button::default().with_label("Reset");
    button_row.fixed(&apply_button, 120);
    button_row.fixed(&increment_button, 120);
    button_row.fixed(&reset_button, 120);
    button_row.end();
    root.fixed(&button_row, 36);

    root.end();
    window.end();
    window.make_resizable(true);
    window.show();

    {
        let state = Rc::clone(&state);
        let mut preview = preview.clone();
        let mut status_label = status_label.clone();
        input.set_callback(move |input| {
            let text = input.value();
            preview.set_label(&text);
            state.borrow_mut().current_text = text;
            status_label.set_label("Status: Entry updated");
        });
    }

    {
        let mut progress = progress.clone();
        let mut progress_label = progress_label.clone();
        let mut status_label = status_label.clone();
        slider.set_callback(move |slider| {
            let value = slider.value().clamp(0.0, 1.0);
            progress.set_value(value);
            progress_label.set_label(&format!("Progress: {:.0}%", value * 100.0));
            status_label.set_label("Status: Slider changed");
        });
    }

    {
        let state = Rc::clone(&state);
        let input = input.clone();
        let mut preview = preview.clone();
        let mut status_label = status_label.clone();
        apply_button.set_callback(move |_| {
            let text = input.value();
            preview.set_label(&text);
            state.borrow_mut().current_text = text;
            status_label.set_label("Status: Apply clicked");
        });
    }

    {
        let state = Rc::clone(&state);
        let mut count_label = count_label.clone();
        let mut status_label = status_label.clone();
        increment_button.set_callback(move |_| {
            let mut state = state.borrow_mut();
            state.click_count = state.click_count.wrapping_add(1);
            count_label.set_label(&format!("Clicked: {}", state.click_count));
            status_label.set_label("Status: Increment clicked");
        });
    }

    {
        let state = Rc::clone(&state);
        let mut input = input.clone();
        let mut preview = preview.clone();
        let mut slider = slider.clone();
        let mut progress = progress.clone();
        let mut progress_label = progress_label.clone();
        let mut count_label = count_label.clone();
        let mut status_label = status_label.clone();
        reset_button.set_callback(move |_| {
            let mut state = state.borrow_mut();
            state.click_count = 0;
            state.current_text = DEFAULT_TEXT.to_string();
            input.set_value(DEFAULT_TEXT);
            preview.set_label(DEFAULT_TEXT);
            slider.set_value(0.5);
            progress.set_value(0.5);
            progress_label.set_label("Progress: 50%");
            count_label.set_label("Clicked: 0");
            status_label.set_label("Status: Reset completed");
        });
    }

    app.run()?;
    Ok(())
}

#[derive(Default)]
struct GuiFState {
    click_count: u32,
    current_text: String,
}
