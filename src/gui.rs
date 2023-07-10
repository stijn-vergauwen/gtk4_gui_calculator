use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow, Button, Grid, Label, Orientation};
use gui_calculator::calculator_app::actions::handle_action;

use self::buttons::get_buttons_for_calculator;

mod buttons;

pub fn build_ui(application: &Application) {
    let grid = Grid::new();
    grid.set_row_homogeneous(true);
    grid.set_column_homogeneous(true);
    grid.set_orientation(Orientation::Vertical);

    let display_label = Label::new(Some("0"));
    grid.attach(&display_label, 0, 0, 4, 1);

    add_buttons_to_grid(&grid);

    let window = ApplicationWindow::builder()
        .application(application)
        .title("Calculator")
        .default_width(300)
        .default_height(400)
        .child(&grid)
        .build();

    window.present();
}

pub fn add_buttons_to_grid(grid: &Grid) {
    let buttons = get_buttons_for_calculator();

    for (index, button) in buttons.iter().enumerate() {
        let column = (index % 4) as i32;
        let row = f64::floor(index as f64 / 4f64) as i32 + 1;

        let action = button.action.clone();

        let button_object = Button::with_label(&button.label);
        button_object.connect_clicked(move |_| handle_action(action));

        grid.attach(&button_object, column, row, 1, 1);
    }
}
