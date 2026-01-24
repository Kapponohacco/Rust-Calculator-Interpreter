use crate::engine::Token;
use crate::engine::CalcError;
use std::collections::HashMap;
use crate::engine::CalcError::HowDidWeGetHere;
use crate::engine::Value;

fn to_rpn(val: Value) -> Result<Vec<Token>, CalcError> {
    match val {
        Value::Number(n) => Ok(vec![Token::Number(n)]),
        Value::Expr(e) => Ok(e),
        _ => Err(CalcError::HowDidWeGetHere("Bool value in evaluator.".to_string()))
    }
}

pub fn eval_rpn (rpn: &[Token], variables: &HashMap<String, Value>) -> Result<Value, CalcError> {
    let mut stack: Vec<Value> = Vec::new();
    if rpn.is_empty() {
        return Err(CalcError::EmptyExpression);
    }
    for token in rpn {
        match token {
            Token::Number(n) => stack.push(Value::Number(*n)),

            Token::Var(name) => {
                if let Some(val) = variables.get(name) {
                    match val{
                        Value::Number(n) => stack.push(Value::Number(*n)),
                        Value::Expr(e) => {
                            let result = eval_rpn(&e, &variables)?;
                            stack.push(result);
                        }
                        _ => return Err(CalcError::HowDidWeGetHere("A saved variable can not be a bool".to_string()))
                    }
                } else {
                    stack.push(Value::Expr(vec![Token::Var(name.clone())]));
                }
            },

            Token::Plus | Token::Minus | Token::Star | Token::Slash | Token::Power => {
                let b = stack.pop().ok_or_else(|| { CalcError::MissingOperand })?;
                let a = stack.pop().ok_or_else(|| { CalcError::MissingOperand })?;

                let result = match (a, b) {
                    (Value::Number(x), Value::Number(y)) => {
                        let v = match token {
                            Token::Plus => x + y,
                            Token::Minus => x - y,
                            Token::Star => x * y,
                            Token::Slash => {
                                if y == 0.0 {
                                    return Err(CalcError::DivisionByZero);
                                }
                                x / y
                            }
                            Token::Power => x.powf(y),
                            _ => return Err(CalcError::HowDidWeGetHere("Non existant operator token in evaluator.".to_string())),
                        };
                        Value::Number(v)
                    }

                    (left, right) => {
                        let mut expr = Vec::new();
                        let left = to_rpn(left)?;
                        let right = to_rpn(right)?;
                        expr.extend(left);
                        expr.extend(right);
                        expr.push(token.clone());
                        Value::Expr(expr)
                    }
                };

                stack.push(result);
            },

            Token::UnaryMinus => {
                let a = stack.pop().ok_or_else(|| { CalcError::MissingOperand })?;
                match a {
                    Value::Number(number) => stack.push(Value::Number(-number)),
                    Value::Expr(mut expr) => {
                        expr.push(Token::UnaryMinus);
                        stack.push(Value::Expr(expr));
                    },
                    _ => return Err(CalcError::HowDidWeGetHere("Bool after a Unary minus".to_string()))
                }
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

    Ok(stack.pop().unwrap())
}
