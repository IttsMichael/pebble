use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};
use crate::models::package::data::Package;

pub fn process(flag: &str) -> Vec<Package> {
    // ensure case-insensitive query (pacman handles case-insensitive regex but we lowercase to be safe)
    let safe_flag = flag.to_lowercase();
    
    let mut child = Command::new("pacman")
        .args(["-Ss", &safe_flag]) 
        .stdout(Stdio::piped())    
        .spawn()
        .expect("Failed to start command");

    let stdout = child.stdout.take().expect("Failed to open stdout");
    let reader = BufReader::new(stdout); 

    let mut results: Vec<Package> = Vec::new();
    let mut lines = reader.lines();

    while let Some(Ok(name_line)) = lines.next() {
        if name_line.trim().is_empty() { continue; }
        
        let desc_line = match lines.next() {
            Some(Ok(desc)) => desc.trim().to_string(),
            _ => String::new(),
        };

        results.push(Package {
            name: name_line.trim().to_string(),
            description: desc_line,
        });
    }
    
    let _ = child.kill();
    results
}
