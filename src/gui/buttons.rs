use gui_calculator::calculator_app::actions::{CalculatorAction, Operator};

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
    let new = |label, action| CalculatorButton::new(String::from(label), action);

    [
        new("7", CalculatorAction::Digit(7)),
        new("8", CalculatorAction::Digit(8)),
        new("9", CalculatorAction::Digit(9)),
        new("/", CalculatorAction::Operator(Operator::Divide)),
        new("4", CalculatorAction::Digit(4)),
        new("5", CalculatorAction::Digit(5)),
        new("6", CalculatorAction::Digit(6)),
        new("*", CalculatorAction::Operator(Operator::Multiply)),
        new("1", CalculatorAction::Digit(1)),
        new("2", CalculatorAction::Digit(2)),
        new("3", CalculatorAction::Digit(3)),
        new("-", CalculatorAction::Operator(Operator::Subtract)),
        new("0", CalculatorAction::Digit(0)),
        new(".", CalculatorAction::Decimal),
        new("=", CalculatorAction::Equals),
        new("+", CalculatorAction::Operator(Operator::Add)),
    ]
}
