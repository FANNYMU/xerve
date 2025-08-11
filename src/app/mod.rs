use crate::services::{Service, ServiceInfo};
use eframe::egui;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use std::process::{Command, Stdio};
use crate::ui::theme;

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
        
        self.terminal.add_log(format!("Found {running_services} running services. Stopping all..."));
        
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
            self.terminal.add_log(format!("PHP directory not found at {php_dir}. Please ensure PHP is installed in the resource directory."));
            return;
        }
        
        let abs_php_dir = match crate::utils::env_path::get_absolute_path(php_dir) {
            Ok(path) => path,
            Err(e) => {
                self.terminal.add_log(format!("Failed to get absolute path for PHP directory: {e}"));
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
                self.terminal.add_log(format!("Successfully added {abs_php_dir} to system PATH permanently"));
            }
            Err(e) => {
                self.terminal.add_log(format!("Failed to permanently add {abs_php_dir} to PATH: {e}"));
                self.terminal.add_log("Falling back to session-only PATH setting.".to_string());
                
                match crate::utils::env_path::add_to_path(&abs_php_dir) {
                    Ok(()) => {
                        self.terminal.add_log(format!("Successfully added {abs_php_dir} to current session PATH"));
                        self.terminal.add_log("Note: This PATH setting is only valid for the current session.".to_string());
                        self.terminal.add_log(format!("To make it permanent, run this command in PowerShell as Administrator: {}", 
                            crate::utils::env_path::get_permanent_path_command(&abs_php_dir)));
                    }
                    Err(e) => {
                        self.terminal.add_log(format!("Failed to add {abs_php_dir} to PATH: {e}"));
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
            self.terminal.add_log(format!("PHP-CGI not found at {php_cgi_path}. Skipping PHP-CGI startup."));
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
                self.terminal.add_log(format!("Failed to start PHP-CGI: {e}"));
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
        theme::apply_theme(ctx);

        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical()
                .auto_shrink([false; 2])
                .show(ui, |ui| {
                    ui.add_space(20.0);

                    theme::content_container(ui, |ui| {
                        ui.add_space(8.0);

                        // Title
                        ui.vertical_centered(|ui| {
                            ui.heading(
                                egui::RichText::new("Xerve")
                                    .size(46.0)
                                    .strong()
                                    .color(theme::ACCENT),
                            );
                            ui.add_space(6.0);
                            ui.label(
                                egui::RichText::new("Elegant Local Development Platform")
                                    .size(16.0)
                                    .italics()
                                    .color(theme::TEXT_MUTED),
                            );
                        });

                        ui.add_space(18.0);

                        // Services card
                        theme::card_frame(ui.style()).show(ui, |ui| {
                            ui.set_min_width(420.0);
                            ui.add_space(6.0);

                            ui.horizontal(|ui| {
                                ui.label(
                                    egui::RichText::new("Services").size(22.0).strong(),
                                );
                                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                    theme::subtle_label(ui, "Manage local daemons", 12.0);
                                });
                            });

                            ui.add_space(10.0);

                            let mut service_row = crate::ui::ServiceRow::new(ui);
                            for service in &self.services {
                                service_row.render(service);
                            }
                        });

                        ui.add_space(16.0);

                        // Tools card
                        theme::card_frame(ui.style()).show(ui, |ui| {
                            ui.set_min_width(420.0);
                            ui.add_space(6.0);
                            ui.horizontal(|ui| {
                                ui.label(egui::RichText::new("Tools").size(22.0).strong());
                                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                    theme::subtle_label(ui, "Quick actions", 12.0);
                                });
                            });
                            ui.add_space(10.0);

                            ui.horizontal_wrapped(|ui| {
                                let btn = |text: &str, color: egui::Color32| {
                                    egui::Button::new(
                                        egui::RichText::new(text)
                                            .color(egui::Color32::WHITE)
                                            .size(14.0),
                                    )
                                    .fill(color)
                                    .min_size(egui::vec2(160.0, 36.0))
                                    .corner_radius(8.0)
                                };

                                if ui.add(btn("Open htdocs", theme::GREEN)).on_hover_text("Open the web root folder").clicked() {
                                    let htdocs_path = "resource\\nginx\\htdocs";
                                    if std::path::Path::new(htdocs_path).exists() {
                                        match open::that(htdocs_path) {
                                            Ok(_) => self.terminal.add_log("Opening htdocs folder...".to_string()),
                                            Err(e) => self.terminal.add_log(format!("Failed to open htdocs folder: {e}")),
                                        }
                                    } else {
                                        self.terminal.add_log("htdocs folder not found.".to_string());
                                    }
                                }

                                if ui.add(btn("Open phpMyAdmin", theme::BLUE)).on_hover_text("Open phpMyAdmin in your browser").clicked() {
                                    match open::that("http://localhost/phpmyadmin/") {
                                        Ok(_) => self.terminal.add_log("Opening phpMyAdmin in browser...".to_string()),
                                        Err(e) => self.terminal.add_log(format!("Failed to open phpMyAdmin: {e}")),
                                    }
                                }
                            });
                        });

                        ui.add_space(16.0);
                        self.terminal.render(ui);

                        ui.add_space(18.0);
                        ui.vertical_centered(|ui| {
                            theme::subtle_label(ui, "v1.0.3", 12.0);
                        });
                    });
                });
        });
    }
}