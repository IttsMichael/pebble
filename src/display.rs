use crossterm::style::Stylize;
use crossterm::{execute, terminal::{Clear, ClearType}, cursor::MoveTo};
use std::io::stdout;

pub fn process() {
    clear();
    title();
}

fn clear() {
    let mut stdout = stdout();
    let _ = execute!(stdout, Clear(ClearType::All), MoveTo(0, 0));
}

fn title() {
    println!("{}", "Welcome to Pebble!".cyan().bold());
    println!("{}", "Zouzitou is fucking gay".cyan().bold());
    println!("{}", "Use karch btw\n".dim());
}
