#[derive(Clone, Copy)]
pub enum CalculatorAction {
    Digit(u8),
    Decimal,
    Operator(Operator),
    Equals,
}

#[derive(Clone, Copy)]
pub enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
}

pub fn handle_action(action: CalculatorAction) {
    match action {
        CalculatorAction::Digit(_) => println!("display digit"),
        CalculatorAction::Decimal => println!("display decimal"),
        CalculatorAction::Operator(_) => println!("do operator"),
        CalculatorAction::Equals => println!("calculate result"),
    }
}