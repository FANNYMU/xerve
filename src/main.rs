#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::egui;

fn main() -> eframe::Result {
    env_logger::init();
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1024.0, 768.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Xover",
        options,
        Box::new(|cc| {
            // Configure dark theme
            cc.egui_ctx.set_visuals(egui::Visuals::dark());

            // image support:
            egui_extras::install_image_loaders(&cc.egui_ctx);

            Ok(Box::<Xover>::default())
        }),
    )
}

struct Xover {
    name: String,
    age: u32,
}

impl Default for Xover {
    fn default() -> Self {
        Self {
            name: "Arthur".to_owned(),
            age: 42,
        }
    }
}

impl eframe::App for Xover {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Custom styling
        let mut style = (*ctx.style()).clone();
        style.spacing.button_padding = egui::vec2(16.0, 8.0);
        style.spacing.item_spacing = egui::vec2(8.0, 8.0);
        ctx.set_style(style);

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add_space(40.0);

            ui.vertical_centered(|ui| {
                ui.group(|ui| {
                    ui.set_min_width(600.0);
                    ui.vertical_centered(|ui| {
                        ui.add_space(20.0);

                        ui.heading(
                            egui::RichText::new("Xover")
                                .size(42.0)
                                .strong()
                                .color(egui::Color32::from_rgb(100, 200, 255)),
                        );

                        ui.add_space(8.0);

                        // Subtitle
                        ui.label(
                            egui::RichText::new("Elegant Local Development Platform")
                                .size(18.0)
                                .italics()
                                .color(egui::Color32::from_rgb(180, 180, 180)),
                        );

                        ui.add_space(20.0);
                    });
                });

                ui.add_space(30.0);

                // Services section
                ui.group(|ui| {
                    ui.set_min_width(600.0);
                    ui.vertical(|ui| {
                        ui.add_space(15.0);

                        // Section title
                        ui.horizontal(|ui| {
                            ui.add_space(15.0);
                            ui.label(
                                egui::RichText::new("Services")
                                    .size(24.0)
                                    .strong()
                                    .color(egui::Color32::WHITE),
                            );
                        });

                        ui.add_space(20.0);

                        fn service_row(ui: &mut egui::Ui, name: &str, status: &str) {
                            ui.horizontal(|ui| {
                                ui.add_space(15.0);

                                ui.label("●");
                                ui.add_space(5.0);
                                ui.label(
                                    egui::RichText::new(name)
                                        .size(20.0)
                                        .strong()
                                        .color(egui::Color32::WHITE),
                                );

                                // Flexible spacer
                                ui.with_layout(
                                    egui::Layout::right_to_left(egui::Align::Center),
                                    |ui| {
                                        ui.add_space(15.0);

                                        // Status indicator with better styling
                                        let (status_color, status_bg) = match status {
                                            "Running" => (
                                                egui::Color32::WHITE,
                                                egui::Color32::from_rgb(46, 160, 67),
                                            ),
                                            "Stopped" => (
                                                egui::Color32::WHITE,
                                                egui::Color32::from_rgb(220, 53, 69),
                                            ),
                                            _ => (egui::Color32::WHITE, egui::Color32::GRAY),
                                        };

                                        // Status badge
                                        let status_response = ui.allocate_response(
                                            egui::vec2(80.0, 24.0),
                                            egui::Sense::hover(),
                                        );

                                        ui.painter().rect_filled(
                                            status_response.rect,
                                            4.0,
                                            status_bg,
                                        );

                                        ui.painter().text(
                                            status_response.rect.center(),
                                            egui::Align2::CENTER_CENTER,
                                            status,
                                            egui::FontId::proportional(12.0),
                                            status_color,
                                        );

                                        ui.add_space(15.0);

                                        // Action buttons with consistent sizing
                                        let button_size = egui::vec2(70.0, 32.0);

                                        // Stop button
                                        let stop_button = egui::Button::new(
                                            egui::RichText::new("Stop").color(egui::Color32::WHITE),
                                        )
                                        .fill(egui::Color32::from_rgb(220, 53, 69))
                                        .min_size(button_size);

                                        if ui.add(stop_button).clicked() {
                                            // Stop action
                                            println!("Stopping {}", name);
                                        }

                                        ui.add_space(8.0);

                                        // Start button
                                        let start_button = egui::Button::new(
                                            egui::RichText::new("Start")
                                                .color(egui::Color32::WHITE),
                                        )
                                        .fill(egui::Color32::from_rgb(40, 167, 69))
                                        .min_size(button_size);

                                        if ui.add(start_button).clicked() {
                                            // Start action
                                            println!("Starting {}", name);
                                        }
                                    },
                                );
                            });

                            ui.add_space(15.0);

                            // Subtle separator line
                            ui.horizontal(|ui| {
                                ui.add_space(15.0);
                                ui.separator();
                                ui.add_space(15.0);
                            });

                            ui.add_space(10.0);
                        }

                        // Service rows
                        service_row(ui, "Nginx", "Stopped");
                        service_row(ui, "MySQL", "Running");

                        ui.add_space(10.0);
                    });
                });

                ui.add_space(30.0);

                // Footer section
                ui.horizontal(|ui| {
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.label(
                            egui::RichText::new("v1.0.0")
                                .size(12.0)
                                .color(egui::Color32::from_rgb(120, 120, 120)),
                        );
                    });
                });
            });
        });
    }
}
