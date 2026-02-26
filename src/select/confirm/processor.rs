use std::io::{self, Write};
use crossterm::style::Stylize;
use crate::models::package::data::Package;

pub fn process(package: &Package) -> bool {
    print!("\nDo you want to install {}? [Y/n] ", package.name.clone().cyan().bold());
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let choice = input.trim().to_lowercase();
    choice.is_empty() || choice == "y" || choice == "yes"
}
