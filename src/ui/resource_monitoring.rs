use eframe::egui;
use crate::services::{Service, ServiceInfo};
use crate::ui::theme;
use sysinfo::{System, ProcessRefreshKind, RefreshKind, MemoryRefreshKind, CpuRefreshKind, Pid};
use std::time::Instant;
use std::collections::HashMap;

#[derive(Clone)]
pub struct ResourceDataPoint {
    cpu_usage: f32,
    memory_usage: u64,
}

pub struct ResourceMonitoring {
    sys: System,
    service_data: HashMap<String, Vec<ResourceDataPoint>>,
    service_pids: HashMap<String, Vec<Pid>>,
    last_update: Instant,
    system_cpu: f32,
    system_memory: u64,
    system_total_memory: u64,
}

impl ResourceMonitoring {
    pub fn new() -> Self {
        let mut sys = System::new_with_specifics(
            RefreshKind::new()
                .with_processes(ProcessRefreshKind::new()
                    .with_cpu()
                    .with_memory())
                .with_memory(MemoryRefreshKind::new())
                .with_cpu(CpuRefreshKind::new())
        );
        
        sys.refresh_all();
        
        ResourceMonitoring {
            sys,
            service_data: HashMap::new(),
            service_pids: HashMap::new(),
            last_update: Instant::now(),
            system_cpu: 0.0,
            system_memory: 0,
            system_total_memory: 0,
        }
    }

    pub fn render(&mut self, ui: &mut egui::Ui, services: &Vec<ServiceInfo>) {
        if self.last_update.elapsed().as_millis() >= 1000 {
            self.update_data(services);
            self.last_update = Instant::now();
        }

        // ui.vertical_centered(|ui| {
        //     ui.heading(
        //         egui::RichText::new("Resource Monitoring")
        //             .size(32.0)
        //             .strong()
        //             .color(theme::ACCENT),
        //     );
        //     ui.add_space(6.0);
        //     ui.label(
        //         egui::RichText::new("Real-time CPU and memory usage for Nginx & MariaDB")
        //             .size(16.0)
        //             .color(theme::TEXT_MUTED),
        //     );
        // });

        ui.add_space(24.0);

        self.render_system_overview(ui);
        ui.add_space(16.0);

        for service in services {
            self.render_service_card(ui, service);
            ui.add_space(16.0);
        }

        if services.is_empty() {
            theme::card_frame(ui.style()).show(ui, |ui| {
                ui.set_min_height(200.0);
                ui.vertical_centered(|ui| {
                    ui.add_space(40.0);
                    ui.label(
                        egui::RichText::new("No services configured")
                            .size(20.0)
                            .color(theme::TEXT_MUTED),
                    );
                    ui.add_space(10.0);
                    ui.label(
                        egui::RichText::new("Add services to start monitoring their resource usage")
                            .size(14.0)
                            .color(theme::TEXT_MUTED),
                    );
                });
            });
        }
    }

    fn update_data(&mut self, services: &Vec<ServiceInfo>) {
        self.sys.refresh_all();
        
        self.system_cpu = self.sys.global_cpu_info().cpu_usage();
        self.system_memory = self.sys.used_memory();
        self.system_total_memory = self.sys.total_memory();

        for service in services {
            let service_name = &service.name;
            
            let (cpu_usage, memory_usage) = if service.status() == "Running" {
                self.get_service_usage(service_name)
            } else {
                self.service_pids.remove(service_name);
                (0.0, 0)
            };
            
            let data_point = ResourceDataPoint {
                cpu_usage,
                memory_usage,
            };
            
            let data_vec = self.service_data.entry(service_name.clone()).or_insert_with(Vec::new);
            data_vec.push(data_point);
            
            if data_vec.len() > 60 {
                data_vec.drain(0..(data_vec.len() - 60));
            }
        }
    }

    fn get_service_usage(&mut self, service_name: &str) -> (f32, u64) {
        let process_names = self.get_process_names_for_service(service_name);
        
        let mut total_cpu = 0.0;
        let mut total_memory = 0u64;
        let mut found_pids = Vec::new();

        for (pid, process) in self.sys.processes() {
            let process_name = process.name();
            
            if !process_names.is_empty() && process_names.iter().any(|name| {
                process_name.to_lowercase().contains(&name.to_lowercase()) ||
                name.to_lowercase().contains(&process_name.to_lowercase())
            }) {
                total_cpu += process.cpu_usage();
                total_memory += process.memory();
                found_pids.push(*pid);
            }
        }

        if !found_pids.is_empty() {
            self.service_pids.insert(service_name.to_string(), found_pids);
        }

        if total_cpu == 0.0 && total_memory == 0 {
            if let Some(pids) = self.service_pids.get(service_name) {
                for pid in pids {
                    if let Some(process) = self.sys.process(*pid) {
                        total_cpu += process.cpu_usage();
                        total_memory += process.memory();
                    }
                }
            }
        }

        (total_cpu, total_memory)
    }

