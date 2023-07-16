use std::{cell::RefCell, rc::Rc};

use self::actions::{
    calculate_result, enter_decimal, enter_digit, select_operator, CalculatorAction, Operation,
};
use gtk4::Label;

pub mod actions;

pub struct Calculator {
    current_calculation: Calculation,
    prev_calculation: Calculation,
    add_decimal_on_next_digit: bool,

    current_display: Rc<Label>,
    prev_display: Rc<Label>,
}

impl Calculator {
    pub fn new(current_display: Rc<Label>, prev_display: Rc<Label>) -> Self {
        Calculator {
            current_calculation: Calculation::new(),
            prev_calculation: Calculation::new(),
            add_decimal_on_next_digit: false,
            current_display,
            prev_display,
        }
    }

    pub fn get_current_display(&self) -> Rc<Label> {
        self.current_display.clone()
    }

    pub fn get_prev_display(&self) -> Rc<Label> {
        self.prev_display.clone()
    }
}

#[derive(Clone, Copy)]
struct Calculation {
    left: Option<f64>,
    right: Option<f64>,
    operation: Option<Operation>,
}

impl Calculation {
    fn new() -> Self {
        Calculation {
            left: None,
            right: None,
            operation: None,
        }
    }

    fn to_text(&self) -> String {
        let left = match self.left {
            Some(num) => num.to_string(),
            None => String::new(),
        };
        let right = match self.right {
            Some(num) => num.to_string(),
            None => String::new(),
        };
        let operation = match self.operation {
            Some(v) => v.to_string(),
            None => String::new(),
        };

        format!("{left} {operation} {right}")
    }

    fn has_empty_fields(&self) -> bool {
        self.left == None || self.right == None || self.operation == None
    }
}

pub fn handle_input(calculator: Rc<RefCell<Calculator>>, action: CalculatorAction) {
    match action {
        CalculatorAction::Digit(digit) => enter_digit(&mut calculator.borrow_mut(), digit),
        CalculatorAction::Decimal => enter_decimal(&mut calculator.borrow_mut()),
        CalculatorAction::Operator(operator) => {
            select_operator(&mut calculator.borrow_mut(), operator)
        }
        CalculatorAction::Equals => calculate_result(&mut calculator.borrow_mut()),
    }

    refresh_display(&calculator.borrow());
}

fn refresh_display(calculator: &Calculator) {
    calculator
        .current_display
        .set_text(&calculator.current_calculation.to_text());

    calculator
        .prev_display
        .set_text(&calculator.prev_calculation.to_text());
}
