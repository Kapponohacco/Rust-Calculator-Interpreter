use crate::engine::CalculatorEngine;
use eframe::egui::{self, ScrollArea, TextEdit};

pub struct CalculatorApp {
    engine: CalculatorEngine,
    input: String,
    last_result: Option<String>,
    history: Vec<(String, String)>,
}

impl CalculatorApp {

    fn apply_theme(&self, ctx: &egui::Context) {
        let mut visuals = egui::Visuals::dark();

        visuals.window_fill = egui::Color32::from_rgb(24, 25, 26);
        visuals.panel_fill = egui::Color32::from_rgb(30, 30, 34);

        visuals.widgets.inactive.bg_fill = egui::Color32::from_rgb(58, 60, 68);
        visuals.widgets.hovered.bg_fill  = egui::Color32::from_rgb(80, 82, 92);
        visuals.widgets.active.bg_fill   = egui::Color32::from_rgb(40, 42, 50);

        visuals.widgets.inactive.fg_stroke.color = egui::Color32::from_rgb(220, 220, 220);

        ctx.set_visuals(visuals);
    }

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
                let mut results: Vec<String> = Vec::new();
                for result in &result {
                    results.push(self.engine.pretty_value(result))
                }
                let s = format!("{:?}", results);
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
        self.apply_theme(ctx);
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.columns(2, |cols| {
                // Lewy panel (inputs, keypad, variables)
                cols[0].vertical(|ui| {
                    if let Some(res) = &self.last_result {
                        ui.label(format!("Result: {}", res));
                    } else {
                        ui.label("Result: -");
                    }

                    let avail = ui.available_size();
                    let btn_w = 44.0_f32;
                    let text_w = (avail.x - btn_w).max(80.0);

                    ui.horizontal(|ui| {
                        let response = ui.add_sized(egui::vec2(text_w, 0.0), TextEdit::singleline(&mut self.input));
                        if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                            self.on_submit();
                            response.request_focus();
                        }
                        if ui.add_sized(egui::vec2(btn_w, 10.0), egui::Button::new("=")).clicked() {
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
                                if ui.add_sized(egui::vec2(36.0, 28.0), egui::Button::new(&label)).clicked() {
                                    self.append_char(label.chars().next().unwrap());
                                }
                            }
                        });
                    }
                    ui.horizontal(|ui| {
                        let ops = ["0", ".", ";"];
                        for op in ops.iter() {
                            if ui.add_sized(egui::vec2(36.0, 28.0), egui::Button::new(*op)).clicked() {
                                self.append_char(op.chars().next().unwrap());
                            }
                        }
                        if ui.add_sized(egui::vec2(52.0, 28.0), egui::Button::new("<-")).clicked() {
                            self.backspace();
                        }
                        if ui.add_sized(egui::vec2(52.0, 28.0), egui::Button::new("C")).clicked() {
                            self.clear_input();
                        }
                    });

                    ui.horizontal(|ui| {
                        let ops = ["+", "-", "*", "/", "^"];
                        for op in ops.iter() {
                            if ui.add_sized(egui::vec2(36.0, 28.0), egui::Button::new(*op)).clicked() {
                                self.append_operator(op);
                            }
                        }
                    });

                    ui.horizontal(|ui| {
                        let ops = ["(", ")", "a", "b", "c"];
                        for op in ops.iter() {
                            if ui.add_sized(egui::vec2(36.0, 28.0), egui::Button::new(*op)).clicked() {
                                self.append_operator(op);
                            }
                        }
                    });

                    ui.horizontal(|ui|{
                       let ops = ["+=", "-=", "*=", "/="];
                       for op in ops.iter() {
                           if ui.add_sized(egui::vec2(44.0, 28.0), egui::Button::new(*op)).clicked() {
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

                            if ui.add_sized(egui::vec2(56.0, 20.0), egui::Button::new("Delete")).clicked() {
                                if self.engine.remove_var(name) {
                                    self.push_history(format!("remove {}", name), "OK".to_string());
                                }
                            }
                        });
                    }
                });

                cols[1].vertical(|ui| {
                    let avail_h = ui.available_size().y.max(24.0);

                    ui.horizontal(|ui| {
                        ui.add_sized(egui::vec2(2.0, avail_h), egui::Separator::default().vertical());
                        ui.add_space(6.0);
                        ui.vertical(|ui| {
                            ui.horizontal(|ui| {
                                ui.label("History");
                                if ui.add_sized(egui::vec2(56.0, 20.0), egui::Button::new("Clear")).clicked() {
                                    self.clear_history();
                                }
                            });

                            ScrollArea::vertical().max_height(300.0).show(ui, |ui| {
                                let history_clone = self.history.clone();
                                for (input, result) in history_clone {
                                    let label = format!("{} => {}", input, result);
                                    if ui.button(&label).clicked() {
                                        self.set_input(input);
                                    }
                                }
                            });
                        });
                    });
                });

            });
        });
    }
}