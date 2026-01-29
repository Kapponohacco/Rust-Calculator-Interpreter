#[derive(Debug)]
pub enum CalcError {
    NotImplemented, // not implemented
    HowDidWeGetHere(String), // for situations that make no sense, mostly an inside joke from another project
    InvalidExpression(String), // for invalid expressions like "!" in the base input during tokenization
    DivisionByZero, // obvious
    InvalidToken(String), // Getting this error should not be possible and that's why it exists
    //UnknownVariable(String), //(deprecated) for non assignment expressions, which contain an unknown variable
    MissingOperand, // When evaluating a rpn expression and cant find a operand for a operator
    EmptyExpression,
    TooManyOperands,
}