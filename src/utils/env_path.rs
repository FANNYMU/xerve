use std::env;
use std::path::{Path};

pub fn is_in_path(dir: &str) -> bool {
    let current_path = env::var("PATH").unwrap_or_default();
    
    let path_components: Vec<&str> = current_path.split(';').collect();
    
    path_components.contains(&dir)
}


pub fn get_absolute_path(relative_path: &str) -> Result<String, String> {
    let path = Path::new(relative_path);
    
    if !path.exists() {
        return Err(format!("Path {} does not exist", relative_path));
    }
    
    match path.canonicalize() {
        Ok(abs_path) => {
            match abs_path.to_str() {
                Some(path_str) => {
                    let clean_path = if path_str.starts_with(r"\\?\") {
                        &path_str[4..]
                    } else {
                        path_str
                    };
                    Ok(clean_path.to_string())
                },
                None => Err("Failed to convert path to string".to_string())
            }
        },
        Err(e) => Err(format!("Failed to get absolute path: {}", e))
    }
}


#[cfg(windows)]
pub fn add_to_path_permanently(dir: &str) -> Result<(), String> {
    use std::process::Command;
    
    if !Path::new(dir).exists() {
        return Err(format!("Directory {} does not exist", dir));
    }
    
    if is_in_path(dir) {
        return Ok(());
    }

    let mut command = Command::new("powershell.exe");
    command
        .arg("-NoProfile")
        .arg("-ExecutionPolicy")
        .arg("Bypass")
        .arg("-Command")
        .arg(format!(
            "if (([Environment]::GetEnvironmentVariable('PATH', 'User') -split ';') -notcontains '{}') {{ \
             $newPath = [Environment]::GetEnvironmentVariable('PATH', 'User') + ';{}'; \
             [Environment]::SetEnvironmentVariable('PATH', $newPath, 'User') }}",
            dir, dir
        ));
    
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        command.creation_flags(0x08000000); // CREATE_NO_WINDOW
    }
    
    let output = command.output();
        
    match output {
        Ok(output) => {
            if output.status.success() {
                Ok(())
            } else {
                let error = String::from_utf8_lossy(&output.stderr);
                Err(format!("Failed to set PATH: {}", error))
            }
        }
        Err(e) => Err(format!("Failed to execute PowerShell command: {}", e))
    }
}


pub fn add_to_path(dir: &str) -> Result<(), String> {
    if !Path::new(dir).exists() {
        return Err(format!("Directory {} does not exist", dir));
    }
    
    if is_in_path(dir) {
        return Ok(());
    }
    
    let current_path = env::var("PATH").unwrap_or_default();
    
    let new_path = if current_path.is_empty() {
        dir.to_string()
    } else {
        format!("{};{}", current_path, dir)
    };
    
    env::set_var("PATH", &new_path);
    
    Ok(())
}

pub fn get_permanent_path_command(dir: &str) -> String {
    format!(
        "powershell -NoProfile -ExecutionPolicy Bypass -Command \"if (([Environment]::GetEnvironmentVariable('PATH', 'User') -split ';') -notcontains '{}') {{ $newPath = [Environment]::GetEnvironmentVariable('PATH', 'User') + ';{}'; [Environment]::SetEnvironmentVariable('PATH', $newPath, 'User') }}\"",
        dir, dir
    )
}

// pub fn get_current_path() -> String {
//     env::var("PATH").unwrap_or_default()
// }

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_is_in_path() {
        assert!(is_in_path("C:\\WINDOWS\\system32"));
    }
    
    #[test]
    fn test_get_absolute_path() {
        let result = get_absolute_path("./src");
        assert!(result.is_ok());
        assert!(!result.unwrap().starts_with(r"\\?\"));
    }
    
    #[test]
    fn test_add_to_path() {
        let test_dir = "./test_dir";
        std::fs::create_dir_all(test_dir).unwrap();
        
        assert!(add_to_path(test_dir).is_ok());
        
        assert!(add_to_path(test_dir).is_ok());
        
        let _ = std::fs::remove_dir_all(test_dir);
    }
}