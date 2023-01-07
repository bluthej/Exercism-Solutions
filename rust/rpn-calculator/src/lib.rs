#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    let mut stack = Vec::new();

    for input in inputs {
        match input {
            CalculatorInput::Value(i) => stack.push(*i),
            _ => {
                let y = stack.pop();
                let x = stack.pop();
                match (x, y) {
                    (Some(x), Some(y)) => match input {
                        CalculatorInput::Add => stack.push(x + y),
                        CalculatorInput::Subtract => stack.push(x - y),
                        CalculatorInput::Multiply => stack.push(x * y),
                        CalculatorInput::Divide => stack.push(x / y),
                        _ => (),
                    },
                    _ => return None,
                }
            }
        }
    }

    match stack[..] {
        [result] => Some(result),
        _ => None,
    }
}
