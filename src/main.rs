use std::io::{self, Write, BufRead, BufReader};
use std::process::{Command, Stdio};

fn main() {
    println!("Welcome to pebble!");
    println!("zouzitou is fucking gay");
    println!("use karch btw"); 
    let mut searchv = String::new();
    
    print!("enter your search: ");
    io::stdout().flush().unwrap(); 

    io::stdin()
        .read_line(&mut searchv)
        .expect("Failed to read line");
    
  
    let searchv = searchv.trim().to_string();
    
    search(Some(searchv));
}

fn search(flag: Option<String>) -> Option<String> {
  
    if let Some(f) = flag {
       
        let mut child = Command::new("sudo")
            .args(["pacman", "-Ss", &f]) 
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