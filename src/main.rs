use ratatui::crossterm::event::{self, Event, KeyEventKind};
use ratatui::crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use ratatui::crossterm::execute;
use ratatui::prelude::*;
use std::io::{self, stdout};
use std::time::Duration;

mod app;
mod ui;
mod events;
mod pacman;
mod models;

use app::App;
use models::AppConfig;

fn main() -> io::Result<()> {
    // 1. Parse arguments.
    let config = AppConfig::parse_args();

    // 2. Setup the raw terminal and alternate screen for Ratatui.
    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout))?;

    // 3. Initialize the application state.
    let mut app = App::new(config);

    // 4. Run the main event loop.
    let result = run_app(&mut terminal, &mut app);

    // 5. Restore the terminal beautifully once the app quits.
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    result
}

fn run_app<B: Backend + io::Write>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<()> {
    loop {
        // Draw the UI for this exact frame based on our app state
        terminal.draw(|f| ui::draw(f, app)).unwrap();

        // Poll for events (keyboard/mouse) for 50ms ( ~20 fps )
        if event::poll(Duration::from_millis(50))? {
            if let Event::Key(key) = event::read()? {
                // Ensure we only trigger on key *presses* not releases
                if key.kind == KeyEventKind::Press {
                    events::handle_key(app, key);
                }
            } else if let Event::Mouse(mouse) = event::read()? {
                events::handle_mouse(app, mouse);
            }
        }

        // Did the user trigger an exit?
        if app.should_quit {
            return Ok(());
        }

                // We poll the RX queue continuously and update the UI natively
        if let Some(rx) = &app.install_rx {
            while let Ok(msg) = rx.try_recv() {
                if msg.starts_with("Starting installation for") {
                    // Auth was successful! Render the logs list going forward natively!
                    app.mode = app::AppMode::Installing;
                    app.install_logs.push(msg);
                } else if msg.starts_with("CLRLINE_ILOVECANDY") {
                    // Update the last line for ILoveCandy animation effect!
                    let anim_frame = msg.replace("CLRLINE_ILOVECANDY", "");
                    if let Some(last) = app.install_logs.last_mut() {
                        if last.starts_with("Installing...") {
                            *last = anim_frame;
                        } else {
                            app.install_logs.push(anim_frame);
                        }
                    } else {
                        app.install_logs.push(anim_frame);
                    }
                } else if msg.contains("1 incorrect password attempt") || msg.contains("Sorry, try again") {
                    // Instantly kick the user back to the password modal natively!
                    app.mode = app::AppMode::Password;
                    app.password_error = Some("Incorrect password. Please try again.".to_string());
                    app.password_input.clear();
                    app.install_logs.clear();
                    app.install_rx = None;
                    break;
                } else if msg.contains("Installation Complete") || msg.contains("failed with status") {
                    app.install_logs.push(msg);
                    app.mode = app::AppMode::InstallComplete;
                } else {
                    app.install_logs.push(msg);
                }
            }
        }
    }
}