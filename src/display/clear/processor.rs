use crossterm::{execute, terminal::{Clear, ClearType}};
use std::io::stdout;

pub fn process() {
    let mut stdout = stdout();
    let _ = execute!(stdout, Clear(ClearType::All));
}
