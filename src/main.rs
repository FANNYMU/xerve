use eframe::egui;
use env_logger;

mod app;
mod services;
mod ui;

fn main() -> eframe::Result {
    env_logger::init();
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1024.0, 768.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Xerve",
        options,
        Box::new(|cc| {
            cc.egui_ctx.set_visuals(egui::Visuals::dark());
            egui_extras::install_image_loaders(&cc.egui_ctx);
            let app = app::XoverApp::default();
            services::set_terminal(app.get_terminal());
            Ok(Box::new(app))
        }),
    )
}