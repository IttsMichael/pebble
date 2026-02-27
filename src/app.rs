use crate::models::{AppConfig, Package};
use crate::pacman;
use ratatui::widgets::ListState;

#[derive(PartialEq)]
pub enum AppMode {
    Search,
    List,
    Password,
    Authenticating,
    Installing,
    InstallComplete,
}

pub struct App {
    pub config: AppConfig,
    pub mode: AppMode,
    
    // Search Box State
    pub search_input: String,
    
    // Results List State
    pub search_results: Vec<Package>,
    pub list_state: ListState,
    
    // Installation State
    pub password_input: String,
    pub password_error: Option<String>,
    pub install_logs: Vec<String>,
    pub install_rx: Option<std::sync::mpsc::Receiver<String>>,
    
    // Lifecycle Flags
    pub should_quit: bool,
}

impl App {
    pub fn new(config: AppConfig) -> Self {
        Self {
            config,
            mode: AppMode::Search,
            search_input: String::new(),
            search_results: Vec::new(),
            list_state: ListState::default(),
            password_input: String::new(),
            password_error: None,
            install_logs: Vec::new(),
            install_rx: None,
            should_quit: false,
        }
    }

    /// Triggers a background search and updates the list exactly.
    pub fn execute_search(&mut self) {
        if !self.search_input.is_empty() {
            self.search_results = pacman::search(&self.search_input);
            
            if !self.search_results.is_empty() {
                self.list_state.select(Some(0));
            } else {
                self.list_state.select(None);
            }
        }
    }

    /// Instead of dropping out of the UI, we trigger our sudo prompt or spawn our thread natively.
    pub fn execute_install(&mut self) {
        if self.list_state.selected().is_some() {
            if pacman::needs_sudo_password() {
                self.mode = AppMode::Password;
                self.password_input.clear();
                self.password_error = None;
            } else {
                self.start_install_process(None);
            }
        }
    }

    /// Trigger the async background thread and hook the stdout to our queue
    pub fn start_install_process(&mut self, password: Option<String>) {
        if let Some(i) = self.list_state.selected() {
            if let Some(pkg) = self.search_results.get(i).cloned() {
                self.mode = AppMode::Authenticating;
                self.install_logs.clear();
                
                let (tx, rx) = std::sync::mpsc::channel();
                self.install_rx = Some(rx);
                pacman::install_async(pkg, password, tx, self.config.clone());
            }
        }
    }

    // --- List Navigation Helpers ---
    
    pub fn next(&mut self) {
        let i = match self.list_state.selected() {
            Some(i) => {
                if i >= self.search_results.len().saturating_sub(1) {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.list_state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let i = match self.list_state.selected() {
            Some(i) => {
                if i == 0 {
                    self.search_results.len().saturating_sub(1)
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.list_state.select(Some(i));
    }
}
