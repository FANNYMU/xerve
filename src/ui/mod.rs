use eframe::egui;
use crate::services::{Service, ServiceInfo};

pub struct ServiceRow<'a> {
    ui: &'a mut egui::Ui,
}

impl<'a> ServiceRow<'a> {
    pub fn new(ui: &'a mut egui::Ui) -> Self {
        ServiceRow { ui }
    }

    pub fn render(&mut self, service: &ServiceInfo) {
        let status = service.status();
        self.ui.horizontal(|ui| {
            ui.add_space(15.0);
            ui.label("●");
            ui.add_space(5.0);
            ui.label(
                egui::RichText::new(&service.name)
                    .size(20.0)
                    .strong()
                    .color(egui::Color32::WHITE),
            );
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.add_space(15.0);
                let (status_color, status_bg) = match status.as_str() {
                    "Running" => (egui::Color32::WHITE, egui::Color32::from_rgb(46, 160, 67)),
                    "Stopped" => (egui::Color32::WHITE, egui::Color32::from_rgb(220, 53, 69)),
                    _ => (egui::Color32::WHITE, egui::Color32::GRAY),
                };
                let status_response =
                    ui.allocate_response(egui::vec2(80.0, 24.0), egui::Sense::hover());
                ui.painter()
                    .rect_filled(status_response.rect, 4.0, status_bg);
                ui.painter().text(
                    status_response.rect.center(),
                    egui::Align2::CENTER_CENTER,
                    &status,
                    egui::FontId::proportional(12.0),
                    status_color,
                );
                ui.add_space(15.0);
                let button_size = egui::vec2(70.0, 32.0);
                if ui
                    .add(
                        egui::Button::new(
                            egui::RichText::new("Stop").color(egui::Color32::WHITE),
                        )
                        .fill(egui::Color32::from_rgb(220, 53, 69))
                        .min_size(button_size),
                    )
                    .clicked()
                {
                    service.stop();
                }
                ui.add_space(8.0);
                if ui
                    .add(
                        egui::Button::new(
                            egui::RichText::new("Start").color(egui::Color32::WHITE),
                        )
                        .fill(egui::Color32::from_rgb(40, 167, 69))
                        .min_size(button_size),
                    )
                    .clicked()
                {
                    service.start();
                }
            });
        });
        self.ui.add_space(15.0);
        self.ui.horizontal(|ui| {
            ui.add_space(15.0);
            ui.separator();
            ui.add_space(15.0);
        });
        self.ui.add_space(10.0);
    }
}