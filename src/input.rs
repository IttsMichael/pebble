use crossterm::style::Stylize;
use std::io::{self, Write};

pub fn process() -> String {
    prompt();
    read()
}

fn prompt() {
    print!("{}", "Enter your search: ".green().bold());
    io::stdout().flush().unwrap();
}

fn read() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");

    if input.trim().is_empty() {
        return read();
    }
    
    input.trim().to_string()
}
