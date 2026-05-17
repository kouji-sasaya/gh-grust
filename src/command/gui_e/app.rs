use gtk4::prelude::*;
use gtk4::{
    Align, Application, ApplicationWindow, Box as GtkBox, Button, Entry, Label, Orientation,
    ProgressBar, Scale,
};
use std::cell::{Cell, RefCell};
use std::error::Error;
use std::io;
use std::rc::Rc;

pub fn run() -> Result<(), Box<dyn Error>> {
    let application = Application::builder()
        .application_id("com.example.gui-e")
        .build();

    application.connect_activate(build_ui);

    // clap が処理済みのサブコマンド引数を GTK 側へ渡さないようにする。
    let exit_code = application.run_with_args(&["gh-grust"]);
    if exit_code == gtk4::glib::ExitCode::SUCCESS {
        Ok(())
    } else {
        Err(io::Error::other(format!(
            "[gui-e] GTK4 アプリケーションが異常終了しました: {exit_code:?}"
        ))
        .into())
    }
}

fn build_ui(app: &Application) {
    let click_count = Rc::new(Cell::new(0_u32));
    let current_text = Rc::new(RefCell::new(String::from("Hello, GTK4!")));

    let window = ApplicationWindow::builder()
        .application(app)
        .title("gui-e: GTK4 Native Widget Sample")
        .default_width(520)
        .default_height(360)
        .build();

    let root = GtkBox::new(Orientation::Vertical, 12);
    root.set_margin_top(20);
    root.set_margin_bottom(20);
    root.set_margin_start(20);
    root.set_margin_end(20);

    let title = Label::new(Some("gui-e: GTK4 Native Widget Sample"));
    title.set_halign(Align::Start);
    title.set_margin_bottom(4);
    title.add_css_class("title-3");

    let description = Label::new(Some(
        "GTK4 のネイティブウィジェットをシグナルで接続する GUI サンプルです。",
    ));
    description.set_halign(Align::Start);
    description.set_wrap(true);

    let entry = Entry::builder()
        .text("Hello, GTK4!")
        .placeholder_text("表示したいメッセージを入力")
        .hexpand(true)
        .build();

    let preview_label = Label::new(Some("Hello, GTK4!"));
    preview_label.set_halign(Align::Start);
    preview_label.set_wrap(true);
    preview_label.set_margin_bottom(4);

    let slider = Scale::with_range(Orientation::Horizontal, 0.0, 1.0, 0.01);
    slider.set_hexpand(true);
    slider.set_value(0.5);
    slider.set_draw_value(true);

    let progress_label = Label::new(Some("Progress: 50%"));
    progress_label.set_halign(Align::Start);

    let progress_bar = ProgressBar::new();
    progress_bar.set_fraction(0.5);
    progress_bar.set_show_text(true);
    progress_bar.set_text(Some("50%"));

    let count_label = Label::new(Some("Clicked: 0"));
    count_label.set_halign(Align::Start);

    let status_label = Label::new(Some("Status: Ready"));
    status_label.set_halign(Align::Start);

    let button_row = GtkBox::new(Orientation::Horizontal, 8);
    let apply_button = Button::with_label("Apply");
    let increment_button = Button::with_label("Increment");
    let reset_button = Button::with_label("Reset");
    button_row.append(&apply_button);
    button_row.append(&increment_button);
    button_row.append(&reset_button);

    {
        let preview_label = preview_label.clone();
        let status_label = status_label.clone();
        let current_text = Rc::clone(&current_text);
        entry.connect_changed(move |entry| {
            let text = entry.text().to_string();
            preview_label.set_text(&text);
            *current_text.borrow_mut() = text;
            status_label.set_text("Status: Entry updated");
        });
    }

    {
        let progress_bar = progress_bar.clone();
        let progress_label = progress_label.clone();
        let status_label = status_label.clone();
        slider.connect_value_changed(move |scale| {
            let value = scale.value().clamp(0.0, 1.0);
            progress_bar.set_fraction(value);
            let text = format!("{:.0}%", value * 100.0);
            progress_bar.set_text(Some(&text));
            progress_label.set_text(&format!("Progress: {text}"));
            status_label.set_text("Status: Slider changed");
        });
    }

    {
        let entry = entry.clone();
        let preview_label = preview_label.clone();
        let status_label = status_label.clone();
        let current_text = Rc::clone(&current_text);
        apply_button.connect_clicked(move |_| {
            let text = entry.text().to_string();
            preview_label.set_text(&text);
            *current_text.borrow_mut() = text;
            status_label.set_text("Status: Apply clicked");
        });
    }

    {
        let count_label = count_label.clone();
        let status_label = status_label.clone();
        let click_count = Rc::clone(&click_count);
        increment_button.connect_clicked(move |_| {
            let next = click_count.get().wrapping_add(1);
            click_count.set(next);
            count_label.set_text(&format!("Clicked: {next}"));
            status_label.set_text("Status: Increment clicked");
        });
    }

    {
        let entry = entry.clone();
        let preview_label = preview_label.clone();
        let slider = slider.clone();
        let progress_bar = progress_bar.clone();
        let progress_label = progress_label.clone();
        let count_label = count_label.clone();
        let status_label = status_label.clone();
        let click_count = Rc::clone(&click_count);
        let current_text = Rc::clone(&current_text);
        reset_button.connect_clicked(move |_| {
            let default_text = String::from("Hello, GTK4!");
            click_count.set(0);
            *current_text.borrow_mut() = default_text.clone();
            entry.set_text(&default_text);
            preview_label.set_text(&default_text);
            slider.set_value(0.5);
            progress_bar.set_fraction(0.5);
            progress_bar.set_text(Some("50%"));
            progress_label.set_text("Progress: 50%");
            count_label.set_text("Clicked: 0");
            status_label.set_text("Status: Reset completed");
        });
    }

    root.append(&title);
    root.append(&description);
    root.append(&entry);
    root.append(&preview_label);
    root.append(&slider);
    root.append(&progress_label);
    root.append(&progress_bar);
    root.append(&count_label);
    root.append(&status_label);
    root.append(&button_row);

    window.set_child(Some(&root));
    window.present();
}
