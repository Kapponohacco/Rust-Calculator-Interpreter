use std::collections::HashMap;
use crate::engine::CalcError;
use crate::engine::tokenize;

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

    pub fn evaluate(&mut self, input: &str) -> Result<f64, CalcError> {
        let tokens = tokenize(input)?;
        println!("{:?}", tokens);
        self.history.push(input.to_string());
        Err(CalcError::NotImplemented)
    }
}