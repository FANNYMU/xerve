use eframe::egui;
use crate::services::{Service, ServiceInfo};
use crate::ui::theme;

pub struct ServiceRow<'a> {
    ui: &'a mut egui::Ui,
}

impl<'a> ServiceRow<'a> {
    pub fn new(ui: &'a mut egui::Ui) -> Self {
        ServiceRow { ui }
    }

    pub fn render(&mut self, service: &ServiceInfo) {
        let status = service.status();

        let response = egui::Frame::new()
            .inner_margin(egui::Margin::symmetric(8i8, 10i8))
            .show(self.ui, |ui| {
                ui.set_min_height(46.0);
                ui.horizontal(|ui| {
                    ui.add_space(8.0);

                    // Status dot
                    let indicator_color = match status.as_str() {
                        "Running" => theme::GREEN,
                        "Stopped" => theme::RED,
                        _ => egui::Color32::GRAY,
                    };
                    let (rect, _) = ui.allocate_exact_size(egui::Vec2::new(10.0, 10.0), egui::Sense::hover());
                    ui.painter().circle_filled(rect.center(), 5.0, indicator_color);

                    ui.add_space(10.0);

                    // Service name
                    ui.label(
                        egui::RichText::new(&service.name)
                            .size(18.0)
                            .strong()
                    );

                    // Right aligned controls
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.add_space(12.0);

                        // Status pill
                        let (fg, bg) = theme::status_colors(&status);
                        let status_response = ui.allocate_response(egui::vec2(88.0, 26.0), egui::Sense::hover());
                        ui.painter().rect_filled(status_response.rect, 13.0, bg);
                        ui.painter().text(
                            status_response.rect.center(),
                            egui::Align2::CENTER_CENTER,
                            &status,
                            egui::FontId::proportional(12.0),
                            fg,
                        );

                        ui.add_space(10.0);

                        let w = ui.available_width();
                        let btn_w = if w < 280.0 { 70.0 } else { 88.0 };
                        let btn_h = if w < 280.0 { 30.0 } else { 34.0 };
                        let button_size = egui::vec2(btn_w, btn_h);

                        // Stop button
                        if ui
                            .add(
                                egui::Button::new(
                                    egui::RichText::new("Stop").color(egui::Color32::WHITE).size(13.0),
                                )
                                .fill(theme::RED)
                                .min_size(button_size)
                                .corner_radius(8.0),
                            )
                            .on_hover_text("Stop the service")
                            .clicked()
                        {
                            service.stop();
                        }

                        ui.add_space(8.0);

                        // Start button
                        if ui
                            .add(
                                egui::Button::new(
                                    egui::RichText::new("Start").color(egui::Color32::WHITE).size(13.0),
                                )
                                .fill(theme::GREEN)
                                .min_size(button_size)
                                .corner_radius(8.0),
                            )
                            .on_hover_text("Start the service")
                            .clicked()
                        {
                            service.start();
                        }
                    });
                });
            })
            .response;

        if response.hovered() {
            let r = response.rect;
            self.ui.painter().rect_filled(r, 8.0, egui::Color32::from_black_alpha(12));
        }

        self.ui.add_space(6.0);
        self.ui.separator();
        self.ui.add_space(6.0);
    }
}