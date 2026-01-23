#[derive(Debug)]
pub enum CalcError {
    NotImplemented,
    InvalidExpression(String),
    DivisionByZero,
    InvalidToken(String),
    UnknownVariable(String),
}