    fn get_process_names_for_service(&self, service_name: &str) -> Vec<&'static str> {
        match service_name {
            "Nginx" => vec!["nginx", "nginx.exe"],
            "MariaDB" => vec!["mariadbd", "mariadbd.exe", "mysqld", "mysqld.exe"],
            _ => vec![], // Only Nginx and MariaDB are supported
        }
    }

    fn render_system_overview(&self, ui: &mut egui::Ui) {
        theme::card_frame(ui.style()).show(ui, |ui| {
            ui.set_min_width(420.0);
            ui.add_space(6.0);

            ui.horizontal(|ui| {
                ui.label(
                    egui::RichText::new("System Overview")
                        .size(20.0)
                        .strong(),
                );
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(
                        egui::RichText::new("Real-time system metrics")
                            .size(12.0)
                            .color(theme::TEXT_MUTED),
                    );
                });
            });

            ui.add_space(12.0);

            ui.horizontal(|ui| {
                ui.label(egui::RichText::new("System CPU:").size(14.0).strong());
                ui.add_space(10.0);
                
                self.render_progress_bar(ui, self.system_cpu, 100.0, theme::BLUE);
                
                ui.add_space(10.0);
                ui.label(egui::RichText::new(format!("{:.1}%", self.system_cpu)).size(14.0));
            });

            ui.add_space(8.0);

            ui.horizontal(|ui| {
                ui.label(egui::RichText::new("System Memory:").size(14.0).strong());
                ui.add_space(10.0);
                
                let memory_percentage = if self.system_total_memory > 0 {
                    (self.system_memory as f32 / self.system_total_memory as f32) * 100.0
                } else {
                    0.0
                };
                
                self.render_progress_bar(ui, memory_percentage, 100.0, theme::GREEN);
                
                ui.add_space(10.0);
                let memory_gb = self.system_memory as f32 / 1_000_000_000.0;
                let total_gb = self.system_total_memory as f32 / 1_000_000_000.0;
                ui.label(egui::RichText::new(format!("{:.1}/{:.1} GB", memory_gb, total_gb)).size(14.0));
            });

            ui.add_space(8.0);

            ui.horizontal(|ui| {
                ui.label(egui::RichText::new("Running Processes:").size(14.0).strong());
                ui.add_space(10.0);
                ui.label(egui::RichText::new(format!("{}", self.sys.processes().len())).size(14.0));
            });
        });
    }

    fn render_service_card(&mut self, ui: &mut egui::Ui, service: &ServiceInfo) {
        let status = service.status();
        let is_running = status == "Running";

        theme::card_frame(ui.style()).show(ui, |ui| {
            ui.set_min_height(280.0);

            ui.horizontal(|ui| {
                ui.label(
                    egui::RichText::new(&service.name)
                        .size(20.0)
                        .strong(),
                );
                
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    let (fg, bg) = theme::status_colors(&status);
                    let status_response = ui.allocate_response(egui::vec2(80.0, 24.0), egui::Sense::hover());
                    ui.painter().rect_filled(status_response.rect, 12.0, bg);
                    ui.painter().text(
                        status_response.rect.center(),
                        egui::Align2::CENTER_CENTER,
                        &status,
                        egui::FontId::proportional(12.0),
                        fg,
                    );
                });
            });

            ui.add_space(16.0);

            if is_running {
                self.render_resource_usage(ui, &service.name);
            } else {
                ui.vertical_centered(|ui| {
                    ui.add_space(40.0);
                    ui.label(
                        egui::RichText::new("Service is not running")
                            .size(16.0)
                            .color(theme::TEXT_MUTED),
                    );
                    ui.add_space(10.0);
                    ui.label(
                        egui::RichText::new("Start the service to monitor its resource usage")
                            .size(14.0)
                            .color(theme::TEXT_MUTED),
                    );
                });
            }
        });
    }

    fn render_resource_usage(&mut self, ui: &mut egui::Ui, service_name: &str) {
        let data_points = self.service_data.get(service_name).cloned().unwrap_or_default();
        
        let current_cpu = data_points.last().map_or(0.0, |p| p.cpu_usage);
        let current_memory = data_points.last().map_or(0, |p| p.memory_usage);

        if let Some(pids) = self.service_pids.get(service_name) {
            if !pids.is_empty() {
                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new("Processes:").size(12.0).color(theme::TEXT_MUTED));
                    ui.label(egui::RichText::new(format!("{} found", pids.len())).size(12.0).color(theme::TEXT_MUTED));
                    
                    if pids.len() <= 3 {
                        for pid in pids {
                            ui.label(egui::RichText::new(format!("PID:{}", pid)).size(10.0).color(theme::TEXT_MUTED));
                        }
                    } else {
                        ui.label(egui::RichText::new(format!("PID:{} +{} more", pids[0], pids.len() - 1)).size(10.0).color(theme::TEXT_MUTED));
                    }
                });
                ui.add_space(4.0);
            }
        }

        ui.horizontal(|ui| {
            ui.label(egui::RichText::new("CPU:").size(14.0).strong());
            ui.add_space(10.0);
            
            self.render_progress_bar(ui, current_cpu, 100.0, theme::BLUE);
            
            ui.add_space(10.0);
            ui.label(egui::RichText::new(format!("{:.1}%", current_cpu)).size(14.0));
        });

        ui.add_space(8.0);

        ui.horizontal(|ui| {
            ui.label(egui::RichText::new("Memory:").size(14.0).strong());
            ui.add_space(10.0);
            
            let memory_mb = current_memory as f32 / 1_000_000.0;
            let max_memory_mb = if memory_mb > 500.0 { memory_mb * 1.2 } else { 500.0 };
            self.render_progress_bar(ui, memory_mb, max_memory_mb, theme::GREEN);
            
            ui.add_space(10.0);
            ui.label(egui::RichText::new(format!("{:.1} MB", memory_mb)).size(14.0));
        });

        ui.add_space(16.0);

        if data_points.len() > 1 {
            self.render_graph(ui, &data_points);
        } else {
            ui.vertical_centered(|ui| {
                ui.add_space(20.0);
                ui.label(
                    egui::RichText::new("Collecting real-time data...")
                        .size(14.0)
                        .color(theme::TEXT_MUTED),
                );
                ui.add_space(4.0);
                ui.label(
                    egui::RichText::new("Monitoring Nginx & MariaDB processes")
                        .size(12.0)
                        .color(theme::TEXT_MUTED),
                );
            });
        }
    }

    fn render_progress_bar(&self, ui: &mut egui::Ui, value: f32, max: f32, color: egui::Color32) {
        let width = 200.0;
        let height = 16.0;
        let (rect, _) = ui.allocate_exact_size(egui::Vec2::new(width, height), egui::Sense::hover());
        
        ui.painter().rect_filled(
            rect,
            8.0,
            theme::BG_DARK,
        );
        
        let percentage = (value / max).min(1.0).max(0.0);
        if percentage > 0.0 {
            let progress_width = percentage * (width - 4.0);
            let progress_rect = egui::Rect::from_min_size(
                rect.min + egui::Vec2::new(2.0, 2.0),
                egui::Vec2::new(progress_width, height - 4.0),
            );
            
            ui.painter().rect_filled(
                progress_rect,
                6.0,
                color,
            );
        }
    }

    fn render_graph(&self, ui: &mut egui::Ui, data_points: &[ResourceDataPoint]) {
        ui.label(egui::RichText::new("Resource Usage History (Real-time)").size(14.0).strong());
        ui.add_space(8.0);
        
        let graph_height = 120.0;
        let graph_width = ui.available_width() - 20.0;
        let (rect, _) = ui.allocate_exact_size(egui::Vec2::new(graph_width, graph_height), egui::Sense::hover());
        
        ui.painter().rect_filled(rect, 8.0, theme::BG_DARK);
        
        if data_points.len() > 1 {
            let padding = 10.0;
            let inner_rect = rect.shrink(padding);
            
            let max_cpu = data_points.iter().map(|p| p.cpu_usage).fold(0.0f32, f32::max).max(10.0);
            let max_memory = data_points.iter().map(|p| p.memory_usage as f32).fold(0.0f32, f32::max).max(10_000_000.0);
        
            let cpu_points: Vec<egui::Pos2> = data_points
                .iter()
                .enumerate()
                .map(|(i, p)| {
                    let x = inner_rect.left() + (i as f32 / (data_points.len() - 1) as f32) * inner_rect.width();
                    let y = inner_rect.bottom() - (p.cpu_usage / max_cpu) * inner_rect.height();
                    egui::Pos2::new(x, y)
                })
                .collect();
            
            if cpu_points.len() > 1 {
                ui.painter().add(egui::Shape::line(
                    cpu_points,
                    egui::Stroke::new(2.0, theme::BLUE),
                ));
            }
            
            let memory_points: Vec<egui::Pos2> = data_points
                .iter()
                .enumerate()
                .map(|(i, p)| {
                    let x = inner_rect.left() + (i as f32 / (data_points.len() - 1) as f32) * inner_rect.width();
                    let normalized_memory = (p.memory_usage as f32 / max_memory) * max_cpu;
                    let y = inner_rect.bottom() - (normalized_memory / max_cpu) * inner_rect.height();
                    egui::Pos2::new(x, y)
                })
                .collect();
            
            if memory_points.len() > 1 {
                ui.painter().add(egui::Shape::line(
                    memory_points,
                    egui::Stroke::new(2.0, theme::GREEN),
                ));
            }
        }
        
        ui.add_space(8.0);
        
        ui.horizontal(|ui| {
            ui.add_space(10.0);
            
            let (rect, _) = ui.allocate_exact_size(egui::Vec2::new(12.0, 2.0), egui::Sense::hover());
            ui.painter().rect_filled(rect, 1.0, theme::BLUE);
            ui.add_space(4.0);
            ui.label(egui::RichText::new("CPU Usage").size(12.0).color(theme::TEXT_MUTED));
            
            ui.add_space(20.0);
            
            let (rect, _) = ui.allocate_exact_size(egui::Vec2::new(12.0, 2.0), egui::Sense::hover());
            ui.painter().rect_filled(rect, 1.0, theme::GREEN);
            ui.add_space(4.0);
            ui.label(egui::RichText::new("Memory Usage").size(12.0).color(theme::TEXT_MUTED));
        });
    }
}