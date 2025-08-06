use crate::services::{Service, ServiceInfo};
use eframe::egui;
use std::sync::{Arc, Mutex};

pub struct XoverApp {
    services: Vec<ServiceInfo>,
}

impl XoverApp {
    pub fn cleanup_services(&self) {
        println!("Cleaning up services...");
        for service in &self.services {
            if service.status() == "Running" {
                println!("Stopping {}...", service.name);
                service.stop();
            }
        }
        std::thread::sleep(std::time::Duration::from_millis(1000)); 
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
            style.spacing.item_spacing = egui::vec2(8.0, 8.0);
            style
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add_space(40.0);
            ui.vertical_centered(|ui| {
                ui.heading(
                    egui::RichText::new("Xover")
                        .size(42.0)
                        .strong()
                        .color(egui::Color32::from_rgb(100, 200, 255)),
                );
                ui.label(
                    egui::RichText::new("Elegant Local Development Platform")
                        .size(18.0)
                        .italics()
                        .color(egui::Color32::from_rgb(180, 180, 180)),
                );
                ui.add_space(30.0);

                ui.group(|ui| {
                    ui.set_min_width(600.0);
                    ui.add_space(15.0);
                    ui.label(
                        egui::RichText::new("Services")
                            .size(24.0)
                            .strong()
                            .color(egui::Color32::WHITE),
                    );
                    ui.add_space(20.0);

                    let mut service_row = crate::ui::ServiceRow::new(ui);
                    for service in &self.services {
                        service_row.render(service);
                    }
                });

                ui.add_space(30.0);
                ui.label(
                    egui::RichText::new("v1.0.0")
                        .size(12.0)
                        .color(egui::Color32::from_rgb(120, 120, 120)),
                );
            });
        });
    }
}