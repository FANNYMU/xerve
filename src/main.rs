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
        } else if self.name == "MariaDB" {
            let mariadb_dir = std::path::Path::new("./resource/mariadb");
            
            let data_dir = mariadb_dir.join("data");
            if !data_dir.exists() {
                println!("MariaDB data directory not found. Please initialize MariaDB first.");
                return;
            }
            
            let output = std::process::Command::new(&self.file_path)
                .current_dir(mariadb_dir)
                .arg("--defaults-file=my.ini")
                .stdout(std::process::Stdio::piped())
                .stderr(std::process::Stdio::piped())
                .spawn();
            
            match output {
                Ok(child) => {
                    std::thread::sleep(std::time::Duration::from_millis(500));
                    println!("MariaDB started with PID: {}", child.id());
                    *self.status.lock().unwrap() = "Running".to_string();
                }
                Err(e) => {
                    eprintln!("Failed to start MariaDB: {}", e);
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
        } else if self.name == "MariaDB" {
            let mariadb_dir = std::path::Path::new("./resource/mariadb");
            
            let output = std::process::Command::new("mysqladmin")
                .current_dir(mariadb_dir)
                .arg("-u")
                .arg("root")
                .arg("shutdown")
                .stdout(std::process::Stdio::piped())
                .stderr(std::process::Stdio::piped())
                .status();
                
            match output {
                Ok(_) => {
                    println!("MariaDB stopped successfully");
                    *self.status.lock().unwrap() = "Stopped".to_string();
                }
                Err(e) => {
                    eprintln!("Failed to stop MariaDB with mysqladmin: {}", e);
                    println!("Trying alternative shutdown method...");
                    
                    let kill_output = std::process::Command::new("taskkill")
                        .arg("/F")
                        .arg("/IM")
                        .arg("mysqld.exe")
                        .stdout(std::process::Stdio::piped())
                        .stderr(std::process::Stdio::piped())
                        .status();
                    
                    match kill_output {
                        Ok(_) => {
                            println!("MariaDB stopped successfully with taskkill");
                            *self.status.lock().unwrap() = "Stopped".to_string();
                        }
                        Err(e) => {
                            eprintln!("Failed to stop MariaDB with taskkill: {}", e);
                            *self.status.lock().unwrap() = "Stopped".to_string();
                        }
                    }
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

    fn render(&mut self, service: &ServiceInfo) {
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

struct XoverApp {
    name: String,
    age: u32,
    services: Vec<ServiceInfo>,
}

impl XoverApp {
    fn cleanup_services(&self) {
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
        
        let nginx_running = nginx_pid_file.exists();
        
        let mariadb_running = false;

        XoverApp {
            name: "Arthur".to_owned(),
            age: 42,
            services: vec![
                ServiceInfo::new(
                    "Nginx",
                    if nginx_running { "Running" } else { "Stopped" },
                    "./resource/nginx/nginx.exe",
                ),
                ServiceInfo::new(
                    "MariaDB",
                    if mariadb_running { "Running" } else { "Stopped" },
                    "./resource/mariadb/bin/mysqld.exe",
                )
            ],
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

                    let mut service_row = ServiceRow::new(ui);
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

fn main() -> eframe::Result {
    env_logger::init();
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1024.0, 768.0]),
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
