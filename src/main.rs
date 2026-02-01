mod engine;
mod gui;

use gui::CalculatorApp;

fn main() {
    let mut app = CalculatorApp::new();
    CalculatorApp::new().run();
}




#[cfg(test)]
mod test_basic{
    use super::*;
    #[test]
    fn test_add(){ let mut app = CalculatorApp::new();
    }
    #[test]
    fn test_sub(){
        let mut app = CalculatorApp::new();
    }
    #[test]
    fn test_mul(){
        let mut app = CalculatorApp::new();
    }
    #[test]
    fn test_div(){
        let mut app = CalculatorApp::new();
    }
    #[test]
    fn test_div_by_zero(){
        let mut app = CalculatorApp::new();
    }
    #[test]
    fn test_invalid_input() {
        let mut app = CalculatorApp::new();
    }
}

#[cfg(test)]
mod test_varables{
    use super::*;
    #[test]
    fn test_declare_variable_using_declared_variable(){
        let mut app = CalculatorApp::new();
    }
    #[test]
    fn test_declare_variable_lazy_evaluation(){
        let mut app = CalculatorApp::new();
    }
}