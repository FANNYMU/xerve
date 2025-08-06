use std::sync::{Arc, Mutex};

pub trait Service {
    fn start(&self);
    fn stop(&self);
    fn status(&self) -> String;
}

pub struct ServiceInfo {
    pub name: String,
    status: Arc<Mutex<String>>,
    file_path: String,
    process_id: Arc<Mutex<Option<u32>>>,
}

impl ServiceInfo {
    pub fn new(name: &str, status: &str, file_path: &str) -> Self {
        ServiceInfo {
            name: name.to_string(),
            status: Arc::new(Mutex::new(status.to_string())),
            file_path: file_path.to_string(),
            process_id: Arc::new(Mutex::new(None)),
        }
    }
}

impl Service for ServiceInfo {
    fn start(&self) {
        println!("Starting {} service...", self.name);

        let mut status_guard = self.status.lock().unwrap();
        if *status_guard == "Running" {
            println!("{} is already running", self.name);
            return;
        }

        if self.name == "Nginx" {
            // For Nginx, we need to set the current directory and specify the config file
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
                    *status_guard = "Running".to_string();
                }
                Err(e) => {
                    eprintln!("Failed to start Nginx: {}", e);
                }
            }
        } else if self.name == "MariaDB" {
            let mariadb_dir = std::path::Path::new("./resource/mariadb");
            let data_dir = mariadb_dir.join("data");

            let mut data_dir_created = false;
            if !data_dir.exists() {
                println!("MariaDB data directory not found. Initializing...");
                
                if let Err(e) = std::fs::create_dir_all(&data_dir) {
                    eprintln!("Failed to create MariaDB data directory: {}", e);
                    return;
                }
                data_dir_created = true;

                let init_status = std::process::Command::new(mariadb_dir.join("bin/mariadb-install-db.exe"))
                    .arg("--datadir=./data")
                    .current_dir(&mariadb_dir)
                    .stdout(std::process::Stdio::inherit())
                    .stderr(std::process::Stdio::inherit())
                    .status();

                match init_status {
                    Ok(status) if status.success() => {
                        println!("✅ MariaDB initialized successfully");
                    }
                    Ok(status) => {
                        eprintln!("❌ MariaDB initialization failed with status: {:?}", status.code());
                        if data_dir_created {
                            if let Err(e) = std::fs::remove_dir_all(&data_dir) {
                                eprintln!("Failed to rollback MariaDB data directory: {}", e);
                            } else {
                                println!("Rolled back MariaDB data directory.");
                            }
                        }
                        return;
                    }
                    Err(e) => {
                        eprintln!("❌ Failed to initialize MariaDB: {}", e);
                        if data_dir_created {
                            if let Err(e) = std::fs::remove_dir_all(&data_dir) {
                                eprintln!("Failed to rollback MariaDB data directory: {}", e);
                            } else {
                                println!("Rolled back MariaDB data directory.");
                            }
                        }
                        return;
                    }
                }
            }

            if !data_dir.exists() || std::fs::read_dir(&data_dir).map(|mut d| d.next().is_none()).unwrap_or(true) {
                eprintln!("MariaDB data directory is missing or empty. Cannot start service.");
                *status_guard = "Error".to_string();
                return;
            }

            println!("Starting MariaDB service...");
            let output = std::process::Command::new(mariadb_dir.join("bin/mysqld.exe"))
                .current_dir(&mariadb_dir)
                .arg("--defaults-file=my.ini")
                .stdout(std::process::Stdio::piped())
                .stderr(std::process::Stdio::piped())
                .spawn();

            match output {
                Ok(mut child) => {
                    let pid = child.id();
                    *self.process_id.lock().unwrap() = Some(pid);

                    let service_name = self.name.clone();
                    let status_arc = Arc::clone(&self.status);
                    std::thread::spawn(move || {
                        match child.wait() {
                            Ok(exit_status) => {
                                if exit_status.success() {
                                    println!("MariaDB process exited successfully (PID: {})", pid);
                                } else {
                                    eprintln!("MariaDB process exited with code: {:?}", exit_status.code());
                                }
                            }
                            Err(e) => {
                                eprintln!("Failed to wait for MariaDB process: {}", e);
                            }
                        }
                        *status_arc.lock().unwrap() = "Stopped".to_string();
                        println!("{} status set to Stopped after process exit.", service_name);
                    });

                    std::thread::sleep(std::time::Duration::from_millis(500));

                    println!("MariaDB started successfully with PID: {}", pid);
                    *status_guard = "Running".to_string();
                }
                Err(e) => {
                    eprintln!("Failed to start MariaDB: {}", e);
                }
            }
        }
        else {
            let output = std::process::Command::new(&self.file_path)
                .arg("-s")
                .arg("start")
                .stdout(std::process::Stdio::piped())
                .stderr(std::process::Stdio::piped())
                .status();
                
            match output {
                Ok(_) => {
                    *status_guard = "Running".to_string();
                }
                Err(e) => {
                    eprintln!("Failed to start {}: {}", self.name, e);
                }
            }
        }
    }

    fn stop(&self) {
        println!("Stopping {} service...", self.name);

        let mut status_guard = self.status.lock().unwrap();

        if *status_guard == "Stopped" {
            println!("{} is already stopped", self.name);
            *self.process_id.lock().unwrap() = None;
            return;
        }
        
        if self.name == "Nginx" {
            // For Nginx, we need to set the current directory
            let nginx_dir = std::path::Path::new("./resource/nginx");
            
            // Check if nginx.pid exists before trying to stop
            let pid_file = nginx_dir.join("logs/nginx.pid");
            if !pid_file.exists() {
                println!("Nginx PID file not found, assuming Nginx is not running. Setting status to Stopped.");
                *status_guard = "Stopped".to_string();
                *self.process_id.lock().unwrap() = None;
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
                    *status_guard = "Stopped".to_string();
                }
                Err(e) => {
                    eprintln!("Failed to stop Nginx: {}", e);
                    *status_guard = "Error".to_string();
                }
            }
            *self.process_id.lock().unwrap() = None;
        } else if self.name == "MariaDB" {
            // For MariaDB, we'll try multiple approaches to stop it
            let mariadb_dir = std::path::Path::new("./resource/mariadb");
            
            let mysqladmin_path = mariadb_dir.join("bin/mysqladmin.exe");
            let output = std::process::Command::new(&mysqladmin_path)
                .current_dir(mariadb_dir)
                .arg("-u")
                .arg("root")
                .arg("shutdown")
                .stdout(std::process::Stdio::piped())
                .stderr(std::process::Stdio::piped())
                .status();
                
            if output.is_ok() {
                println!("MariaDB stopped successfully with mysqladmin");
                *status_guard = "Stopped".to_string();
                *self.process_id.lock().unwrap() = None;
                return;
            }
            
            // If mysqladmin fails, try with the stored process ID
            let process_id = *self.process_id.lock().unwrap();
            if let Some(pid) = process_id {
                eprintln!("Failed to stop MariaDB with mysqladmin, trying to kill process ID {}", pid);
                
                let kill_by_pid = std::process::Command::new("taskkill")
                    .arg("/F")
                    .arg("/PID")
                    .arg(pid.to_string())
                    .stdout(std::process::Stdio::piped())
                    .stderr(std::process::Stdio::piped())
                    .status();
                    
                if kill_by_pid.is_ok() {
                    println!("MariaDB stopped successfully by PID");
                    *status_guard = "Stopped".to_string();
                    *self.process_id.lock().unwrap() = None;
                    return;
                }
            }
            
            eprintln!("Failed to stop MariaDB with mysqladmin and PID, trying alternative methods...");
            
            // Try to kill mysqld process
            let kill_mysqld = std::process::Command::new("taskkill")
                .arg("/F")
                .arg("/IM")
                .arg("mysqld.exe")
                .stdout(std::process::Stdio::piped())
                .stderr(std::process::Stdio::piped())
                .status();
                
            // Try to kill mariadbd process
            let kill_mariadbd = std::process::Command::new("taskkill")
                .arg("/F")
                .arg("/IM")
                .arg("mariadbd.exe")
                .stdout(std::process::Stdio::piped())
                .stderr(std::process::Stdio::piped())
                .status();
                
            if kill_mysqld.is_ok() || kill_mariadbd.is_ok() {
                println!("MariaDB stopped successfully with taskkill");
                *status_guard = "Stopped".to_string();
            } else {
                eprintln!("Failed to stop MariaDB with taskkill");
                *status_guard = "Error".to_string();
            }
            *self.process_id.lock().unwrap() = None;
        } else {
            let output = std::process::Command::new(&self.file_path)
                .arg("-s")
                .arg("stop")
                .stdout(std::process::Stdio::piped())
                .stderr(std::process::Stdio::piped())
                .status();
                
            match output {
                Ok(_) => {
                    *status_guard = "Stopped".to_string();
                }
                Err(e) => {
                    eprintln!("Failed to stop {}: {}", self.name, e);
                }
            }
            *self.process_id.lock().unwrap() = None;
        }
    }

    fn status(&self) -> String {
        self.status.lock().unwrap().clone()
    }
}