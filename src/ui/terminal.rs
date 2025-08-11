use eframe::egui;
use std::sync::{Arc, Mutex};
use crate::ui::theme;

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
        theme::card_frame(ui.style())
            .show(ui, |ui| {
                ui.set_min_height(180.0);

                // Header
                ui.horizontal(|ui| {
                    ui.label(
                        egui::RichText::new("Terminal Output")
                            .size(16.0)
                            .strong(),
                    );
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        theme::subtle_label(ui, "READ ONLY", 10.0);
                    });
                });

                ui.add_space(6.0);

                egui::ScrollArea::vertical()
                    .auto_shrink([false; 2])
                    .stick_to_bottom(true)
                    .show(ui, |ui| {
                        ui.set_min_width(ui.available_width() - 10.0);

                        let rect = ui.max_rect();
                        let grid_color = egui::Color32::from_black_alpha(12);
                        let step = 16.0;
                        for x in (rect.left() as i32..rect.right() as i32).step_by(step as usize) {
                            let x = x as f32;
                            ui.painter().line_segment([
                                egui::pos2(x, rect.top()),
                                egui::pos2(x, rect.bottom()),
                            ], egui::Stroke::new(1.0, grid_color));
                        }

                        let logs = self.get_logs();
                        for log in &logs {
                            ui.horizontal(|ui| {
                                theme::subtle_label(ui, ">", 12.0);
                                ui.add_space(5.0);
                                ui.label(
                                    egui::RichText::new(log)
                                        .size(12.0),
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
                                        .italics(),
                                );
                            });
                        }

                        ui.add_space(5.0);
                    });
            });
    }
}