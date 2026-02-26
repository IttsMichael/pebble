use std::io::{self, Write};
use crossterm::style::Stylize;

pub fn process(max: usize) -> usize {
    loop {
        print!("{}", "\nEnter number to install (1-{}): ".replace("{}", &max.to_string()).yellow().bold());
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        if let Ok(num) = input.trim().parse::<usize>() {
            if num > 0 && num <= max {
                return num;
            }
        }
        
        println!("{}", "Invalid selection. Try again.".red());
    }
}
