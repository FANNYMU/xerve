use crate::services::{Service, ServiceInfo};
use eframe::egui;
use std::sync::{Arc, Mutex};

pub struct XoverApp {
    services: Vec<ServiceInfo>,
}

impl XoverApp {
    pub fn cleanup_services(&self) {
        println!("Cleaning up services...");
        let mut running_services = 0;
        
        // Count running services
        for service in &self.services {
            if service.status() == "Running" {
                running_services += 1;
            }
        }
        
        if running_services == 0 {
            println!("No services to clean up.");
            return;
        }
        
        // Stop all running services
        for service in &self.services {
            // Only try to stop services that are marked as running
            if service.status() == "Running" {
                println!("Stopping {}...", service.name);
                service.stop();
            }
        }
        
        println!("Waiting for services to shut down...");
        std::thread::sleep(std::time::Duration::from_millis(3000));
        println!("Service cleanup completed.");
    }
}

impl Default for XoverApp {
    fn default() -> Self {
        let nginx_dir = std::path::Path::new("./resource/nginx");
        let nginx_pid_file = nginx_dir.join("logs/nginx.pid");
        
        // Check if Nginx is running by checking if nginx.pid file exists
        let nginx_status = if nginx_pid_file.exists() {
            "Running"
        } else {
            "Stopped"
        };
        
        let nginx_status = Arc::new(Mutex::new(nginx_status.to_string()));

        let mariadb_status = Arc::new(Mutex::new("Stopped".to_string()));

        let nginx_service = ServiceInfo::new(
            "Nginx",
            &nginx_status.lock().unwrap(),
            "./resource/nginx/nginx.exe",
        );
        let mariadb_service = ServiceInfo::new(
            "MariaDB",
            &mariadb_status.lock().unwrap(),
            "./resource/mariadb/bin/mariadbd.exe",
        );

        XoverApp {
            services: vec![nginx_service, mariadb_service],
        }
    }
}

impl eframe::App for XoverApp {
    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        self.cleanup_services();
    }
    
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.set_style({
            let mut style = (*ctx.style()).clone();
            style.spacing.button_padding = egui::vec2(16.0, 8.0);
            style.spacing.item_spacing = egui::vec2(10.0, 10.0);
            style.visuals.widgets.noninteractive.bg_fill = egui::Color32::from_rgb(24, 24, 24);
            style.visuals.extreme_bg_color = egui::Color32::from_rgb(18, 18, 18);
            style.visuals.widgets.inactive.bg_fill = egui::Color32::from_rgb(35, 35, 35);
            style.visuals.widgets.hovered.bg_fill = egui::Color32::from_rgb(45, 45, 45);
            style.visuals.widgets.active.bg_fill = egui::Color32::from_rgb(50, 50, 50);
            style.visuals.widgets.open.bg_fill = egui::Color32::from_rgb(55, 55, 55);
            style.visuals.window_fill = egui::Color32::from_rgb(27, 27, 27);
            style.visuals.window_stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(60, 60, 60));
            style.visuals.panel_fill = egui::Color32::from_rgb(20, 20, 20);
            style
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add_space(50.0);
            ui.vertical_centered(|ui| {
                ui.heading(
                    egui::RichText::new("Xover")
                        .size(48.0)
                        .strong()
                        .color(egui::Color32::from_rgb(80, 180, 255)),
                );
                
                ui.label(
                    egui::RichText::new("Elegant Local Development Platform")
                        .size(16.0)
                        .italics()
                        .color(egui::Color32::from_rgb(160, 160, 160)),
                );
                ui.add_space(40.0);

                egui::Frame::group(ui.style())
                    .fill(egui::Color32::from_rgb(30, 30, 30))
                    .stroke(egui::Stroke::new(1.0, egui::Color32::from_rgb(60, 60, 60)))
                    .corner_radius(12.0)
                    .inner_margin(egui::Margin::symmetric(10i8, 10i8))
                    .show(ui, |ui| {
                        ui.set_min_width(620.0);
                        ui.add_space(10.0);
                        
                        ui.label(
                            egui::RichText::new("Services")
                                .size(22.0)
                                .strong()
                                .color(egui::Color32::from_rgb(230, 230, 230)),
                        );
                        ui.add_space(15.0);

                        let mut service_row = crate::ui::ServiceRow::new(ui);
                        for service in &self.services {
                            service_row.render(service);
                        }
                    });

                ui.add_space(35.0);
                
                ui.label(
                    egui::RichText::new("v1.0.0")
                        .size(12.0)
                        .color(egui::Color32::from_rgb(100, 100, 100)),
                );
            });
        });
    }
}