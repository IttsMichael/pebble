use crossterm::style::Stylize;
use std::io::{self, Write};
use crate::models::Package;

pub fn process(results: Vec<Package>) -> Option<Package> {
    if results.is_empty() {
        return None;
    }

    let selected_index = prompt(results.len());
    
    if let Some(index) = selected_index {
        let selected_package = &results[index];
        
        if confirm(selected_package) {
            Some(selected_package.clone())
        } else {
            None
        }
    } else {
        None
    }
}

fn prompt(max_items: usize) -> Option<usize> {
    loop {
        print!("\n{} ", format!("Enter a number to install (1-{}). 0 to go back to search:", max_items).yellow().bold());
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");

        let input = input.trim();
        if input.is_empty() {
            continue;
        }

        match input.parse::<usize>() {
            Ok(num) if num > 0 && num <= max_items => {
                return Some(num - 1);
            }

            Ok(0) => {
                return None;
            }

            _ => {
                println!("{}", "Invalid selection. Please enter a valid number.".red());
            }
        }
    }
}

fn confirm(package: &Package) -> bool {
    let package_id = package.name.split_whitespace().next().unwrap_or(&package.name);
    
    print!("\n{} {} {} ", "Do you want to install".yellow().bold(), package_id.green().bold(), "? [Y/n]".yellow().bold());
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");

    let input = input.trim().to_lowercase();
    
    input.is_empty() || input == "y" || input == "yes"
}