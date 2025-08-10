use crate::services::{Service, ServiceInfo};
use eframe::egui;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use std::process::{Command, Stdio};

pub struct XerveApp {
    services: Vec<ServiceInfo>,
    terminal: crate::ui::Terminal,
    _php_cgi_process: Option<std::process::Child>,
}

impl XerveApp {
    pub fn cleanup_services(&self) {
        self.terminal.add_log("Cleaning up services...".to_string());
        let mut running_services = 0;
        
        for service in &self.services {
            if service.status() == "Running" {
                running_services += 1;
            }
        }
        
        if running_services == 0 {
            self.terminal.add_log("No services to clean up.".to_string());
            return;
        }
        
        self.terminal.add_log(format!("Found {} running services. Stopping all...", running_services));
        
        for service in &self.services {
            if service.status() == "Running" {
                self.terminal.add_log(format!("Stopping {}...", service.name));
                service.stop();
            }
        }
        
        self.terminal.add_log("Waiting for services to shut down...".to_string());
        
        let start_time = Instant::now();
        let timeout = Duration::from_secs(10);
        let check_interval = Duration::from_millis(500);
        
        loop {
            let mut all_stopped = true;
            for service in &self.services {
                if service.status() == "Running" {
                    all_stopped = false;
                    break;
                }
            }
            
            if all_stopped {
                self.terminal.add_log("All services stopped successfully.".to_string());
                break;
            }
            
            if start_time.elapsed() >= timeout {
                self.terminal.add_log("Timeout waiting for services to stop. Some services may still be running.".to_string());
                break;
            }
            
            std::thread::sleep(check_interval);
        }
        
        self.terminal.add_log("Service cleanup completed.".to_string());
    }
    
    pub fn get_terminal(&self) -> crate::ui::Terminal {
        self.terminal.clone()
    }
    
    fn setup_php_path(&self) {
        let php_dir = "./resource/php-8.4.11";
        
        if !std::path::Path::new(php_dir).exists() {
            self.terminal.add_log(format!("PHP directory not found at {}. Please ensure PHP is installed in the resource directory.", php_dir));
            return;
        }
        
        let abs_php_dir = match crate::utils::env_path::get_absolute_path(php_dir) {
            Ok(path) => path,
            Err(e) => {
                self.terminal.add_log(format!("Failed to get absolute path for PHP directory: {}", e));
                return;
            }
        };
        
        if crate::utils::env_path::is_in_path(&abs_php_dir) {
            self.terminal.add_log("PHP is already in PATH.".to_string());
            return;
        }
        
        #[cfg(windows)]
        match crate::utils::env_path::add_to_path_permanently(&abs_php_dir) {
            Ok(()) => {
                self.terminal.add_log(format!("Successfully added {} to system PATH permanently", abs_php_dir));
            }
            Err(e) => {
                self.terminal.add_log(format!("Failed to permanently add {} to PATH: {}", abs_php_dir, e));
                self.terminal.add_log("Falling back to session-only PATH setting.".to_string());
                
                match crate::utils::env_path::add_to_path(&abs_php_dir) {
                    Ok(()) => {
                        self.terminal.add_log(format!("Successfully added {} to current session PATH", abs_php_dir));
                        self.terminal.add_log("Note: This PATH setting is only valid for the current session.".to_string());
                        self.terminal.add_log(format!("To make it permanent, run this command in PowerShell as Administrator: {}", 
                            crate::utils::env_path::get_permanent_path_command(&abs_php_dir)));
                    }
                    Err(e) => {
                        self.terminal.add_log(format!("Failed to add {} to PATH: {}", abs_php_dir, e));
                    }
                }
            }
        }
        
        #[cfg(not(windows))]
        match crate::utils::env_path::add_to_path(&abs_php_dir) {
            Ok(()) => {
                self.terminal.add_log(format!("Successfully added {} to current session PATH", abs_php_dir));
                self.terminal.add_log("Note: This PATH setting is only valid for the current session.".to_string());
            }
            Err(e) => {
                self.terminal.add_log(format!("Failed to add {} to PATH: {}", abs_php_dir, e));
            }
        }
    }
    
    fn start_php_cgi(&mut self) {
        let php_cgi_path = "./resource/php-8.4.11/php-cgi.exe";
        if !std::path::Path::new(php_cgi_path).exists() {
            self.terminal.add_log(format!("PHP-CGI not found at {}. Skipping PHP-CGI startup.", php_cgi_path));
            return;
        }
        
        let mut command = Command::new(php_cgi_path);
        command
            .arg("-b")
            .arg("127.0.0.1:9000")
            .arg("-c")
            .arg("./resource/php-8.4.11/php.ini")
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());
        
        #[cfg(windows)]
        {
            use std::os::windows::process::CommandExt;
            command.creation_flags(0x08000000); // CREATE_NO_WINDOW
        }
        
        match command.spawn() {
            Ok(child) => {
                self._php_cgi_process = Some(child);
                
                self.terminal.add_log("PHP-CGI started in background on 127.0.0.1:9000".to_string());
            }
            Err(e) => {
                self.terminal.add_log(format!("Failed to start PHP-CGI: {}", e));
            }
        }
    }
}

impl Default for XerveApp {
    fn default() -> Self {
        let nginx_dir = std::path::Path::new("./resource/nginx");
        let nginx_pid_file = nginx_dir.join("logs/nginx.pid");
        
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

        let mut app = XerveApp {
            services: vec![nginx_service, mariadb_service],
            terminal: crate::ui::Terminal::new(),
            _php_cgi_process: None,
        };
        
        app.setup_php_path();
        app.start_php_cgi();
        
        app
    }
}

impl eframe::App for XerveApp {
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
            ui.add_space(30.0);
            ui.vertical_centered(|ui| {
                ui.heading(
                    egui::RichText::new("Xerve")
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
                ui.add_space(20.0);

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

                ui.add_space(20.0);
                
                self.terminal.render(ui);

                ui.add_space(25.0);
                
                ui.label(
                    egui::RichText::new("v1.0.0")
                        .size(12.0)
                        .color(egui::Color32::from_rgb(100, 100, 100)),
                );
            });
        });
    }
}