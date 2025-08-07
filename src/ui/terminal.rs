use eframe::egui;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct Terminal {
    logs: Arc<Mutex<Vec<String>>>,
}

impl Terminal {
    pub fn new() -> Self {
        Self {
            logs: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn add_log(&self, log: String) {
        let mut logs = self.logs.lock().unwrap();
        logs.push(log);
        if logs.len() > 1000 {
            let mut new_logs = Vec::with_capacity(1000);
            let start_index = logs.len() - 1000;
            for i in start_index..logs.len() {
                new_logs.push(logs[i].clone());
            }
            *logs = new_logs;
        }
    }

    pub fn get_logs(&self) -> Vec<String> {
        self.logs.lock().unwrap().clone()
    }

    pub fn render(&self, ui: &mut egui::Ui) {
        egui::Frame::group(ui.style())
            .fill(egui::Color32::from_rgb(25, 25, 25))
            .stroke(egui::Stroke::new(1.0, egui::Color32::from_rgb(60, 60, 60)))
            .corner_radius(8.0)
            .inner_margin(egui::Margin::symmetric(10i8, 10i8))
            .show(ui, |ui| {
                ui.set_min_height(150.0);
                
                ui.horizontal(|ui| {
                    ui.label(
                        egui::RichText::new("Terminal Output")
                            .size(16.0)
                            .strong()
                            .color(egui::Color32::from_rgb(200, 200, 200)),
                    );
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.label(
                            egui::RichText::new("READ ONLY")
                                .size(10.0)
                                .color(egui::Color32::from_rgb(100, 100, 100)),
                        );
                    });
                });
                
                ui.add_space(8.0);
                
                egui::ScrollArea::vertical()
                    .auto_shrink([false; 2])
                    .stick_to_bottom(true)
                    .show(ui, |ui| {
                        ui.set_min_width(ui.available_width() - 10.0);
                        
                        let logs = self.get_logs();
                        for log in &logs {
                            ui.horizontal(|ui| {
                                ui.label(
                                    egui::RichText::new(">")
                                        .size(12.0)
                                        .color(egui::Color32::from_rgb(100, 100, 100)),
                                );
                                
                                ui.add_space(5.0);
                                
                                ui.label(
                                    egui::RichText::new(log)
                                        .size(12.0)
                                        .color(egui::Color32::from_rgb(180, 180, 180)),
                                );
                            });
                        }
                        
                        if logs.is_empty() {
                            ui.add_space(10.0);
                            ui.horizontal(|ui| {
                                ui.add_space(5.0);
                                ui.label(
                                    egui::RichText::new("No logs available. Start a service to see output here.")
                                        .size(12.0)
                                        .color(egui::Color32::from_rgb(100, 100, 100))
                                        .italics(),
                                );
                            });
                        }
                        
                        ui.add_space(5.0);
                    });
            });
    }
}