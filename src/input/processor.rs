use std::io::{self, Write};

pub fn process() -> String {
    let mut searchv = String::new();
    
    print!("enter your search: ");
    io::stdout().flush().unwrap(); 

    io::stdin()
        .read_line(&mut searchv)
        .expect("Failed to read line");
    
    searchv.trim().to_string()
}
