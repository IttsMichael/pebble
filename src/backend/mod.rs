pub mod search;
pub mod sudo;
pub mod install;
pub mod uninstall;

// Re-export specific highly used methods strictly cleanly to the frontend!
pub use search::{search, search_installed};
pub use sudo::needs_sudo_password;
pub use install::install_async;
pub use uninstall::uninstall_async;
