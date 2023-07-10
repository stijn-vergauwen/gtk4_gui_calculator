use gtk4::prelude::*;
use gtk4::Application;

mod gui;

const APPLICATION_ID: &str = "com.example.calculator";

fn main() {
    let application = Application::builder()
        .application_id(APPLICATION_ID)
        .build();

    application.connect_activate(|app| {
        gui::build_ui(&app);
    });

    application.run();
}
