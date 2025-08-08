use std::sync::{Arc, Mutex};
use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};
use once_cell::sync::OnceCell;

static TERMINAL: OnceCell<crate::ui::Terminal> = OnceCell::new();

pub fn set_terminal(terminal: crate::ui::Terminal) {
    TERMINAL.set(terminal).ok();
}

fn get_terminal() -> Option<&'static crate::ui::Terminal> {
    TERMINAL.get()
}

fn log_message(message: String) {
    if let Some(terminal) = get_terminal() {
        terminal.add_log(message);
    }
}

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
    
    pub fn update_status_static(status_arc: Arc<Mutex<String>>, new_status: &str) {
        let mut status_guard = status_arc.lock().unwrap();
        *status_guard = new_status.to_string();
    }
    
    #[cfg(windows)]
    fn hide_window(&self, cmd: &mut Command) {
        use std::os::windows::process::CommandExt;
        const CREATE_NO_WINDOW: u32 = 0x08000000;
        cmd.creation_flags(CREATE_NO_WINDOW);
    }

    #[cfg(not(windows))]
    fn hide_window(&self, _cmd: &mut Command) {
        // No special handling needed for non-Windows platforms
    }

    fn run_command_with_output_capture(
        &self,
        mut command: Command,
        operation: &str,
    ) -> Result<Option<std::process::Child>, String> {
        log_message(format!("[{}] Running: {:?}", self.name, command));
        
        self.hide_window(&mut command);

        match command
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
        {
            Ok(mut child) => {
                if let Some(stdout) = child.stdout.take() {
                    let service_name = self.name.clone();
                    std::thread::spawn(move || {
                        let reader = BufReader::new(stdout);
                        for line in reader.lines() {
                            match line {
                                Ok(line) => {
                                    log_message(format!("[{}] {}", service_name, line));
                                }
                                Err(e) => {
                                    log_message(format!("[{}] Error reading stdout: {}", service_name, e));
                                }
                            }
                        }
                    });
                }
                
                if let Some(stderr) = child.stderr.take() {
                    let service_name = self.name.clone();
                    std::thread::spawn(move || {
                        let reader = BufReader::new(stderr);
                        for line in reader.lines() {
                            match line {
                                Ok(line) => {
                                    log_message(format!("[{}] STDERR: {}", service_name, line));
                                }
                                Err(e) => {
                                    log_message(format!("[{}] Error reading stderr: {}", service_name, e));
                                }
                            }
                        }
                    });
                }
                
                if operation == "start" && (self.name == "MariaDB" || self.name == "Nginx") {
                    Ok(Some(child))
                } else {
                    let start_time = Instant::now();
                    let timeout = Duration::from_secs(30);
                    
                    loop {
                        if start_time.elapsed() >= timeout {
                            return Err("Process wait timeout exceeded".to_string());
                        }
                        
                        match child.try_wait() {
                            Ok(Some(status)) => {
                                log_message(format!("[{}] Process finished with status: {}", self.name, status));
                                return Ok(None);
                            }
                            Ok(None) => {
                                std::thread::sleep(Duration::from_millis(100));
                            }
                            Err(e) => {
                                log_message(format!("[{}] Error waiting for process: {}", self.name, e));
                                return Err(e.to_string());
                            }
                        }
                    }
                }
            }
            Err(e) => {
                let error_msg = format!("[{}] Failed to execute command: {}", self.name, e);
                log_message(error_msg.clone());
                Err(error_msg)
            }
        }
    }

    fn update_status(&self, new_status: &str) {
        match self.status.lock() {
            Ok(mut status_guard) => {
                *status_guard = new_status.to_string();
            }
            Err(e) => {
                log_message(format!("Failed to acquire status lock: {}", e));
            }
        }
    }
    
    fn is_running(&self) -> bool {
        match self.status.lock() {
            Ok(status_guard) => {
                *status_guard == "Running"
            }
            Err(e) => {
                log_message(format!("Failed to acquire status lock: {}", e));
                false 
            }
        }
    }
    
    fn is_stopped(&self) -> bool {
        match self.status.lock() {
            Ok(status_guard) => {
                *status_guard == "Stopped"
            }
            Err(e) => {
                log_message(format!("Failed to acquire status lock: {}", e));
                true 
            }
        }
    }
}

