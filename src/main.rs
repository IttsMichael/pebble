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

        // Handle specific state transitions (like running the install blocking)
        if app.should_install {
            // Restore terminal to let `pacman` and `sudo` draw securely to standard out
            disable_raw_mode()?;
            execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
            terminal.show_cursor().unwrap();
            
            app.execute_install();
            app.should_install = false; // Reset trigger
            
            // Wait for user to read output, then bring the TUI hub back online
            println!("\nPress Enter to return to Pebble Hub...");
            let mut buf = String::new();
            io::stdin().read_line(&mut buf)?;
            
            enable_raw_mode()?;
            execute!(terminal.backend_mut(), EnterAlternateScreen)?;
            terminal.clear().unwrap();
        }
    }
}