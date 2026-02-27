use ratatui::{
    layout::{Constraint, Direction, Layout, Rect, Alignment},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};
use crate::app::{App, AppMode};

pub fn draw(f: &mut Frame, app: &mut App) {
    // 1. Split screen into 3 chunks: Header (3 lines), Content (remaining), Footer (3 lines)
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(1),
            Constraint::Length(3),
        ])
        .split(f.area());

    draw_header(f, app, chunks[0]);
    draw_content(f, app, chunks[1]);
    draw_footer(f, app, chunks[2]);
}

fn draw_header(f: &mut Frame, _app: &mut App, area: Rect) {
    let header_text = vec![
        Line::from(vec![
            Span::styled(" Pebble Hub ", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
            Span::styled(" — The Arch Package Explorer", Style::default().fg(Color::DarkGray)),
        ]),
    ];

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Cyan));

    let paragraph = Paragraph::new(header_text).block(block);
    f.render_widget(paragraph, area);
}

fn draw_content(f: &mut Frame, app: &mut App, area: Rect) {
    if app.mode == AppMode::Installing || app.mode == AppMode::InstallComplete || app.mode == AppMode::Authenticating {
        let block = Block::default()
            .title(if app.action == crate::app::ActionType::Install { " Installation Logs " } else { " Uninstallation Logs " })
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Magenta));
            
        // Calculate where to scroll to always show the newest logs at the bottom
        let log_count = app.install_logs.len() as u16;
        let view_height = area.height.saturating_sub(2); // subtract borders
        let scroll = if log_count > view_height { log_count - view_height } else { 0 };

        let text: Vec<Line> = app.install_logs.iter().map(|l| Line::from(l.clone())).collect();
        let paragraph = Paragraph::new(text).block(block).scroll((scroll, 0));
        f.render_widget(paragraph, area);
        return;
    }

    if app.mode == AppMode::Home {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(30), Constraint::Percentage(40), Constraint::Percentage(30)])
            .split(area);

        let button_row = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(20),
                Constraint::Percentage(25),
                Constraint::Percentage(10),
                Constraint::Percentage(25),
                Constraint::Percentage(20),
            ])
            .split(chunks[1]);

        let install_style = if app.home_selected_index == 0 {
            Style::default().fg(Color::Green).add_modifier(Modifier::BOLD | Modifier::REVERSED)
        } else {
            Style::default().fg(Color::Green)
        };

        let uninstall_style = if app.home_selected_index == 1 {
            Style::default().fg(Color::Red).add_modifier(Modifier::BOLD | Modifier::REVERSED)
        } else {
            Style::default().fg(Color::Red)
        };

        let install_block = Block::default().borders(Borders::ALL).border_style(install_style);
        let uninstall_block = Block::default().borders(Borders::ALL).border_style(uninstall_style);

        let install_text = Paragraph::new("\n\nInstall").alignment(Alignment::Center).block(install_block);
        let uninstall_text = Paragraph::new("\n\nUninstall").alignment(Alignment::Center).block(uninstall_block);

        f.render_widget(install_text, button_row[1]);
        f.render_widget(uninstall_text, button_row[3]);
        return;
    }

    // Split the content area into the Search Bar on top and the List below
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Search Input box is 3 tall
            Constraint::Min(1),    // List takes the rest
        ])
        .split(area);

    // --- Draw Search Box ---
    let search_style = match app.mode {
        AppMode::Search => Style::default().fg(Color::Yellow),
        _ => Style::default().fg(Color::DarkGray),
    };

    let title = if app.action == crate::app::ActionType::Install {
        " Search Packages "
    } else {
        " Search Installed Packages "
    };

    let search_block = Block::default()
        .title(title)
        .borders(Borders::ALL)
        .border_style(search_style);

    let search_text = Paragraph::new(app.search_input.clone()).block(search_block);
    f.render_widget(search_text, chunks[0]);

    // Show cursor if we're in search mode
    if app.mode == AppMode::Search {
        f.set_cursor_position((
            chunks[0].x + app.search_input.len() as u16 + 1,
            chunks[0].y + 1,
        ));
    }

    // --- Draw Interactive Results List ---
    let items: Vec<ListItem> = app
        .search_results
        .iter()
        .map(|pkg| {
            let parts: Vec<&str> = pkg.name.split_whitespace().collect();
            let name_str = parts.get(0).unwrap_or(&"Unknown");
            
            let line1 = Line::from(Span::styled(name_str.to_string(), Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)));
            let line2 = Line::from(Span::styled(pkg.description.clone(), Style::default().fg(Color::DarkGray)));
            
            ListItem::new(vec![line1, line2])
        })
        .collect();

    let list_style = match app.mode {
        AppMode::List => Style::default().fg(Color::Yellow),
        _ => Style::default().fg(Color::DarkGray),
    };

    let list = List::new(items)
        .block(Block::default().title(" Packages ").borders(Borders::ALL).border_style(list_style))
        .highlight_style(Style::default().bg(Color::DarkGray).add_modifier(Modifier::BOLD))
        .highlight_symbol(">> ");

    f.render_stateful_widget(list, chunks[1], &mut app.list_state);

    // --- Draw Loading Overlay During Auth ---
    if app.mode == AppMode::Authenticating {
        let popup_y = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length((area.height.saturating_sub(3)) / 2), Constraint::Length(3), Constraint::Min(0)])
            .split(area)[1];
            
        let popup_area = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Length((area.width.saturating_sub(30)) / 2), Constraint::Length(30), Constraint::Min(0)])
            .split(popup_y)[1];

        let auth_block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD));
            
        let auth_text = if app.action == crate::app::ActionType::Install {
            "Authenticating..."
        } else {
            "Uninstalling app..."
        };
            
        let paragraph = Paragraph::new(auth_text).alignment(Alignment::Center).block(auth_block);
        
        f.render_widget(ratatui::widgets::Clear, popup_area); 
        f.render_widget(paragraph, popup_area);
    }

    // --- Draw Sudo Password Overlay Modal ---
    if app.mode == AppMode::Password {
        let popup_y = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length((area.height.saturating_sub(3)) / 2), Constraint::Length(3), Constraint::Min(0)])
            .split(area)[1];
            
        let popup_area = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Length((area.width.saturating_sub(40)) / 2), Constraint::Length(40), Constraint::Min(0)])
            .split(popup_y)[1];

        let mut pw_block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Red).add_modifier(Modifier::BOLD));
            
        if let Some(err) = &app.password_error {
            pw_block = pw_block.title(format!(" {} ", err));
        } else {
            pw_block = pw_block.title(" Sudo Password Required ");
        }
            
        let hidden_pw = "*".repeat(app.password_input.len());
        let paragraph = Paragraph::new(hidden_pw).block(pw_block);
        
        f.render_widget(ratatui::widgets::Clear, popup_area); // Clear background directly behind the modal overlay!
        f.render_widget(paragraph, popup_area);
        
        // Trap the cursor in the modal!
        f.set_cursor_position((
            popup_area.x + app.password_input.len() as u16 + 1,
            popup_area.y + 1,
        ));
    }
}

