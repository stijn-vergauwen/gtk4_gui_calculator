use std::{cell::RefCell, rc::Rc};

use self::actions::{enter_digit, CalculatorAction, Operation, select_operator, calculate_result};
use gtk4::Label;

pub mod actions;

pub struct Calculator {
    current_calculation: Calculation,
    prev_calculation: Calculation,

    display: Rc<Label>,
}

impl Calculator {
    pub fn new(display: Rc<Label>) -> Self {
        Calculator {
            current_calculation: Calculation::new(),
            prev_calculation: Calculation::new(),
            display,
        }
    }

    pub fn get_display(&self) -> Rc<Label> {
        self.display.clone()
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
        CalculatorAction::Decimal => println!("display decimal"),
        CalculatorAction::Operator(operator) => select_operator(&mut calculator.borrow_mut(), operator),
        CalculatorAction::Equals => calculate_result(&mut calculator.borrow_mut()),
    }

    refresh_display(&calculator.borrow());
}

fn refresh_display(calculator: &Calculator) {
    calculator.display.set_text(&calculator.current_calculation.to_text());
}
