use std::io::{BufRead, BufReader, Write, self};
use std::process::ChildStdout;
use crossterm::style::Stylize;

pub fn process(stdout: ChildStdout) {
    let reader = BufReader::new(stdout);
    
    // Simulate Pacman eating dots (ILoveCandy style)
    let frames = [
        "C o o o",
        "c  o o o",
        "C   o o",
        "c    o o",
        "C     o",
        "c      o",
        "C       ",
        "c       ",
    ];
    let mut frame_idx = 0;

    println!("{}", "\nStarting installation...".cyan().bold());

    for _line in reader.lines() {
        print!("\r{} [{}]", "Installing...".magenta().bold(), frames[frame_idx % frames.len()].yellow().bold());
        io::stdout().flush().unwrap();
        
        frame_idx += 1;
    }
    
    // Clear and finalize the line once it finishes
    print!("\r                                     \r");
    println!("{}", "Installation complete!".green().bold());
}
