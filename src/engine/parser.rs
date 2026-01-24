use crate::engine::CalcError;
use crate::engine::Token;

fn precedence(token: &Token) -> i32 {
    match token {
        Token::Plus | Token::Minus => 1,
        Token::Star | Token::Slash => 2,
        Token::UnaryMinus => 3,
        Token::Power => 4,
        _ => 0,
    }
}

fn is_operator(token: &Token) -> bool {
    match token { 
        Token::Plus | Token::Minus | Token::Star | Token::Slash | Token::Power => true,
        _ => false,
    }
}

fn is_unary(prev: Option<&Token>, current: &Token) -> bool {
    match current {
        Token::Minus => {
            match prev {
                None => true,
                Some(Token::LParen)
                | Some(Token::Plus)
                | Some(Token::Minus)
                | Some(Token::Star)
                | Some(Token::Slash) => true,
                _ => false,
            }
        }
        _ => false,
    }
}

pub fn parse_to_rpn(tokens: &Vec<Token>) -> Result<Vec<Token>, CalcError> {
    let mut output: Vec<Token> = Vec::new();
    let mut operators: Vec<Token> = Vec::new();
    let mut prev: Option<&Token> = None;

    for token in tokens {
        match token {
            Token::Number(_) | Token::Var(_) => {
                output.push(token.clone());
            },
            op if is_unary(prev, op) => {
                operators.push(Token::UnaryMinus);
            },
            op if is_operator(op) => {
                while let Some(top) = operators.last() {
                    if is_operator(top) && precedence(top) >= precedence(op) {
                        output.push(operators.pop().unwrap());
                    } else {
                        break;
                    }
                }
                operators.push(op.clone());
            },

            Token::LParen => {
                operators.push(Token::LParen);
            },

            Token::RParen => {
                while let Some(top) = operators.pop() {
                    if top == Token::LParen {
                        break;
                    }
                    output.push(top);
                }
            },

            _ => {
                return Err(CalcError::InvalidToken(
                    "Unexpected token in expression".into()
                ));
            }
        }
        prev = Some(token);
    }

    while let Some(op) = operators.pop() {
        if matches!(op, Token::LParen | Token::RParen) {
            return Err(CalcError::InvalidExpression(
                "Mismatched parentheses".into()
            ));
        }
        output.push(op);
    }

    Ok(output)
}