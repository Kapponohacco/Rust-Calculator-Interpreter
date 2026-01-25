use crate::engine::CalculatorEngine;
use eframe::egui;

pub struct CalculatorApp {
    engine: CalculatorEngine,
    input: String,
    last_result: Option<String>,
}

impl CalculatorApp {
    pub fn new() -> Self {
        Self {
            engine: CalculatorEngine::new(),
            input: String::new(),
            last_result: None,
        }
    }

    pub fn on_submit(&mut self) {
        match self.engine.evaluate(&self.input) {
            Ok(result) => {
                let s = format!("{:?}", result);
                self.last_result = Some(s);
            }
            Err(err) => {
                let s = format!("Error: {:?}", err);
                self.last_result = Some(s);
            }
        }
    }

    pub fn set_input(&mut self, input: String) {
        self.input = input;
    }

    pub fn run(self) {
        let options = eframe::NativeOptions::default();
        eframe::run_native(
            "Calculator",
            options,
            Box::new(move |_cc| Ok(Box::new(self) as Box<dyn eframe::App>)),
        );
    }

    fn append_char(&mut self, ch: char) {
        self.input.push(ch);
    }

    fn append_operator(&mut self, op: &str) {
        if self.input.is_empty() {
            self.input.push_str(op);
            self.input.push(' ');
            return;
        }

        while self.input.ends_with(' ') {
            self.input.pop();
        }

        self.input.push(' ');
        self.input.push_str(op);
        self.input.push(' ');
    }

    fn backspace(&mut self) {
        if self.input.is_empty() {
            return;
        }
        if self.input.ends_with(' ') {
            while self.input.ends_with(' ') {
                self.input.pop();
            }
            while !self.input.is_empty() && !self.input.ends_with(' ') {
                self.input.pop();
            }
            if self.input.ends_with(' ') {
                self.input.pop();
            }
        } else {
            self.input.pop();
        }
    }

    fn clear_input(&mut self) {
        self.input.clear();
    }
}

impl eframe::App for CalculatorApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    if let Some(res) = &self.last_result {
                        ui.label(format!("Result: {}", res));
                    } else {
                        ui.label("Result: -");
                    }

                    ui.horizontal(|ui| {
                        ui.text_edit_singleline(&mut self.input);
                        if ui.button("=").clicked() {
                            self.on_submit();
                        }
                    });

                    ui.separator();

                    ui.label("Keypad:");
                    for row in 0..3 {
                        ui.horizontal(|ui| {
                            for col in 1..=3 {
                                let digit = (row * 3 + col) as u8;
                                let label = format!("{}", digit);
                                if ui.button(&label).clicked() {
                                    self.append_char(label.chars().next().unwrap());
                                }
                            }
                        });
                    }
                    ui.horizontal(|ui| {
                        if ui.button("0").clicked() {
                            self.append_char('0');
                        }
                        if ui.button(".").clicked() {
                            self.append_char('.');
                        }
                        if ui.button("←").clicked() {
                            self.backspace();
                        }
                        if ui.button("C").clicked() {
                            self.clear_input();
                        }
                    });

                    ui.horizontal(|ui| {
                        let ops = ["+", "-", "*", "/", "^"];
                        for op in ops.iter() {
                            if ui.button(*op).clicked() {
                                self.append_operator(op);
                            }
                        }
                    });
                });
            });
        });
    }
}