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
                    *self.process_id.lock().unwrap() = Some(child.id());
                    println!("MariaDB started with process ID: {}", child.id());
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
                    *self.status.lock().unwrap() = "Stopped".to_string();
                }
            }
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
                *self.status.lock().unwrap() = "Stopped".to_string();
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
                    *self.status.lock().unwrap() = "Stopped".to_string();
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
                *self.status.lock().unwrap() = "Stopped".to_string();
            } else {
                eprintln!("Failed to stop MariaDB with taskkill");
                *self.status.lock().unwrap() = "Stopped".to_string();
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