use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers, MouseButton, MouseEvent, MouseEventKind};
use crate::app::{App, AppMode};

/// Converts a MouseEvent into a logical KeyEvent that `events.rs` can process.
pub fn handle_mouse(app: &App, mouse: MouseEvent) -> Option<KeyEvent> {
    match mouse.kind {
        // --- Scrolling ---
        MouseEventKind::ScrollDown => {
            if !app.search_results.is_empty() && (app.mode == AppMode::Search || app.mode == AppMode::List) {
                return Some(KeyEvent {
                    code: KeyCode::Down,
                    modifiers: KeyModifiers::empty(),
                    kind: KeyEventKind::Press,
                    state: KeyEventState::empty(),
                });
            }
        }
        MouseEventKind::ScrollUp => {
            if !app.search_results.is_empty() && (app.mode == AppMode::Search || app.mode == AppMode::List) {
                return Some(KeyEvent {
                    code: KeyCode::Up,
                    modifiers: KeyModifiers::empty(),
                    kind: KeyEventKind::Press,
                    state: KeyEventState::empty(),
                });
            }
        }

        // --- Clicking ---
        MouseEventKind::Down(MouseButton::Left) => {
            // Header is rows 0..=2. Search Input is rows 3..=5.
            // If the user clicks anywhere in the header or search input (row <= 5),
            // and they are currently in the List, return Esc to jump back to Search mode!
            if mouse.row <= 5 && app.mode == AppMode::List {
                return Some(KeyEvent {
                    code: KeyCode::Esc,
                    modifiers: KeyModifiers::empty(),
                    kind: KeyEventKind::Press,
                    state: KeyEventState::empty(),
                });
            }
            
            // If the user clicks inside the list area (row >= 6) while in Search mode,
            // jump down into the list.
            if mouse.row >= 6 && app.mode == AppMode::Search && !app.search_results.is_empty() {
                return Some(KeyEvent {
                    code: KeyCode::Down,
                    modifiers: KeyModifiers::empty(),
                    kind: KeyEventKind::Press,
                    state: KeyEventState::empty(),
                });
            }
        }
        _ => {}
    }

    None
}
