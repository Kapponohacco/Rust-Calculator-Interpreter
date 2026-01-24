#[derive(Debug)]
pub enum CalcError {
    NotImplemented, // not implemented
    HowDidWeGetHere, // for situations that make no sense, mostly an inside joke from another project
    InvalidExpression(String), // for invalid expressions like "!" in the base input during tokenization
    DivisionByZero, // obvious
    EmptyAssignmentExpression, // for things like " a =  "
    InvalidToken(String), // Getting this error should not be possible and that's why it exists 
    UnknownVariable(String), // for non assignment expressions, which contain an unknown variable
}