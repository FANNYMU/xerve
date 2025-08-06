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
}

impl ServiceInfo {
    pub fn new(name: &str, status: &str, file_path: &str) -> Self {
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
                    *self.status.lock().unwrap() = "Running".to_string();
                }
                Err(e) => {
                    eprintln!("Failed to start Nginx: {}", e);
                }
            }
        } else if self.name == "MariaDB" {
            // For MariaDB, we need to set the current directory and specify the config file
            let mariadb_dir = std::path::Path::new("./resource/mariadb");
            
            // Check if data directory exists, if not, we might need to initialize MariaDB
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
            // For Nginx, we need to set the current directory
            let nginx_dir = std::path::Path::new("./resource/nginx");
            
            // Check if nginx.pid exists before trying to stop
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
                    // Still mark as stopped to avoid repeated error messages
                    *self.status.lock().unwrap() = "Stopped".to_string();
                }
            }
        } else if self.name == "MariaDB" {
            // For MariaDB, try to stop using mysqladmin
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
                    // If mysqladmin fails, try with the mysqld process directly
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