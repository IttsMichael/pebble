use crossterm::style::Stylize;
use std::io::{self, Write};
use crate::models::Package;

pub fn process(results: Vec<Package>) -> Option<Package> {
    if results.is_empty() {
        return None;
    }

    let selected_index = prompt(results.len());
    let selected_package = &results[selected_index];
    
    if confirm(selected_package) {
        Some(selected_package.clone())
    } else {
        None
    }
}

fn prompt(max_items: usize) -> usize {
    loop {
        print!("\n{} {} ", "Enter number to install (1-{}):".replace("{}", &max_items.to_string()).yellow().bold(), "".reset());
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");

        let input = input.trim();
        if input.is_empty() {
            continue;
        }

        match input.parse::<usize>() {
            Ok(num) if num > 0 && num <= max_items => return num - 1, // 0-indexed
            _ => println!("{}", "Invalid selection. Please enter a valid number.".red()),
        }
    }
}

fn confirm(package: &Package) -> bool {
    // Extracting the package name cleanly from the raw pacman output line
    let package_id = package.name.split_whitespace().next().unwrap_or(&package.name);
    
    print!("\n{} {} {} ", "Do you want to install".yellow().bold(), package_id.green().bold(), "? [Y/n]".yellow().bold());
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");

    let input = input.trim().to_lowercase();
    
    // Default to yes if empty, otherwise check for y
    input.is_empty() || input == "y" || input == "yes"
}
