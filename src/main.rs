mod engine;
mod gui;

use gui::CalculatorApp;

fn main() {
    //we will need to add a GUI event loop here
    let mut app = CalculatorApp::new();

    //just for testing
    app.set_input("2 + 3".to_string());
    app.on_submit();
}