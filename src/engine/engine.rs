use std::cmp::PartialEq;
use std::collections::{HashMap, HashSet};
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
        let op_pos = tokens.iter().position(|t| {
            matches!(t,Token::Assign| Token::Equal | Token::PlusEqual| Token::MinusEqual| Token::StarEqual| Token::SlashEqual)
        });

        match op_pos {
            //<Var> = <expr> or <expr> == <expr>, evaluates to Value and gets saved to variables or to a bool,
            // Aded +=, -=, *= and /=
            Some(_) => {
                let result = self.evaluate_equality_and_handle_assignment(&tokens,op_pos.unwrap())?;
                Ok(result)
            },
            //<expr>, evaluates to Value
            None => {
                let parsed = parse_to_rpn(&tokens)?;
                println!("{:?}", parsed);
                let mut visited = HashSet::new();
                let result = eval_rpn(&parsed, &self.variables, &mut visited)?;
                Ok(result)
            }
        }
    }

    fn evaluate_equality_and_handle_assignment(&mut self, tokens: &Vec<Token>, equality_pos: usize) -> Result<Value, CalcError> {
        let left_expression = &tokens[0..equality_pos].to_vec();
        let right_expression = &tokens[equality_pos+1..].to_vec();
        match (left_expression.len(),right_expression.len()){
            (0,_) => Err(CalcError::EmptyExpression),
            (_,0) => Err(CalcError::EmptyExpression),
            _ =>{
                match tokens[equality_pos]{
                    Token::Equal => {
                        let parsed_left = parse_to_rpn(left_expression)?;
                        let parsed_right = parse_to_rpn(right_expression)?;
                        let mut visited = HashSet::new();
                        let result_left = eval_rpn(&parsed_left, &self.variables, &mut visited)?;
                        println!("{:?}",visited);
                        let result_right = eval_rpn(&parsed_right, &self.variables, &mut visited)?;
                        Ok(Value::Bool(result_left ==  result_right))
                    },
                    Token::Assign => {
                        if let Token::Var(name) = &left_expression[0]{
                            let parsed = parse_to_rpn(right_expression)?;
                            let mut visited = HashSet::new();
                            visited.insert(name.clone());
                            let result = eval_rpn(&parsed, &self.variables, &mut visited)?;
                            self.variables.insert(name.clone(), result.clone());
                            Ok(result)
                        }
                        else {
                            Err(CalcError::InvalidExpression("Cannot assign to a non variable expression".to_string()))
                        }
                    },
                    Token::PlusEqual | Token::MinusEqual | Token::StarEqual | Token::SlashEqual => {
                        if let Token::Var(name) = &left_expression[0] {
                            let parsed_right = parse_to_rpn(right_expression)?;
                            let variable_val = self.variables
                                .get(name)
                                .cloned()
                                .unwrap_or(Value::Expr(vec![Token::Var(name.clone())]));

                            let variable_tokens = match variable_val{
                                Value::Expr(tokens) => tokens,
                                Value::Number(num) => vec![Token::Number(num)],
                                _ => {Err(CalcError::HowDidWeGetHere("Variable has bool as value".to_string()))?}
                            };
                            // Combine: old + rhs
                            let mut combined = variable_tokens;
                            combined.extend(parsed_right);
                            match tokens[equality_pos]{
                                Token::PlusEqual => combined.push(Token::Plus),
                                Token::MinusEqual => combined.push(Token::Minus),
                                Token::StarEqual => combined.push(Token::Star),
                                Token::SlashEqual => combined.push(Token::Slash),
                                _ => Err(CalcError::HowDidWeGetHere("What".to_string()))?
                            }
                            let mut visited = HashSet::new();
                            visited.insert(name.clone());
                            let result = eval_rpn(&combined, &self.variables,&mut visited)?;
                            self.variables.insert(name.clone(), result.clone());
                            Ok(result)
                        }
                        else{
                            Err(CalcError::InvalidExpression("Cannot assign to a non variable expression".to_string()))
                        }

                    }
                    _ => Err(CalcError::HowDidWeGetHere("Another token in place of equality/assignment token".to_string()))
                }
            }
        }
    }
}