use std::process::{Command, Stdio};
use std::io::{BufRead, BufReader, Write};
use crate::models::{Package, AppConfig};

/// Execute a quiet pacman search and parse the results natively
pub fn search(query: &str) -> Vec<Package> {
    if query.trim().is_empty() {
        return Vec::new();
    }

    let query_lower = query.to_lowercase();
    
    let mut child = Command::new("pacman")
        .args(["-Ss", &query_lower])
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to execute pacman");

    let stdout = child.stdout.take().expect("Failed to open pacman stdout");
    let reader = BufReader::new(stdout);
    
    let mut results = Vec::new();
    let mut current_package: Option<Package> = None;

    for line in reader.lines() {
        let line = line.unwrap();
        
        if !line.starts_with("    ") {
            let mut parts = line.split_whitespace();
            if let Some(identifier) = parts.next() {
                if identifier.to_lowercase().contains(&query_lower) {
                    current_package = Some(Package {
                        name: line.clone(),
                        description: String::new(),
                    });
                } else {
                    current_package = None;
                }
            }
        } else if let Some(mut pkg) = current_package.take() {
            pkg.description = line.trim().to_string();
            results.push(pkg);
        }
    }

    let _ = child.wait().expect("Failed to wait on pacman child");
    results
}

pub fn needs_sudo_password() -> bool {
    Command::new("sudo")
        .args(["-n", "true"])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map(|s| !s.success())
        .unwrap_or(true)
}

/// Execute a system-level pacman install depending on flags asynchronously
pub fn install_async(
    package: Package, 
    password: Option<String>, 
    tx: std::sync::mpsc::Sender<String>, 
    config: AppConfig
) {
    let _ = tx.send(format!("Starting installation for {}...", package.name));
    
    std::thread::spawn(move || {
        // 1. If a password was provided, verify it synchronously BEFORE starting the massive install thread!
        if let Some(pw) = &password {
            let mut auth_cmd = Command::new("sudo");
            auth_cmd.args(["-S", "-v"]) // Update cached credentials
               .stdin(Stdio::piped())
               .stdout(Stdio::null())
               .stderr(Stdio::piped());

            if let Ok(mut child) = auth_cmd.spawn() {
                if let Some(mut stdin) = child.stdin.take() {
                    let _ = writeln!(stdin, "{}", pw);
                }
                
                if let Ok(status) = child.wait() {
                    if !status.success() {
                        // Password was wrong! Inform the TUI instantly without ever calling pacman
                        let _ = tx.send("1 incorrect password attempt".to_string());
                        return;
                    }
                }
            }
        }

        let package_id = package.name.split_whitespace().next().unwrap_or(&package.name).to_string();
        
        let mut cmd = Command::new("sudo");
        // No -S required! Credentials are valid!
        cmd.args(["pacman", "-S", "--noconfirm", &package_id]);
        
        cmd.stdin(Stdio::null())
           .stdout(Stdio::piped())
           .stderr(Stdio::piped());
           
        let mut child = match cmd.spawn() {
            Ok(c) => c,
            Err(e) => {
                let _ = tx.send(format!("Failed to spawn sudo command: {}", e));
                return;
            }
        };

        // Pipe stdout to the TUI renderer loop natively!
        if let Some(stdout) = child.stdout.take() {
            let tx_out = tx.clone();
            std::thread::spawn(move || {
                let reader = BufReader::new(stdout);
                let frames = [ "C o o o", "c  o o o", "C   o o", "c    o o", "C     o", "c      o", "C       ", "c       "];
                
                for (frame_idx, line) in reader.lines().enumerate() {
                    if let Ok(l) = line {
                        if config.beta_mode {
                            // Wipe the screen and continuously send frames
                            let _ = tx_out.send(format!("CLRLINE_ILOVECANDYInstalling... [{}]", frames[frame_idx % frames.len()]));
                        } else {
                            let _ = tx_out.send(l);
                        }
                    }
                }
            });
        }

        // Pipe stderr just in case of failure blocks
        if let Some(stderr) = child.stderr.take() {
            let tx_err = tx.clone();
            std::thread::spawn(move || {
                let reader = BufReader::new(stderr);
                for line in reader.lines() {
                    if let Ok(l) = line {
                        let _ = tx_err.send(l);
                    }
                }
            });
        }

        match child.wait() {
            Ok(status) => {
                if status.success() {
                    let _ = tx.send(String::from("\nInstallation Complete! [Press Esc or Enter to return]"));
                } else {
                    let _ = tx.send(format!("\nInstallation failed with status: {} [Press Esc to return]", status));
                }
            }
            Err(e) => {
                let _ = tx.send(format!("\nFailed to wait for the command: {} [Press Esc to return]", e));
            }
        }
    });
}
