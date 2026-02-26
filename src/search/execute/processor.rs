use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};

pub fn process(flag: &str) -> Option<String> {
    let mut child = Command::new("pacman")
        .args(["-Ss", flag]) 
        .stdout(Stdio::piped())    
        .spawn()
        .expect("Failed to start command");

    let stdout = child.stdout.take().expect("Failed to open stdout");
    let reader = BufReader::new(stdout); 

    let mut result = None;
    for line in reader.lines() {
        if let Ok(content) = line {
            result = Some(content);
            break; 
        }
    }
    
    let _ = child.kill();
    result
}
