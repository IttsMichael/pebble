use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};

pub fn process(flag: Option<String>) -> Option<String> {
    if let Some(f) = flag {
        let mut child = Command::new("pacman")
            .args(["-Ss", &f]) 
            .stdout(Stdio::piped())    
            .spawn()
            .expect("Failed to start command");

        let stdout = child.stdout.take().expect("Failed to open stdout");
        let reader = BufReader::new(stdout); 

        for line in reader.lines() {
            match line {
                Ok(content) => {
                    println!("searching...");
                    println!("{}", content);
                    break; 
                }
                Err(e) => eprintln!("Error reading output: {}", e),
            }
        }
        
        let _ = child.kill();
    }

    None 
}
