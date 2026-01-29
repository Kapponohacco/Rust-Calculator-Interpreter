use std::cmp::PartialEq;
use std::collections::{HashMap, HashSet};
use crate::engine::CalcError;
use crate::engine::Token;
use crate::engine::eval_ast;
use crate::engine::tokenize;
use crate::engine::Expr;
use crate::engine::parser::parse_to_ast;

#[derive(Debug, Clone,PartialEq)]
pub enum Value {
    Number(f64),
    Expression(Expr),
    Bool(bool),
}
pub struct CalculatorEngine {
    variables: HashMap<String, Expr>,
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
            //<Var> = <expression> or <expression> == <expression>, evaluates to Value and gets saved to variables or to a bool,
            // Aded +=, -=, *= and /=
            Some(_) => {
                let result = self.evaluate_equality_and_handle_assignment(&tokens,op_pos.unwrap())?;
                Ok(result)
            },
            //<expr>, evaluates to Value
            None => {
                let parsed = parse_to_ast(&tokens)?;
                println!("{:?}", parsed);
                let mut visited = HashSet::new();
                let result = eval_ast(&parsed, &self.variables, &mut visited)?;
                Ok(self.expr_to_value(&result))
            }
        }
    }

    pub fn expr_to_value(&mut self, expr: &Expr) -> Value {
        match expr {
            Expr::Number(n) => Value::Number(*n),
            _ => Value::Expression(expr.clone()),
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
                        let parsed_left = parse_to_ast(left_expression)?;
                        let parsed_right = parse_to_ast(right_expression)?;
                        let mut visited = HashSet::new();
                        let result_left = eval_ast(&parsed_left, &self.variables, &mut visited)?;
                        println!("{:?}",visited);
                        let result_right = eval_ast(&parsed_right, &self.variables, &mut visited)?;
                        Ok(Value::Bool(result_left ==  result_right))
                    },
                    Token::Assign => {
                        if let Token::Var(name) = &left_expression[0]{
                            let parsed = parse_to_ast(right_expression)?;
                            let mut visited = HashSet::new();
                            visited.insert(name.clone());
                            let result = eval_ast(&parsed, &self.variables, &mut visited)?;
                            self.variables.insert(name.clone(), result.clone());
                            Ok(self.expr_to_value(&result))
                        }
                        else {
                            Err(CalcError::InvalidExpression("Cannot assign to a non variable expression".to_string()))
                        }
                    },
                    Token::PlusEqual | Token::MinusEqual | Token::StarEqual | Token::SlashEqual => {
                        if let Token::Var(name) = &left_expression[0] {
                            let parsed_right = parse_to_ast(right_expression)?;
                            let variable_expr = self.variables
                                .get(name)
                                .cloned()
                                .unwrap_or(Expr::Var(name.clone()));

                            // Combine: old + rhs

                            let combined:Expr = match tokens[equality_pos]{
                                Token::PlusEqual => Expr::Add(vec![variable_expr, parsed_right]),
                                Token::MinusEqual => Expr::Sub(Box::new(variable_expr), Box::new(parsed_right)),
                                Token::StarEqual => Expr::Mul(vec![variable_expr, parsed_right]),
                                Token::SlashEqual => Expr::Div(Box::new(variable_expr), Box::new(parsed_right)),
                                _ => Err(CalcError::HowDidWeGetHere("What".to_string()))?
                            };
                            let mut visited = HashSet::new();
                            visited.insert(name.clone());
                            let result = eval_ast(&combined, &self.variables,&mut visited)?;
                            self.variables.insert(name.clone(), result.clone());
                            Ok(self.expr_to_value(&result))
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