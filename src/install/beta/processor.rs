use std::process::{Command, Stdio};
use crate::models::package::data::Package;
use crate::install::beta::animator;

pub fn process(package: &Package) {
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
        animator::processor::process(stdout);
    }

    let _ = child.wait().expect("Failed to wait on child process");
}
