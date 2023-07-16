use std::cell::RefCell;
use std::rc::Rc;

use gtk4::prelude::*;
use gtk4::Application;
use gtk4::Label;
use gui_calculator::calculator_app::Calculator;

mod gui;

const APPLICATION_ID: &str = "com.example.calculator";

fn main() {
    let application = Application::builder()
        .application_id(APPLICATION_ID)
        .build();

    
    application.connect_activate(|app| {
        let top_display = Rc::new(Label::new(None));
        let bottom_display = Rc::new(Label::new(None));
        let calculator = Rc::new(RefCell::new(Calculator::new(bottom_display, top_display)));
        gui::build_ui(&app, calculator.clone());
    });

    application.run();
}
