use crate::tui::app::{App, AppState};
use crate::thread::Message;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, List, ListItem, ListState, Paragraph, Wrap},
    Frame,
};

pub fn draw(f: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(1), Constraint::Length(1)])
        .split(f.area());

    // Main content area
    draw_main_content(f, app, chunks[0]);
    
    // Status bar
    draw_status_bar(f, app, chunks[1]);
}

fn draw_main_content(f: &mut Frame, app: &mut App, area: Rect) {
    match app.state {
        AppState::EmailList => draw_email_list(f, app, area),
        AppState::EmailView => draw_email_view(f, app, area),
        AppState::Search => draw_search(f, app, area),
        AppState::Help => draw_help(f, app, area),
        AppState::Compose => draw_compose(f, app, area),
    }
}

fn draw_email_list(f: &mut Frame, app: &mut App, area: Rect) {
    let items: Vec<ListItem> = app
        .search_results
        .iter()
        .enumerate()
        .map(|(i, search_item)| {
            let style = if i == app.selected_email {
                Style::default().bg(Color::Blue).fg(Color::White)
            } else if search_item.tags.iter().any(|tag| tag == "unread") {
                Style::default().add_modifier(Modifier::BOLD)
            } else {
                Style::default()
            };

            let authors = &search_item.authors;
            let subject = &search_item.subject;
            let date = chrono::DateTime::from_timestamp(search_item.timestamp, 0)
                .map(|dt| dt.format("%Y-%m-%d %H:%M").to_string())
                .unwrap_or_else(|| "Unknown".to_string());
            
            // Format: "Authors: Subject [Date]"
            let content = format!("{}: {} [{}]", authors, subject, date);
            
            ListItem::new(content).style(style)
        })
        .collect();

    let mut state = ListState::default();
    state.select(Some(app.selected_email));

    let list = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(format!("Inbox ({} emails)", app.email_count()))
        )
        .highlight_style(Style::default().bg(Color::Blue).fg(Color::White));

    f.render_stateful_widget(list, area, &mut state);
}

fn draw_email_view(f: &mut Frame, app: &mut App, area: Rect) {
    if let Some(message) = &app.current_email {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(6), Constraint::Min(1)])
            .split(area);

        // Email headers
        draw_email_headers(f, message, chunks[0]);
        
        // Email body
        draw_email_body(f, message, chunks[1]);
    } else {
        let paragraph = Paragraph::new("No email selected")
            .block(Block::default().borders(Borders::ALL).title("Email View"));
        f.render_widget(paragraph, area);
    }
}

fn draw_email_headers(f: &mut Frame, message: &Message, area: Rect) {
    let from = &message.headers.from;
    let to = &message.headers.to;
    let subject = &message.headers.subject;
    let date = chrono::DateTime::from_timestamp(message.timestamp, 0)
        .map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string())
        .unwrap_or_else(|| "Unknown".to_string());

    let headers_text = vec![
        Line::from(vec![
            Span::styled("From: ", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(from),
        ]),
        Line::from(vec![
            Span::styled("To: ", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(to),
        ]),
        Line::from(vec![
            Span::styled("Subject: ", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(subject),
        ]),
        Line::from(vec![
            Span::styled("Date: ", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(date.to_string()),
        ]),
    ];

    let headers = Paragraph::new(headers_text)
        .block(Block::default().borders(Borders::ALL).title("Headers"))
        .wrap(Wrap { trim: true });

    f.render_widget(headers, area);
}

fn draw_email_body(f: &mut Frame, message: &Message, area: Rect) {
    // For now, we'll display the first text part or a placeholder
    let body_text = if !message.body.is_empty() {
        if let Some(text_part) = message.body.iter().find(|part| part.content_type.starts_with("text/plain")) {
            match &text_part.content {
                crate::body::BodyContent::Text(text) => text.clone(),
                _ => "[No text content]".to_string(),
            }
        } else if let Some(html_part) = message.body.iter().find(|part| part.content_type.starts_with("text/html")) {
            // TODO: Convert HTML to text using our existing converter
            match &html_part.content {
                crate::body::BodyContent::Text(html) => format!("[HTML content - {} chars]", html.len()),
                _ => "[No HTML content]".to_string(),
            }
        } else {
            "[No readable content]".to_string()
        }
    } else {
        "[No body content]".to_string()
    };

    let paragraph = Paragraph::new(body_text)
        .block(Block::default().borders(Borders::ALL).title("Content"))
        .wrap(Wrap { trim: true });

    f.render_widget(paragraph, area);
}

fn draw_search(f: &mut Frame, app: &mut App, area: Rect) {
    let paragraph = Paragraph::new(format!("Search: {}", app.search_query))
        .block(Block::default().borders(Borders::ALL).title("Search"));
    f.render_widget(paragraph, area);
}

fn draw_help(f: &mut Frame, _app: &mut App, area: Rect) {
    let help_text = vec![
        Line::from("Whynot TUI - Keyboard Shortcuts"),
        Line::from(""),
        Line::from("Navigation:"),
        Line::from("  j/↓     - Move down"),
        Line::from("  k/↑     - Move up"),
        Line::from("  Enter   - Open selected email"),
        Line::from("  Esc     - Go back"),
        Line::from(""),
        Line::from("Actions:"),
        Line::from("  /       - Search"),
        Line::from("  c       - Compose"),
        Line::from("  r       - Reply"),
        Line::from("  f       - Forward"),
        Line::from("  ?       - Show this help"),
        Line::from("  q       - Quit"),
        Line::from(""),
        Line::from("Press any key to continue..."),
    ];

    let paragraph = Paragraph::new(help_text)
        .block(Block::default().borders(Borders::ALL).title("Help"))
        .wrap(Wrap { trim: true });

    // Center the help dialog
    let popup_area = centered_rect(80, 80, area);
    f.render_widget(Clear, popup_area);
    f.render_widget(paragraph, popup_area);
}

fn draw_compose(f: &mut Frame, _app: &mut App, area: Rect) {
    let paragraph = Paragraph::new("Compose mode - Not yet implemented")
        .block(Block::default().borders(Borders::ALL).title("Compose"));
    f.render_widget(paragraph, area);
}

fn draw_status_bar(f: &mut Frame, app: &mut App, area: Rect) {
    let status_text = if let Some(ref message) = app.status_message {
        message.clone()
    } else {
        match app.state {
            AppState::EmailList => format!("Email {} of {} | Press ? for help", 
                app.selected_email + 1, app.email_count()),
            AppState::EmailView => "Press Esc to go back | Press ? for help".to_string(),
            AppState::Search => "Type your search query and press Enter".to_string(),
            AppState::Help => "Press any key to close help".to_string(),
            AppState::Compose => "Compose mode".to_string(),
        }
    };

    let status = Paragraph::new(status_text)
        .style(Style::default().bg(Color::Blue).fg(Color::White));
    
    f.render_widget(status, area);
}

/// Helper function to create a centered rectangle
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}