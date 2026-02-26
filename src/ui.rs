use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
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
        AppMode::List => Style::default().fg(Color::DarkGray),
    };

    let search_block = Block::default()
        .title(" Search Query ")
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
        AppMode::Search => Style::default().fg(Color::DarkGray),
    };

    let list = List::new(items)
        .block(Block::default().title(" Packages ").borders(Borders::ALL).border_style(list_style))
        .highlight_style(Style::default().bg(Color::DarkGray).add_modifier(Modifier::BOLD))
        .highlight_symbol(">> ");

    f.render_stateful_widget(list, chunks[1], &mut app.list_state);
}

fn draw_footer(f: &mut Frame, app: &mut App, area: Rect) {
    let hints = match app.mode {
        AppMode::Search => "[Enter] Search   [Esc] Quit   [Up/Down] Navigate",
        AppMode::List => "[Enter] Install   [Esc] Search   [Down/J] Next   [Up/K] Prev",
    };

    let footer = Paragraph::new(Line::from(Span::styled(hints, Style::default().fg(Color::Gray))))
        .block(Block::default().borders(Borders::ALL).border_style(Style::default().fg(Color::DarkGray)));
    
    f.render_widget(footer, area);
}
