use crate::engine::CalculatorEngine;
use crate::engine::CalcError;

pub struct CalculatorApp {
    engine: CalculatorEngine,
    input: String,
}

impl CalculatorApp {
    pub fn new() -> Self {
        Self {
            engine: CalculatorEngine::new(),
            input: String::new(),
        }
    }

    pub fn on_submit(&mut self) {
        match self.engine.evaluate(&self.input) {
            Ok(result) => println!("{:?}", result),
            Err(err) => println!("{:?}", err),
        }
    }
    pub fn set_input(&mut self, input: String) {
        self.input = input;
    }
}