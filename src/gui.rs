use std::cell::RefCell;
use std::rc::Rc;

use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow, Button, Grid, Orientation};
use gui_calculator::calculator_app::{handle_input, Calculator};

use self::buttons::get_buttons_for_calculator;

mod buttons;

pub fn build_ui(application: &Application, calculator: Rc<RefCell<Calculator>>) {
    let grid = Grid::new();
    grid.set_row_homogeneous(true);
    grid.set_column_homogeneous(true);
    grid.set_orientation(Orientation::Vertical);

    grid.attach(calculator.borrow().get_display().as_ref(), 0, 0, 4, 1);

    add_buttons_to_grid(&grid, calculator);

    let window = ApplicationWindow::builder()
        .application(application)
        .title("Calculator")
        .default_width(300)
        .default_height(400)
        .child(&grid)
        .build();

    window.present();
}

pub fn add_buttons_to_grid(grid: &Grid, calculator: Rc<RefCell<Calculator>>) {
    let buttons = get_buttons_for_calculator();

    for (index, button) in buttons.iter().enumerate() {
        let column = (index % 4) as i32;
        let row = f64::floor(index as f64 / 4f64) as i32 + 1;

        let action = button.action.clone();

        // Idk exaxctly how yet but using Rc<> allowed the reference to be used in 'static closure. oh because owned data doesn't need lifetime annotation?
        let cloned_calculator = calculator.clone();

        let button_object = Button::with_label(&button.label);
        button_object.connect_clicked(move |_| handle_input(cloned_calculator.clone(), action));

        grid.attach(&button_object, column, row, 1, 1);
    }
}
