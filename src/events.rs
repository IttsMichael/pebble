use ratatui::crossterm::event::{KeyCode, KeyEvent, MouseEvent, MouseEventKind};
use crate::app::{App, AppMode};

pub fn handle_key(app: &mut App, key: KeyEvent) {
    match app.mode {
        // --- When in the search box ---
        AppMode::Search => match key.code {
            KeyCode::Enter => {
                app.execute_search();
                if !app.search_results.is_empty() {
                    // Instantly shift focus to the list once results populate
                    app.mode = AppMode::List; 
                }
            }
            KeyCode::Char(c) => {
                app.search_input.push(c);
            }
            KeyCode::Backspace => {
                app.search_input.pop();
            }
            KeyCode::Esc => {
                app.should_quit = true;
            }
            KeyCode::Down => {
                if !app.search_results.is_empty() {
                    app.mode = AppMode::List;
                }
            }
            _ => {}
        },

        // --- When navigating the list ---
        AppMode::List => match key.code {
            KeyCode::Char('q') | KeyCode::Esc => {
                // Pressing Esc removes focus from the list and returns to the search bar
                app.mode = AppMode::Search;
                app.list_state.select(None);
            }
            KeyCode::Down | KeyCode::Char('j') => {
                app.next();
            }
            KeyCode::Up | KeyCode::Char('k') => {
                app.previous();
            }
            KeyCode::Enter => {
                if app.list_state.selected().is_some() {
                    app.should_install = true;
                }
            }
            // Allow typing instantly to jump back to search mode
            KeyCode::Char(c) => {
                app.mode = AppMode::Search;
                app.search_input.push(c);
                app.list_state.select(None);
            }
            KeyCode::Backspace => {
                app.mode = AppMode::Search;
                app.search_input.pop();
                app.list_state.select(None);
            }
            _ => {}
        },
    }
}

pub fn handle_mouse(app: &mut App, mouse: MouseEvent) {
    match mouse.kind {
        MouseEventKind::ScrollDown => {
            if !app.search_results.is_empty() {
                app.mode = AppMode::List;
                app.next();
            }
        }
        MouseEventKind::ScrollUp => {
            if !app.search_results.is_empty() {
                app.mode = AppMode::List;
                app.previous();
            }
        }
        _ => {}
    }
}
