use std::process::{Command, Stdio};
use std::io::{BufRead, BufReader, Write, self};
use crossterm::style::Stylize;
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

/// Execute a system-level pacman install depending on flags
pub fn install(package: &Package, config: &AppConfig) {
    if config.beta_mode {
        beta(package);
    } else {
        normal(package);
    }
}

fn normal(package: &Package) {
    let package_id = package.name.split_whitespace().next().unwrap_or(&package.name);
    
    // Clear screen to give pacman clean real estate after the Alternate Screen closes!
    print!("{esc}c", esc = 27 as char);

    let mut child = Command::new("sudo")
        .args(["pacman", "-S", package_id])
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .expect("Failed to execute sudo pacman -S");

    let _ = child.wait().expect("Failed to wait on child process");
}

fn beta(package: &Package) {
    let package_id = package.name.split_whitespace().next().unwrap_or(&package.name);
    
    print!("{esc}c", esc = 27 as char);

    let mut child = Command::new("sudo")
        .args(["pacman", "-S", package_id]) // In beta, omit --noconfirm for safety? Or keep it? We keep standard pacman -S for now to let user confirm size
        .stdin(Stdio::inherit())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to execute sudo pacman -S");

    if let Some(stdout) = child.stdout.take() {
        let reader = BufReader::new(stdout);
        
        // ILoveCandy
        let frames = [ "C o o o", "c  o o o", "C   o o", "c    o o", "C     o", "c      o", "C       ", "c       "];
        
        // Print the custom header to bypass output!
        println!("{}", "\nStarting masked installation...".cyan().bold());

        for (frame_idx, _line) in reader.lines().enumerate() {
            print!("\r{} [{}]", "Installing...".magenta().bold(), frames[frame_idx % frames.len()].yellow().bold());
            io::stdout().flush().unwrap();
        }
        
        print!("\r                                     \r");
        println!("{}", "Installation complete!".green().bold());
    }

    let _ = child.wait().expect("Failed to wait on child process");
}
