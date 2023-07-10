use gtk4::Label;


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

pub fn handle_action(action: CalculatorAction, display_label: &Label) {
    match action {
        CalculatorAction::Digit(digit) => enter_digit(digit, display_label),
        CalculatorAction::Decimal => println!("display decimal"),
        CalculatorAction::Operator(_) => println!("do operator"),
        CalculatorAction::Equals => println!("calculate result"),
    }
}

fn enter_digit(digit: u8, display_label: &Label) {
    let new_input = format!("{}{}", display_label.text(), digit);

    display_label.set_text(&new_input);
}