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

    pub fn remove_var(&mut self, name: &str) -> bool {
        self.variables.remove(name).is_some()
    }

    pub fn list_vars(&self) -> Vec<String> {
        let mut keys: Vec<String> = self.variables.keys().cloned().collect();
        keys.sort();
        keys
    }

    pub fn var_display(&self, name: &str) -> Option<String> {
        if let Some(expr) = self.variables.get(name) {
            let mut visited = HashSet::new();
            match eval_ast(expr, &self.variables, &mut visited) {
                Ok(Expr::Number(n)) => Some(format!("{}", n)),
                Ok(e) => Some(format!("{:?}", e)),
                Err(_) => Some(format!("{:?}", expr)), // gdy ewaluacja się nie powiedzie, pokaż zapis wyrażenia
            }
        } else {
            None
        }
    }


    
    pub fn evaluate(&mut self, input: &str) -> Result<Vec<Value>, CalcError> {
        let tokens = tokenize(input)?;
        println!("tokens: {:?}", tokens);
        let eof_split = tokens.split(|t| matches!(t,Token::EndOfFile));
        let mut op_pos: Option<usize> = None;
        let mut results: Vec<Value> = Vec::new();
        for split in eof_split {
            if split.is_empty(){
                break;
            }
            println!("split: {:?}", split); 
            op_pos = split.iter().position(|t| { matches!(t,Token::Assign| Token::Equal | Token::PlusEqual| Token::MinusEqual| Token::StarEqual| Token::SlashEqual) });
            println!("op_pos: {:?}", op_pos);
            match op_pos {
                //<Var> = <expression> or <expression> == <expression>, evaluates to Value and gets saved to variables or to a bool,
                // Aded +=, -=, *= and /=
                Some(_) => {
                    let result = self.evaluate_equality_and_handle_assignment(&split, op_pos.unwrap())?;
                    results.push(result)
                },
                //<expr>, evaluates to Value
                None => {
                    let parsed = parse_to_ast(&split.to_vec())?;
                    println!("{:?}", parsed);
                    let mut visited = HashSet::new();
                    let result = eval_ast(&parsed, &self.variables, &mut visited)?;
                    results.push(self.expr_to_value(&result))
                }
            }
        }
        Ok(results)
    }


    pub fn expr_to_value(&mut self, expr: &Expr) -> Value {
        match expr {
            Expr::Number(n) => Value::Number(*n),
            _ => Value::Expression(expr.clone()),
        }
    }

    fn evaluate_equality_and_handle_assignment(&mut self, tokens: &[Token], equality_pos: usize) -> Result<Value, CalcError> {
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

    pub fn pretty_value(&mut self,value: &Value) -> String {
        match value {
            Value::Number(n) => {
                if n.fract() == 0.0 {
                    format!("{}", *n as i64)
                } else {
                    format!("{}", n)
                }
            }
            Value::Expression(expr) => self.pretty_expr(expr,0),
            Value::Bool(b) => b.to_string(),
        }
    }
    fn pretty_expr(&mut self, expr: &Expr, parent_prec: u8) -> String {
        fn precedence(expr: &Expr) -> u8 {
            match expr {
                Expr::Add(_) => 1,
                Expr::Sub(_, _) => 1,
                Expr::Mul(_) => 2,
                Expr::Div(_, _) => 2,
                Expr::Pow(_, _) => 3,
                Expr::Neg(_) => 4,
                Expr::Number(_) | Expr::Var(_) => 5,
            }
        }
        let my_prec = precedence(expr);

        let s = match expr {
            Expr::Number(n) => {
                if n.fract() == 0.0 {
                    format!("{}", *n as i64)
                } else {
                    format!("{}", n)
                }
            }

            Expr::Var(name) => name.clone(),

            Expr::Add(terms) => {
                terms
                    .iter()
                    .map(|t| self.pretty_expr(t, my_prec))
                    .collect::<Vec<_>>()
                    .join(" + ")
            }

            Expr::Mul(terms) => {
                terms
                    .iter()
                    .map(|t| {
                        let part = self.pretty_expr(t, my_prec);
                        if precedence(t) < my_prec {
                            format!("({})", part)
                        } else {
                            part
                        }
                    })
                    .collect::<Vec<_>>()
                    .join(" * ")
            }

            Expr::Sub(a, b) => {
                let left = self.pretty_expr(a, my_prec);
                let right = self.pretty_expr(b, my_prec + 1);
                format!("{} - {}", left, right)
            }

            Expr::Div(a, b) => {
                let left = self.pretty_expr(a, my_prec);
                let right = self.pretty_expr(b, my_prec + 1);
                format!("{} / {}", left, right)
            }

            Expr::Pow(a, b) => {
                let left = self.pretty_expr(a, my_prec);
                let right = self.pretty_expr(b, my_prec);
                format!("{}^{}", left, right)
            }

            Expr::Neg(e) => {
                let inner = self.pretty_expr(e, my_prec);
                if precedence(e) < my_prec {
                    format!("-({})", inner)
                } else {
                    format!("-{}", inner)
                }
            }
        };

        if my_prec < parent_prec {
            format!("({})", s)
        } else {
            s
        }
    }
}