impl Service for ServiceInfo {
    fn start(&self) {
        log_message(format!("Starting {} service...", self.name));

        if self.is_running() {
            log_message(format!("{} is already running", self.name));
            return;
        }

        if self.name == "Nginx" {
            let nginx_dir = std::path::Path::new("./resource/nginx");

            let mut command = Command::new(&self.file_path);
            command
                .current_dir(nginx_dir)
                .arg("-c")
                .arg("conf/nginx.conf");

            match self.run_command_with_output_capture(command, "start") {
                Ok(_) => {
                    log_message("Nginx started successfully".to_string());
                    self.update_status("Running");
                }
                Err(e) => {
                    log_message(format!("Failed to start Nginx: {}", e));
                    self.update_status("Error");
                }
            }
        } else if self.name == "MariaDB" {
            let mariadb_dir = std::path::Path::new("./resource/mariadb");
            let data_dir = mariadb_dir.join("data");

            let _data_dir_created = false;
            if !data_dir.exists() {
                log_message("MariaDB data directory not found. Initializing...".to_string());

                if let Err(e) = std::fs::create_dir_all(&data_dir) {
                    log_message(format!("Failed to create MariaDB data directory: {}", e));
                    self.update_status("Error");
                    return;
                }

                let mut init_command = Command::new(mariadb_dir.join("bin/mariadb-install-db.exe"));
                init_command
                    .arg("--datadir=./data")
                    .current_dir(&mariadb_dir);

                match self.run_command_with_output_capture(init_command, "init") {
                    Ok(_) => {
                        log_message("MariaDB initialized successfully".to_string());
                    }
                    Err(e) => {
                        log_message(format!("MariaDB initialization failed: {}", e));
                        if let Err(e) = std::fs::remove_dir_all(&data_dir) {
                            log_message(format!(
                                "Failed to rollback MariaDB data directory: {}",
                                e
                            ));
                        } else {
                            log_message("Rolled back MariaDB data directory.".to_string());
                        }
                        self.update_status("Error");
                        return;
                    }
                }
            }

            if !data_dir.exists()
                || std::fs::read_dir(&data_dir)
                    .map(|mut d| d.next().is_none())
                    .unwrap_or(true)
            {
                log_message(
                    "MariaDB data directory is missing or empty. Cannot start service.".to_string(),
                );
                self.update_status("Error");
                return;
            }

            log_message("Starting MariaDB service...".to_string());
            let mut command = Command::new(mariadb_dir.join("bin/mariadbd.exe")); // Fixed: use mariadbd.exe instead of mysqld.exe
            command
                .current_dir(&mariadb_dir)
                .arg("--defaults-file=my.ini");

            match self.run_command_with_output_capture(command, "start") {
                Ok(Some(mut child)) => {
                    let pid = child.id();
                    match self.process_id.lock() {
                        Ok(mut process_id_guard) => {
                            *process_id_guard = Some(pid);
                        }
                        Err(e) => {
                            log_message(format!("Failed to acquire process_id lock: {}", e));
                        }
                    }

                    let service_name = self.name.clone();
                    let status_arc = Arc::clone(&self.status);
                    std::thread::spawn(move || {
                        match child.wait() {
                            Ok(exit_status) => {
                                log_message(format!(
                                    "{} process exited with status: {}",
                                    service_name, exit_status
                                ));
                            }
                            Err(e) => {
                                log_message(format!(
                                    "Error waiting for {} process: {}",
                                    service_name, e
                                ));
                            }
                        }
                        ServiceInfo::update_status_static(status_arc, "Stopped");
                        log_message(format!(
                            "{} status set to Stopped after process exit.",
                            service_name
                        ));
                    });

                    std::thread::sleep(Duration::from_millis(500));
                    log_message(format!("MariaDB started successfully with PID: {}", pid));
                    self.update_status("Running");
                }
                Ok(None) => {
                    log_message("MariaDB command completed but process not running".to_string());
                    self.update_status("Stopped");
                }
                Err(e) => {
                    log_message(format!("Failed to start MariaDB: {}", e));
                    self.update_status("Error");
                }
            }
        } else {
            let mut command = Command::new(&self.file_path);
            command.arg("-s").arg("start");

            match self.run_command_with_output_capture(command, "start") {
                Ok(_) => {
                    log_message(format!("{} started successfully", self.name));
                    self.update_status("Running");
                }
                Err(e) => {
                    log_message(format!("Failed to start {}: {}", self.name, e));
                    self.update_status("Error");
                }
            }
        }
    }

