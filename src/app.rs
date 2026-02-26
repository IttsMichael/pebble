use crate::models::{AppConfig, Package};
use crate::pacman;
use ratatui::widgets::ListState;

#[derive(PartialEq)]
pub enum AppMode {
    Search,
    List,
}

pub struct App {
    pub config: AppConfig,
    pub mode: AppMode,
    
    // Search Box State
    pub search_input: String,
    
    // Results List State
    pub search_results: Vec<Package>,
    pub list_state: ListState,
    
    // Lifecycle Flags
    pub should_quit: bool,
    pub should_install: bool,
}

impl App {
    pub fn new(config: AppConfig) -> Self {
        Self {
            config,
            mode: AppMode::Search,
            search_input: String::new(),
            search_results: Vec::new(),
            list_state: ListState::default(),
            should_quit: false,
            should_install: false,
        }
    }

    /// Triggers a background search and updates the list exactly.
    pub fn execute_search(&mut self) {
        if !self.search_input.is_empty() {
            // Execute the pacman search! 
            // In a more complex app, this would be an async Tokio task to prevent stutter.
            self.search_results = pacman::search(&self.search_input);
            
            // If we found things, automatically focus the first item!
            if !self.search_results.is_empty() {
                self.list_state.select(Some(0));
            } else {
                self.list_state.select(None);
            }
        }
    }

    /// Triggers the full installation process out of the raw mode
    pub fn execute_install(&mut self) {
        if let Some(i) = self.list_state.selected() {
            if let Some(pkg) = self.search_results.get(i) {
                pacman::install(pkg, &self.config);
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
