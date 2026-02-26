use std::process::{Command, Stdio};
use std::io::{BufRead, BufReader};
use crossterm::style::Stylize;
use crate::models::Package;

pub fn process(query: Option<String>) -> Vec<Package> {
    if let Some(q) = query {
        let results = execute(q);
        format(&results);
        results
    } else {
        Vec::new()
    }
}

fn execute(query: String) -> Vec<Package> {
    // Search is case-insensitive for easier use
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
            // New package line: repo/name version 
            let mut parts = line.split_whitespace();
            if let Some(identifier) = parts.next() {
                // If the package matches our specific query string at all, let's keep it
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
            // Description line
            pkg.description = line.trim().to_string();
            results.push(pkg);
        }
    }

    let _ = child.wait().expect("Failed to wait on pacman child");

    results
}

fn format(results: &[Package]) {
    println!("{}", "\nSearching...\n".cyan().bold());
    
    if results.is_empty() {
        println!("{}", "No packages found.".red());
        return;
    }

    for (i, pkg) in results.iter().enumerate() {
        let index_str = format!("[{}]", i + 1).yellow().bold();
        println!("{} {}", index_str, pkg.name.clone().green().bold());
        println!("    {}", pkg.description.clone().dim());
    }
}
