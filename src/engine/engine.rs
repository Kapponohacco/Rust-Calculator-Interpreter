use std::cmp::PartialEq;
use std::collections::HashMap;
use crate::engine::CalcError;
use crate::engine::eval_rpn;
use crate::engine::Token;
use crate::engine::parse_to_rpn;
use crate::engine::tokenize;

#[derive(Debug, Clone,PartialEq)]
pub enum Value {
    Number(f64),
    Expr(Vec<Token>),
    Bool(bool),
}

pub struct CalculatorEngine {
    variables: HashMap<String, Value>,
    history: Vec<String>,
}

impl CalculatorEngine {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
            history: Vec::new(),
        }
    }

    pub fn evaluate(&mut self, input: &str) -> Result<Value, CalcError> {
        let tokens = tokenize(input)?;
        let equal_pos = tokens.iter().position(|t| matches!(t, Token::Equal));
        match equal_pos {
            //<Var> = <expr> or <expr> = <expr>, evaluates to Value and gets saved to variables or to a bool
            Some(_) => {
                let result = self.evaluate_equality_and_handle_assignment(&tokens,equal_pos.unwrap())?;
                Ok(result)
            },
            //<expr>, evaluates to Value
            None => {
                let parsed = parse_to_rpn(&tokens)?;
                println!("{:?}", parsed);
                let result = eval_rpn(&parsed, &self.variables)?;
                Ok(result)
            }
        }
    }

    fn evaluate_equality_and_handle_assignment(&mut self, tokens: &Vec<Token>, equality_pos: usize) -> Result<Value, CalcError> {
        let left_expression = &tokens[0..equality_pos].to_vec();
        let right_expression = &tokens[equality_pos+1..].to_vec();
        match left_expression.len() {
            0 => Err(CalcError::EmptyExpression),
            _ if right_expression.is_empty() => Err(CalcError::EmptyExpression),
            1 => {
                match &left_expression[0] {
                    Token::Number(n) => {
                        let parsed = parse_to_rpn(right_expression)?;
                        let result = eval_rpn(&parsed, &self.variables)?;
                        match result {
                            Value::Number(nr) => Ok(Value::Bool(nr == *n)),
                            _ => Ok(Value::Bool(false)),
                        }
                    },
                    Token::Var(name) => {
                        let parsed = parse_to_rpn(right_expression)?;
                        let result = eval_rpn(&parsed, &self.variables)?;
                        self.variables.insert(name.clone(), result.clone());
                        Ok(result)
                    }
                    _ => Err(CalcError::InvalidExpression("Left expression cant consist of a single operator or paren expression.".to_string()))
                }
            }
            _ =>{
                let parsed_left = parse_to_rpn(left_expression)?;
                let parsed_right = parse_to_rpn(right_expression)?;
                let result_left = eval_rpn(&parsed_left, &self.variables)?;
                let result_right = eval_rpn(&parsed_right, &self.variables)?;
                Ok(Value::Bool(result_left ==  result_right))



            },
        }
    }

}