
use crate::engine::CalculatorEngine;
use eframe::egui::{self, ScrollArea};

pub struct CalculatorApp {
    engine: CalculatorEngine,
    input: String,
    last_result: Option<String>,
    history: Vec<(String, String)>,
}

impl CalculatorApp {
    pub fn new() -> Self {
        Self {
            engine: CalculatorEngine::new(),
            input: String::new(),
            last_result: None,
            history: Vec::new(),
        }
    }

    pub fn on_submit(&mut self) {
        match self.engine.evaluate(&self.input) {
            Ok(result) => {
                let s = format!("{:?}", result);
                self.last_result = Some(s.clone());
                self.push_history(self.input.clone(), s);
            }
            Err(err) => {
                let s = format!("Error: {:?}", err);
                self.last_result = Some(s.clone());
            }
        }
    }

    fn push_history(&mut self, input: String, result: String) {
        self.history.insert(0, (input, result));
        if self.history.len() > 100 {
            self.history.truncate(100);
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

    fn clear_history(&mut self) {
        self.history.clear();
    }
}

impl eframe::App for CalculatorApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.columns(3, |cols| {
                cols[0].vertical(|ui| {
                    if let Some(res) = &self.last_result {
                        ui.label(format!("Result: {}", res));
                    } else {
                        ui.label("Result: -");
                    }

                    ui.horizontal(|ui| {
                        let response = ui.text_edit_singleline(&mut self.input);
                        if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                            self.on_submit();
                            response.request_focus();
                        }
                        if ui.button("=").clicked() {
                            self.on_submit();
                            response.request_focus();
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
                        if ui.button("<-").clicked() {
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

                    ui.separator();
                    ui.label("Variables:");
                    let vars = self.engine.list_vars();
                    for (i, name) in vars.iter().enumerate() {
                        ui.horizontal(|ui| {
                            ui.label(format!("{}. {}", i + 1, name));

                            if let Some(val) = self.engine.var_display(name) {
                                ui.label(format!("= {}", val));
                            } else {
                                ui.label("= -");
                            }

                            if ui.button("Delete").clicked() {
                                if self.engine.remove_var(name) {
                                    self.push_history(format!("remove {}", name), "OK".to_string());
                                }
                            }
                        });
                    }

                });

                cols[2].vertical(|ui| {
                    ui.horizontal(|ui| {
                        ui.label("History");
                        if ui.button("Clear").clicked() {
                            self.clear_history();
                        }
                    });

                    ScrollArea::vertical().max_height(300.0).show(ui, |ui| {
                        let history_clone = self.history.clone();
                        for (input, result) in history_clone {
                            let label = format!("{} = {}", input, result);
                            if ui.button(&label).clicked() {
                                self.set_input(input);
                            }
                        }
                    });
                });
            });
        });
    }
}