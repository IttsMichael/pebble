use std::process::{Command, Stdio};
use std::io::{BufRead, BufReader};
use crate::models::Package;

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

/// Execute a quiet pacman search ONLY for locally installed packages (`pacman -Qs`)
pub fn search_installed(query: &str) -> Vec<Package> {
    if query.trim().is_empty() {
        return Vec::new();
    }

    let query_lower = query.to_lowercase();
    
    let mut child = Command::new("pacman")
        .args(["-Qs", &query_lower])
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to execute pacman -Qs");

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
