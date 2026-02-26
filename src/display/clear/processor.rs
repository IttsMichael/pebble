use crossterm::{execute, terminal::{Clear, ClearType}, cursor::MoveTo};
use std::io::stdout;

pub fn process() {
    let mut stdout = stdout();
    let _ = execute!(stdout, Clear(ClearType::All), MoveTo(0, 0));
}
