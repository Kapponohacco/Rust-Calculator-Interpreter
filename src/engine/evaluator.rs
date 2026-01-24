use crate::engine::Token;
use crate::engine::CalcError;
use std::collections::HashMap;

pub fn eval_rpn (rpn: &[Token], variables: &HashMap<String, f64>) -> Result<f64, CalcError> {
    let mut stack: Vec<f64> = Vec::new();

    for token in rpn {
        match token {
            Token::Number(n) => stack.push(*n),

            Token::Var(name) => {
                let value = variables
                    .get(name)
                    .ok_or_else(|| CalcError::UnknownVariable(name.clone()))?;
                stack.push(*value);
            },

            Token::Plus | Token::Minus | Token::Star | Token::Slash | Token::Power => {
                let b = stack.pop().ok_or_else(|| {
                    CalcError::InvalidExpression("Missing operand".into())
                })?;
                let a = stack.pop().ok_or_else(|| {
                    CalcError::InvalidExpression("Missing operand".into())
                })?;

                let result = match token {
                    Token::Plus => a + b,
                    Token::Minus => a - b,
                    Token::Star => a * b,
                    Token::Slash => {
                        if b == 0.0 {
                            return Err(CalcError::DivisionByZero);
                        }
                        a / b
                    },
                    Token::Power => a.powf(b),
                    _ => Err(CalcError::InvalidToken("Evaluation in rpn operands".to_string()))?,
                };

                stack.push(result);
            },

            Token::UnaryMinus => {
                let a = stack.pop().ok_or_else(|| {
                    CalcError::InvalidExpression("Missing operand".into())
                })?;
                stack.push(-a);
            },

            _ => {
                return Err(CalcError::InvalidToken("Evaluation in rpn".to_string()))?;
            }
        }
    }

    if stack.len() != 1 {
        return Err(CalcError::InvalidExpression(
            "Too many operands".into(),
        ));
    }

    Ok(stack[0])
}
