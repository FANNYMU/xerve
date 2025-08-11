#![windows_subsystem = "windows"]

use std::sync::Arc;
use eframe::egui;

use crate::utils::load_icon::load_icon_from_file;

mod app;
mod services;
mod ui;
mod utils;

fn main() -> eframe::Result {
    env_logger::init();
    let icon = load_icon_from_file("docs/logo.png").map(Arc::new);
    let mut viewport = egui::ViewportBuilder::default()
        .with_inner_size([1024.0, 768.0]);
    
    if let Some(icon_data) = icon {
        viewport = viewport.with_icon(icon_data);
    }
    
    let options = eframe::NativeOptions {
        viewport,
        ..Default::default()
    };
    eframe::run_native(
        "Xerve",
        options,
        Box::new(|cc| {
            cc.egui_ctx.set_visuals(egui::Visuals::dark());
            egui_extras::install_image_loaders(&cc.egui_ctx);
            let app = app::XerveApp::default();
            services::set_terminal(app.get_terminal());
            Ok(Box::new(app))
        }),
    )
}