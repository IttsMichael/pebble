use std::process::{Command, Stdio};
use std::io::{BufRead, BufReader, Write, self};
use crossterm::style::Stylize;

use crate::models::{Package, AppConfig};

pub fn process(package: Option<Package>, config: AppConfig) {
    match package {
        Some(pkg) => {
            if config.beta_mode {
                beta(&pkg); 
            } else {
                normal(&pkg);
            }
        }
        None => {
            println!("{}", "aborted".red());
        }
    }
}

fn normal(package: &Package) {
    let package_id = package.name.split_whitespace().next().unwrap_or(&package.name);

    let mut child = Command::new("sudo")
        .args(["pacman", "-S", "--noconfirm", package_id])
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .expect("Failed to execute sudo pacman -S");

    let _ = child.wait().expect("Failed to wait on child process");
}

fn beta(package: &Package) {
    let package_id = package.name.split_whitespace().next().unwrap_or(&package.name);

    let mut child = Command::new("sudo")
        .args(["pacman", "-S", "--noconfirm", package_id])
        .stdin(Stdio::inherit())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to execute sudo pacman -S");

    // Start consuming piped stdout in the animator
    if let Some(stdout) = child.stdout.take() {
        let reader = BufReader::new(stdout);
        
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
        println!("{}", "\nStarting installation...".cyan().bold());

        for (frame_idx, _line) in reader.lines().enumerate() {
            print!("\r{} [{}]", "Installing...".magenta().bold(), frames[frame_idx % frames.len()].yellow().bold());
            io::stdout().flush().unwrap();
        }
        
        // Clear and finalize the line once it finishes
        print!("\r                                     \r");
        println!("{}", "Installation complete!".green().bold());
    }

    let _ = child.wait().expect("Failed to wait on child process");
}
