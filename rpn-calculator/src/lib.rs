#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    let mut calculation_stack = vec![];
    for inp in inputs {
        match inp {
            CalculatorInput::Value(i) => calculation_stack.push(*i),
            _ => {
                if calculation_stack.len() < 2 {
                    return None
                }

                let no1 = calculation_stack.pop().unwrap();
                let no2 = calculation_stack.pop().unwrap();

                match inp {
                    CalculatorInput::Add => calculation_stack.push(no1 + no2),
                    CalculatorInput::Subtract => calculation_stack.push(no2 - no1),
                    CalculatorInput::Multiply => calculation_stack.push(no2 * no1),
                    CalculatorInput::Divide => calculation_stack.push(no2 / no1),
                    _ => return None
                }
                
            }
        }
    }
    if calculation_stack.len() == 1 {
        return Some(calculation_stack[0]);
    } else {
        None
    }
}