fn draw_footer(f: &mut Frame, app: &mut App, area: Rect) {
    let hints = match app.mode {
        AppMode::Home => "[Left/Right] Select Action   [Enter] Confirm   [Esc/Q] Quit",
        AppMode::Search => "[Enter] Search   [Esc] Home   [Up/Down] Navigate",
        AppMode::List => {
            if app.action == crate::app::ActionType::Install {
                "[Enter] Install   [Esc] Search   [Down/J] Next   [Up/K] Prev"
            } else {
                "[Enter] Uninstall   [Esc] Search   [Down/J] Next   [Up/K] Prev"
            }
        },
        AppMode::Password => "[Enter] Submit Password   [Esc] Cancel",
        AppMode::Authenticating => {
            if app.action == crate::app::ActionType::Install {
                "Verifying authentication in background..."
            } else {
                "Uninstalling app..."
            }
        },
        AppMode::Installing => {
            if app.action == crate::app::ActionType::Install {
                "Installing securely in background..."
            } else {
                "Uninstalling securely in background..."
            }
        },
        AppMode::InstallComplete => "[Enter/Esc] Return to Home",
    };

    let footer = Paragraph::new(Line::from(Span::styled(hints, Style::default().fg(Color::Gray))))
        .block(Block::default().borders(Borders::ALL).border_style(Style::default().fg(Color::DarkGray)));
    
    f.render_widget(footer, area);
}