    fn stop(&self) {
        log_message(format!("Stopping {} service...", self.name));

        if self.is_stopped() {
            log_message(format!("{} is already stopped", self.name));
            match self.process_id.lock() {
                Ok(mut process_id_guard) => {
                    *process_id_guard = None;
                }
                Err(e) => {
                    log_message(format!("Failed to acquire process_id lock: {}", e));
                }
            }
            return;
        }

        if self.name == "Nginx" {
            let nginx_dir = std::path::Path::new("./resource/nginx");

            let pid_file = nginx_dir.join("logs/nginx.pid");
            if !pid_file.exists() {
                log_message("Nginx PID file not found, assuming Nginx is not running. Setting status to Stopped.".to_string());
                self.update_status("Stopped");
                *self.process_id.lock().unwrap() = None;
                return;
            }

            let mut command = Command::new(&self.file_path);
            command.current_dir(nginx_dir).arg("-s").arg("stop");

            match self.run_command_with_output_capture(command, "stop") {
                Ok(_) => {
                    log_message("Nginx stopped successfully".to_string());
                    self.update_status("Stopped");
                }
                Err(e) => {
                    log_message(format!("Failed to stop Nginx: {}", e));
                    self.update_status("Error");
                }
            }
            match self.process_id.lock() {
                Ok(mut process_id_guard) => {
                    *process_id_guard = None;
                }
                Err(e) => {
                    log_message(format!("Failed to acquire process_id lock: {}", e));
                }
            }
        } else if self.name == "MariaDB" {
            let mariadb_dir = std::path::Path::new("./resource/mariadb");

            log_message("Attempting to stop MariaDB with mysqladmin...".to_string());
            let mut mysqladmin_cmd = Command::new(mariadb_dir.join("bin/mysqladmin.exe"));
            mysqladmin_cmd
                .current_dir(mariadb_dir)
                .arg("-u")
                .arg("root")
                .arg("shutdown");

            match self.run_command_with_output_capture(mysqladmin_cmd, "stop") {
                Ok(_) => {
                    log_message("MariaDB stopped successfully with mysqladmin".to_string());
                    self.update_status("Stopped");
                    match self.process_id.lock() {
                        Ok(mut process_id_guard) => {
                            *process_id_guard = None;
                        }
                        Err(e) => {
                            log_message(format!("Failed to acquire process_id lock: {}", e));
                        }
                    }
                    return;
                }
                Err(_) => {
                    log_message(
                        "Failed to stop MariaDB with mysqladmin, trying alternative methods..."
                            .to_string(),
                    );
                }
            }

            let mut kill_mariadbd = Command::new("taskkill");
            kill_mariadbd
                .arg("/F")
                .arg("/IM")
                .arg("mariadbd.exe");
            
            match self.run_command_with_output_capture(kill_mariadbd, "stop") {
                Ok(_) => {
                    log_message(
                        "MariaDB stopped successfully with taskkill (mariadbd)".to_string(),
                    );
                    self.update_status("Stopped");
                    match self.process_id.lock() {
                        Ok(mut process_id_guard) => {
                            *process_id_guard = None;
                        }
                        Err(e) => {
                            log_message(format!("Failed to acquire process_id lock: {}", e));
                        }
                    }
                    return;
                }
                Err(_) => {
                    log_message("Failed to kill mariadbd process".to_string());
                }
            }
            
            match self.process_id.lock() {
                Ok(mut process_id_guard) => {
                    *process_id_guard = None;
                }
                Err(e) => {
                    log_message(format!("Failed to acquire process_id lock: {}", e));
                }
            }
        } else {
            let mut command = Command::new(&self.file_path);
            command.arg("-s").arg("stop");

            match self.run_command_with_output_capture(command, "stop") {
                Ok(_) => {
                    log_message(format!("{} stopped successfully", self.name));
                    self.update_status("Stopped");
                }
                Err(e) => {
                    log_message(format!("Failed to stop {}: {}", self.name, e));
                    self.update_status("Error");
                }
            }
            match self.process_id.lock() {
                Ok(mut process_id_guard) => {
                    *process_id_guard = None;
                }
                Err(e) => {
                    log_message(format!("Failed to acquire process_id lock: {}", e));
                }
            }
        }
    }

    fn status(&self) -> String {
        match self.status.lock() {
            Ok(status_guard) => status_guard.clone(),
            Err(e) => {
                log_message(format!("Failed to acquire status lock: {}", e));
                "Error".to_string()
            }
        }
    }
}
