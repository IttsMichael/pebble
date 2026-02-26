use std::io::{self, Write};
use crossterm::style::Stylize;

pub fn process() {
    print!("{}", "\nEnter your search: ".green().bold());
    io::stdout().flush().unwrap();
}
