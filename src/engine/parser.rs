use crate::engine::CalcError;
use crate::engine::Token;

#[derive(Clone, Debug, PartialEq)]
pub enum Expr{
    Number(f64),
    Var(String),
    Add(Vec<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Vec<Expr>),
    Div(Box<Expr>, Box<Expr>),
    Pow(Box<Expr>, Box<Expr>),
    Neg(Box<Expr>)
}

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
        Token::Plus | Token::Minus | Token::Star | Token::Slash | Token::Power  => true,
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

fn apply_op(op: Token, stack: &mut Vec<Expr>) -> Result<(), CalcError> {
    match op {
        Token::Plus => {
            let b = stack.pop().ok_or(CalcError::MissingOperand)?;
            let a = stack.pop().ok_or(CalcError::MissingOperand)?;
            stack.push(Expr::Add(vec![a, b]));
        }

        Token::Minus => {
            let b = stack.pop().ok_or(CalcError::MissingOperand)?;
            let a = stack.pop().ok_or(CalcError::MissingOperand)?;
            stack.push(Expr::Sub(Box::new(a), Box::new(b)));
        }

        Token::Star => {
            let b = stack.pop().ok_or(CalcError::MissingOperand)?;
            let a = stack.pop().ok_or(CalcError::MissingOperand)?;
            stack.push(Expr::Mul(vec![a, b]));
        }

        Token::Slash => {
            let b = stack.pop().ok_or(CalcError::MissingOperand)?;
            let a = stack.pop().ok_or(CalcError::MissingOperand)?;
            stack.push(Expr::Div(Box::new(a), Box::new(b)));
        }

        Token::Power =>{
            let b = stack.pop().ok_or(CalcError::MissingOperand)?;
            let a = stack.pop().ok_or(CalcError::MissingOperand)?;
            stack.push(Expr::Pow(Box::new(a), Box::new(b)));
        }

        Token::UnaryMinus => {
            let a = stack.pop().ok_or(CalcError::MissingOperand)?;
            stack.push(Expr::Neg(Box::new(a)));
        }

        _ => return Err(CalcError::InvalidToken("apply_op".into())),
    }

    Ok(())
}

pub fn parse_to_ast(tokens: &Vec<Token>) -> Result<Expr, CalcError> {
    let mut expr_stack: Vec<Expr> = Vec::new();
    let mut ops: Vec<Token> = Vec::new();
    let mut prev: Option<&Token> = None;

    for token in tokens {
        match token {
            Token::Number(n) => expr_stack.push(Expr::Number(*n)),
            Token::Var(v)    => expr_stack.push(Expr::Var(v.clone())),
            op if is_unary(prev, op) => {
                ops.push(Token::UnaryMinus);
            },
            op if is_operator(op) => {
                while let Some(top) = ops.last() {
                    if precedence(top) >= precedence(op) {
                        let op = ops.pop().unwrap();
                        apply_op(op, &mut expr_stack)?;
                    } else {
                        break;
                    }
                }
                ops.push(op.clone());
            },

            Token::LParen => {
                ops.push(Token::LParen);
            },

            Token::RParen => {
                while let Some(op) = ops.pop() {
                    if op == Token::LParen { break; }
                    apply_op(op, &mut expr_stack)?;
                }
            },

            a => {
                return Err(CalcError::InvalidToken(
                    format!("Unexpected token {:?} in expression",a)
                ));
            }
        }
        prev = Some(token);
    }

    while let Some(op) = ops.pop() {
        if matches!(op, Token::LParen | Token::RParen) {
            return Err(CalcError::InvalidExpression("Mismatched parentheses".into()));
        }
        apply_op(op, &mut expr_stack)?;
    }

    if expr_stack.len() > 1 {
        return Err(CalcError::InvalidExpression("Can't create AST from this expression, too many operands".into()));
    }
    else if expr_stack.len() == 0 {
        return Err(CalcError::EmptyExpression);
    }

    Ok(expr_stack.pop().unwrap())
}