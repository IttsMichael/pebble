use std::process::{Command, Stdio};
use crate::models::package::data::Package;

pub fn process(package: &Package) {
    // We only need the package name which is the stuff before the space/version inside pacman -Ss
    // The pacman names are usually like `repository/packagename version-release`
    // However, the `name` field in our parser is the whole first line.
    // Let's trim off the repository prefix (e.g., core/ or extra/) and any trailing parts if they exist,
    // though pacman can install from the full `repository/packagename` string too!
    
    // We should just use the full string since `pacman -S cachyos/localsend` works perfectly.
    let package_id = package.name.split_whitespace().next().unwrap_or(&package.name);

    let mut child = Command::new("sudo")
        .args(["pacman", "-S", package_id])
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .expect("Failed to execute sudo pacman -S");

    let _ = child.wait().expect("Failed to wait on child process");
}
