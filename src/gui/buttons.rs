use gui_calculator::calculator_app::actions::{CalculatorAction, Operation};

pub struct CalculatorButton {
    pub label: String,
    pub action: CalculatorAction,
}

impl CalculatorButton {
    pub fn new(label: String, action: CalculatorAction) -> Self {
        CalculatorButton { label, action }
    }
}

pub fn get_buttons_for_calculator() -> [CalculatorButton; 16] {
    use CalculatorAction::*;
    let new = |label, action| CalculatorButton::new(String::from(label), action);

    [
        new("7", Digit(7)),
        new("8", Digit(8)),
        new("9", Digit(9)),
        new("/", Operator(Operation::Divide)),
        new("4", Digit(4)),
        new("5", Digit(5)),
        new("6", Digit(6)),
        new("*", Operator(Operation::Multiply)),
        new("1", Digit(1)),
        new("2", Digit(2)),
        new("3", Digit(3)),
        new("-", Operator(Operation::Subtract)),
        new("0", Digit(0)),
        new(".", Decimal),
        new("=", Equals),
        new("+", Operator(Operation::Add)),
    ]
}
