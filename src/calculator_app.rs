use std::{cell::RefCell, rc::Rc};

use self::actions::{enter_digit, CalculatorAction, Operation};
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
}

pub fn handle_input(calculator: Rc<RefCell<Calculator>>, action: CalculatorAction) {
    match action {
        CalculatorAction::Digit(digit) => println!("Enter digit"),
        CalculatorAction::Decimal => println!("display decimal"),
        CalculatorAction::Operator(_) => println!("do operator"),
        CalculatorAction::Equals => println!("calculate result"),
    }

    // refresh_display(calculator)
}

// fn refresh_display(calculator: &Calculator) {
//     todo!();
// }
