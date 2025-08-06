use eframe::egui;
use std::sync::{Arc, Mutex};

trait Service {
    fn start(&self);
    fn stop(&self);
    fn status(&self) -> String;
}

struct ServiceInfo {
    name: String,
    status: Arc<Mutex<String>>,
    file_path: String,
}

impl ServiceInfo {
    fn new(name: &str, status: &str, file_path: &str) -> Self {
        ServiceInfo {
            name: name.to_string(),
            status: Arc::new(Mutex::new(status.to_string())),
            file_path: file_path.to_string(),
        }
    }
}

impl Service for ServiceInfo {
    fn start(&self) {
        println!("Starting {} service...", self.name);

        // Check if service is already running
        if self.status() == "Running" {
            println!("{} is already running", self.name);
            return;
        }

        if self.name == "Nginx" {
            let nginx_dir = std::path::Path::new("./resource/nginx");
            let output = std::process::Command::new(&self.file_path)
                .current_dir(nginx_dir)
                .arg("-c")
                .arg("conf/nginx.conf")
                .stdout(std::process::Stdio::piped())
                .stderr(std::process::Stdio::piped())
                .spawn();
            
            match output {
                Ok(_) => {
                    println!("Nginx started successfully");
                    *self.status.lock().unwrap() = "Running".to_string();
                }
                Err(e) => {
                    eprintln!("Failed to start Nginx: {}", e);
                }
            }
        } else {
            let output = std::process::Command::new(&self.file_path)
                .arg("-s")
                .arg("start")
                .stdout(std::process::Stdio::piped())
                .stderr(std::process::Stdio::piped())
                .status();
                
            match output {
                Ok(_) => {
                    *self.status.lock().unwrap() = "Running".to_string();
                }
                Err(e) => {
                    eprintln!("Failed to start {}: {}", self.name, e);
                }
            }
        }
    }

    fn stop(&self) {
        println!("Stopping {} service...", self.name);

        if self.status() == "Stopped" {
            println!("{} is already stopped", self.name);
            return;
        }

        if self.name == "Nginx" {
            let nginx_dir = std::path::Path::new("./resource/nginx");
            
            let pid_file = nginx_dir.join("logs/nginx.pid");
            if !pid_file.exists() {
                println!("Nginx PID file not found, assuming Nginx is not running");
                *self.status.lock().unwrap() = "Stopped".to_string();
                return;
            }
            
            let output = std::process::Command::new(&self.file_path)
                .current_dir(nginx_dir)
                .arg("-s")
                .arg("stop")
                .stdout(std::process::Stdio::piped())
                .stderr(std::process::Stdio::piped())
                .status();
                
            match output {
                Ok(_) => {
                    println!("Nginx stopped successfully");
                    *self.status.lock().unwrap() = "Stopped".to_string();
                }
                Err(e) => {
                    eprintln!("Failed to stop Nginx: {}", e);
                    *self.status.lock().unwrap() = "Stopped".to_string();
                }
            }
        } else {
            let output = std::process::Command::new(&self.file_path)
                .arg("-s")
                .arg("stop")
                .stdout(std::process::Stdio::piped())
                .stderr(std::process::Stdio::piped())
                .status();
                
            match output {
                Ok(_) => {
                    *self.status.lock().unwrap() = "Stopped".to_string();
                }
                Err(e) => {
                    eprintln!("Failed to stop {}: {}", self.name, e);
                }
            }
        }
    }

    fn status(&self) -> String {
        self.status.lock().unwrap().clone()
    }
}

struct ServiceRow<'a> {
    ui: &'a mut egui::Ui,
}

impl<'a> ServiceRow<'a> {
    fn new(ui: &'a mut egui::Ui) -> Self {
        ServiceRow { ui }
    }

    fn render(&mut self, service: &ServiceInfo, name: &str) {
        let status = service.status();
        self.ui.horizontal(|ui| {
            ui.add_space(15.0);
            ui.label("●");
            ui.add_space(5.0);
            ui.label(
                egui::RichText::new(name)
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
                let stop_button =
                    egui::Button::new(egui::RichText::new("Stop").color(egui::Color32::WHITE))
                        .fill(egui::Color32::from_rgb(220, 53, 69))
                        .min_size(button_size);
                if ui.add(stop_button).clicked() {
                    println!("Stopping {}", name);
                    service.stop();
                }
                ui.add_space(8.0);
                let start_button =
                    egui::Button::new(egui::RichText::new("Start").color(egui::Color32::WHITE))
                        .fill(egui::Color32::from_rgb(40, 167, 69))
                        .min_size(button_size);
                if ui.add(start_button).clicked() {
                    println!("Starting {}", name);
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

struct XoverApp {
    name: String,
    age: u32,
    services: Vec<ServiceInfo>,
}

impl Default for XoverApp {
    fn default() -> Self {
        let nginx_dir = std::path::Path::new("./resource/nginx");
        let nginx_pid_file = nginx_dir.join("logs/nginx.pid");
        
        let nginx_status = if nginx_pid_file.exists() {
            "Running"
        } else {
            "Stopped"
        };
        
        let nginx_status = Arc::new(Mutex::new(nginx_status.to_string()));
        
        let mysql_status = Arc::new(Mutex::new("Stopped".to_string()));

        let nginx_service = ServiceInfo::new(
            "Nginx",
            &nginx_status.lock().unwrap(),
            "./resource/nginx/nginx.exe",
        );
        let mysql_service = ServiceInfo::new(
            "MySQL",
            &mysql_status.lock().unwrap(),
            "./resource/mysql/mysql.exe",
        );

        XoverApp {
            name: "Arthur".to_owned(),
            age: 42,
            services: vec![nginx_service, mysql_service],
        }
    }
}

impl eframe::App for XoverApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
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
                ui.group(|ui| {
                    ui.set_min_width(600.0);
                    ui.vertical(|ui| {
                        ui.add_space(15.0);
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
                        let mut service_row = ServiceRow::new(ui);
                        for service in &self.services {
                            let name = service.name.clone();
                            service_row.render(service, &name);
                        }
                        ui.add_space(10.0);
                    });
                });
                ui.add_space(30.0);
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
            cc.egui_ctx.set_visuals(egui::Visuals::dark());
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Ok(Box::new(XoverApp::default()))
        }),
    )
}
