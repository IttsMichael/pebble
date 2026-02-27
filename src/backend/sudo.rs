use std::process::{Command, Stdio};
use std::io::Write;

/// Validates whether the current system credential cache needs an explicit sudo password
pub fn needs_sudo_password() -> bool {
    Command::new("sudo")
        .args(["-n", "true"])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map(|s| !s.success())
        .unwrap_or(true)
}

/// A strictly isolated authentication check to securely update the sudo credential cache without leaking standard out!
pub fn verify_password_sync(pw: &str) -> bool {
    let mut auth_cmd = Command::new("sudo");
    auth_cmd.args(["-S", "-v"]) // Update cached credentials purely
       .stdin(Stdio::piped())
       .stdout(Stdio::null())
       .stderr(Stdio::piped());

    if let Ok(mut child) = auth_cmd.spawn() {
        if let Some(mut stdin) = child.stdin.take() {
            let _ = writeln!(stdin, "{}", pw);
        }
        
        if let Ok(status) = child.wait() {
            return status.success();
        }
    }
    false
}
