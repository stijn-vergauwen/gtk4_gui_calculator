use super::{Calculation, Calculator};

#[derive(Clone, Copy)]
pub enum CalculatorAction {
    Digit(u8),
    Decimal,
    Operator(Operation),
    Equals,
}

#[derive(Clone, Copy, PartialEq)]
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

pub fn enter_digit(calculator: &mut Calculator, digit: u8) {
    match calculator.current_calculation.operation {
        Some(_) => {
            calculator.current_calculation.right = Some(concat_digits(
                calculator.current_calculation.right,
                digit,
                calculator.add_decimal_on_next_digit,
            ))
        }
        None => {
            calculator.current_calculation.left = Some(concat_digits(
                calculator.current_calculation.left,
                digit,
                calculator.add_decimal_on_next_digit,
            ))
        }
    }

    calculator.add_decimal_on_next_digit = false
}

pub fn enter_decimal(calculator: &mut Calculator) {
    let calculation = &calculator.current_calculation;
    if (calculation.operation.is_none() && !is_decimal(calculation.left.unwrap_or(0.0)))
        || calculation.operation.is_some() && !is_decimal(calculation.right.unwrap_or(0.0))
    {
        calculator.add_decimal_on_next_digit = true;
    }
}

pub fn select_operator(calculator: &mut Calculator, operator: Operation) {
    if calculator.current_calculation.operation.is_none() {
        calculator.current_calculation.operation = Some(operator);
    }
}

pub fn calculate_result(calculator: &mut Calculator) {
    if !calculator.current_calculation.has_empty_fields() {
        let result = make_calculation(
            calculator
                .current_calculation
                .left
                .expect("Fields should never be empty"),
            calculator
                .current_calculation
                .right
                .expect("Fields should never be empty"),
            calculator
                .current_calculation
                .operation
                .expect("Fields should never be empty"),
        );

        calculator.prev_calculation = calculator.current_calculation.clone();

        calculator.current_calculation = Calculation {
            left: Some(result),
            right: None,
            operation: None,
        }
    }
}

fn concat_digits(digits: Option<f64>, add: u8, with_decimal: bool) -> f64 {
    let digits = match digits {
        Some(val) => val.to_string(),
        None => String::new(),
    };

    let formatted = if with_decimal {
        format!("{}.{}", digits, add)
    } else {
        format!("{}{}", digits, add)
    };

    formatted
        .parse::<f64>()
        .expect("Concatenated digits should always be a valid float")
}

fn make_calculation(left: f64, right: f64, operation: Operation) -> f64 {
    match operation {
        Operation::Add => left + right,
        Operation::Subtract => left - right,
        Operation::Multiply => left * right,
        Operation::Divide => left / right,
    }
}

fn is_decimal(number: f64) -> bool {
    let as_int = number as u32;
    number - as_int as f64 > 0.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_identify_decimal_values() {
        assert_eq!(is_decimal(4.0), false);
        assert_eq!(is_decimal(20.0), false);
        assert_eq!(is_decimal(4.1), true);
        assert_eq!(is_decimal(0.765), true);
    }
}
