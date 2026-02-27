use std::process::{Command, Stdio};
use std::io::{BufRead, BufReader};
use crate::models::{Package, AppConfig};

/// Execute a system-level pacman install depending on flags asynchronously
pub fn install_async(
    package: Package, 
    password: Option<String>, 
    tx: std::sync::mpsc::Sender<String>, 
    config: AppConfig
) {
    let _ = tx.send(format!("Starting installation for {}...", package.name));
    
    std::thread::spawn(move || {
        // 1. If a password was provided, verify it synchronously BEFORE starting the massive install thread natively!
        if let Some(pw) = &password {
            if !crate::backend::sudo::verify_password_sync(pw) {
                // Password was wrong! Inform the TUI instantly without ever calling pacman
                let _ = tx.send("1 incorrect password attempt".to_string());
                return;
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
