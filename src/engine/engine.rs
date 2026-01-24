use std::collections::HashMap;
use crate::engine::CalcError;
use crate::engine::eval_rpn;
use crate::engine::Token;
use crate::engine::parse_to_rpn;
use crate::engine::tokenize;

pub enum Expression {
}

pub struct CalculatorEngine {
    variables: HashMap<String, f64>,
    history: Vec<String>,
}

impl CalculatorEngine {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
            history: Vec::new(),
        }
    }

    /*Plan:
    Find the Equal Token,
     If its on index 1, its a assignment to a variable
        Chceck if there is a variable on the left side and an expression on the right side
     If its somewheer else it a boolean equality,
        Check if there are expressions on both sides of the Token
     Else
        Its just a normal Calculation
    */
    pub fn evaluate(&mut self, input: &str) -> Result<f64, CalcError> {
        let tokens = tokenize(input)?;
        //assignment: <Var> = <expression>
        if let Some(Token::Equal) = tokens.get(1) {
            let result = self.handle_assignment(&tokens)?;
            println!("{}", result);
            println!("{:?}", self.variables);
            Ok(result)
        }
        // For now normal operations, will change it soon
        else{
            let parsed = parse_to_rpn(&tokens)?;
            println!("{:?}", parsed);

            let result = eval_rpn(&parsed, &self.variables)?;

            self.history.push(input.to_string());
            Err(CalcError::NotImplemented)
        }

    }

    fn handle_assignment(&mut self, tokens: &Vec<Token>) -> Result<f64, CalcError> {
        let variable_name = &tokens[0];
        let expression = &tokens.get(2..).unwrap().to_vec();
        match variable_name {
            _ if expression.is_empty() => Err(CalcError::EmptyAssignmentExpression),
            Token::Var(name) => {
                let parsed = parse_to_rpn(expression)?;
                let result = eval_rpn(&parsed,&self.variables)?;
                self.variables.insert(name.to_string(), result.clone());
                Ok(result)
            }
            _ => Err(CalcError::NotImplemented),
        }
    }

}