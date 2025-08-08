use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button, glib};

const APP_ID: &str = "com.smarniw.fetch";

fn main() -> glib::ExitCode {
    println!("Hello from fetch!");

    let app = Application::builder().application_id(APP_ID).build();

    app.connect_activate(build_ui);

    app.run()
}

fn build_ui(app: &Application) {
    let button = Button::builder()
        .label("press me!")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    button.connect_clicked(|button| button.set_label("pressed!"));

    let window = ApplicationWindow::builder()
        .application(app)
        .title("Fetch")
        .child(&button)
        .build();

    window.present();
}
