use std::{cell::RefCell, rc::Rc};

use super::Calculator;

#[derive(Clone, Copy)]
pub enum CalculatorAction {
    Digit(u8),
    Decimal,
    Operator(Operation),
    Equals,
}

#[derive(Clone, Copy)]
pub enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
}

impl ToString for Operation {
    fn to_string(&self) -> String {
        match self {
            Self::Add => String::from("+"),
            Self::Subtract => String::from("-"),
            Self::Multiply => String::from("*"),
            Self::Divide => String::from("/"),
        }
    }
}

pub fn enter_digit(calculator: Rc<RefCell<Calculator>>, digit: u8) {
    let mut calculator = calculator.borrow_mut();
    match calculator.current_calculation.operation {
        Some(_) => {
            calculator.current_calculation.right =
                Some(concat_digits(calculator.current_calculation.right, digit))
        }
        None => {
            calculator.current_calculation.left =
                Some(concat_digits(calculator.current_calculation.left, digit))
        }
    }
}

fn concat_digits(digits: Option<f64>, add: u8) -> f64 {
    let digits = match digits {
        Some(val) => val.to_string(),
        None => String::new(),
    };
    format!("{}{}", digits, add)
        .parse::<f64>()
        .expect("Concatenated digits should always be a valid float")
}
