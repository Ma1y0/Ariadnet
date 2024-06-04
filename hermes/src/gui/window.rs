use gtk::{glib, Application, Box, Button, Label};
use gtk::{prelude::*, ApplicationWindow};
use std::cell::Cell;

const APP_ID: &str = "org.web.hermes";

fn build_ui(app: &Application) {
    let button = Button::builder()
        .label("Click Me")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    let count = Cell::new(0);

    let label = Label::builder()
        .label(format!("{}", count.get()).as_str())
        .build();

    button.connect_clicked(move |_| {
        count.set(count.get() + 1);
        println!("{}", count.get());
    });

    let box_v = Box::builder()
        .margin_top(12)
        .orientation(gtk::Orientation::Vertical)
        .build();

    box_v.append(&label);
    box_v.append(&button);

    let window = ApplicationWindow::builder()
        .application(app)
        .title("Hermes the web browser of the future")
        .child(&box_v)
        .build();

    window.present();
}

pub fn init() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();

    app.connect_activate(build_ui);

    app.run()
}
