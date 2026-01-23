use crate::engine::CalcError;
use crate::engine::tokenizer::Tokens;
fn precedence(token: &Tokens) -> i32 {
    match token {
        Tokens::Plus | Tokens::Minus => 1,
        Tokens::Star | Tokens::Slash => 2,
        Tokens::Power => 3,
        _ => 0,
    }
}
fn is_operator(token: &Tokens) -> bool {
    match token { 
        Tokens::Plus | Tokens::Minus | Tokens::Star | Tokens::Slash | Tokens::Power => true,
        _ => false,
    }
}

pub fn parse_to_rpn(tokens: &[Tokens]) -> Result<Vec<Tokens>, CalcError> {
    let mut output: Vec<Tokens> = Vec::new();
    let mut operators: Vec<Tokens> = Vec::new();

    for token in tokens {
        match token {
            Tokens::Number(_) | Tokens::Var(_) => {
                output.push(token.clone());
            }

            op if is_operator(op) => {
                while let Some(top) = operators.last() {
                    if is_operator(top) && precedence(top) >= precedence(op) {
                        output.push(operators.pop().unwrap());
                    } else {
                        break;
                    }
                }
                operators.push(op.clone());
            }

            Tokens::LParen => {
                operators.push(Tokens::LParen);
            }

            Tokens::RParen => {
                while let Some(top) = operators.pop() {
                    if top == Tokens::LParen {
                        break;
                    }
                    output.push(top);
                }
            }

            _ => {
                return Err(CalcError::InvalidToken(
                    "Unexpected token in expression".into()
                ));
            }
        }
    }

    while let Some(op) = operators.pop() {
        if matches!(op, Tokens::LParen | Tokens::RParen) {
            return Err(CalcError::InvalidExpression(
                "Mismatched parentheses".into()
            ));
        }
        output.push(op);
    }

    Ok(output)
}