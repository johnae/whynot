use crate::tui::app::{App, AppState};
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

    let title = if app.search_query == "tag:inbox" {
        format!("Inbox ({} emails)", app.email_count())
    } else {
        format!("Search: {} ({} results)", app.search_query, app.email_count())
    };

    let list = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(title)
        )
        .highlight_style(Style::default().bg(Color::Blue).fg(Color::White));

    f.render_stateful_widget(list, area, &mut state);
}

fn draw_email_view(f: &mut Frame, app: &mut App, area: Rect) {
    if app.current_email.is_some() {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(7), Constraint::Min(1)])
            .split(area);

        // Email headers
        draw_email_headers(f, app, chunks[0]);
        
        // Email body
        draw_email_body(f, app, chunks[1]);
    } else {
        let paragraph = Paragraph::new("No email selected")
            .block(Block::default().borders(Borders::ALL).title("Email View"));
        f.render_widget(paragraph, area);
    }
}

fn draw_email_headers(f: &mut Frame, app: &App, area: Rect) {
    let message = match &app.current_email {
        Some(msg) => msg,
        None => return,
    };
    let from = &message.headers.from;
    let to = &message.headers.to;
    let subject = &message.headers.subject;
    let date = chrono::DateTime::from_timestamp(message.timestamp, 0)
        .map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string())
        .unwrap_or_else(|| "Unknown".to_string());

    let mut headers_text = vec![
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

    // Add thread info if this is part of a multi-message thread
    if let Some(thread_info) = app.get_thread_info() {
        headers_text.push(Line::from(vec![
            Span::styled("Thread: ", Style::default().add_modifier(Modifier::BOLD)),
            Span::styled(thread_info, Style::default().fg(Color::Yellow)),
        ]));
    }

    let headers = Paragraph::new(headers_text)
        .block(Block::default().borders(Borders::ALL).title("Headers"))
        .wrap(Wrap { trim: true });

    f.render_widget(headers, area);
}

fn draw_email_body(f: &mut Frame, app: &App, area: Rect) {
    // Use the processed email body text if available
    let body_text = app.current_email_body.clone()
        .unwrap_or_else(|| "[No body content]".to_string());

    // Count total lines for scroll indicators
    let lines: Vec<&str> = body_text.lines().collect();
    let total_lines = lines.len();
    
    // Create title with scroll indicator
    let title = if total_lines > (area.height as usize).saturating_sub(2) {
        format!("Content (scroll: {}/{} lines)", 
               app.scroll_position + 1, 
               total_lines.max(1))
    } else {
        "Content".to_string()
    };

    let paragraph = Paragraph::new(body_text)
        .block(Block::default().borders(Borders::ALL).title(title))
        .wrap(Wrap { trim: true })
        .scroll((app.scroll_position as u16, 0));

    f.render_widget(paragraph, area);
}

fn draw_search(f: &mut Frame, app: &mut App, area: Rect) {
    // Create a centered modal for search input
    let modal_area = centered_rect(60, 20, area);
    
    // Clear the background
    f.render_widget(Clear, modal_area);
    
    // Draw the search input with cursor
    let input_text = format!("Search: {}_", app.search_input);
    let paragraph = Paragraph::new(input_text)
        .block(Block::default()
            .borders(Borders::ALL)
            .title("Search (Enter to search, Esc to cancel)")
            .style(Style::default().fg(Color::Yellow))
        )
        .wrap(Wrap { trim: false });
    
    f.render_widget(paragraph, modal_area);
}

fn draw_help(f: &mut Frame, _app: &mut App, area: Rect) {
    let help_text = vec![
        Line::from("Whynot TUI - Keyboard Shortcuts"),
        Line::from(""),
        Line::from("Navigation:"),
        Line::from("  j/↓     - Move down (email list) / Scroll down (email view)"),
        Line::from("  k/↑     - Move up (email list) / Scroll up (email view)"),
        Line::from("  PgUp/PgDn - Page up/down (email view)"),
        Line::from("  Home/G  - Go to top/bottom (email view)"),
        Line::from("  Enter   - Open selected email"),
        Line::from("  Esc     - Go back"),
        Line::from(""),
        Line::from("Actions:"),
        Line::from("  /       - Search"),
        Line::from("  c       - Compose (from email list)"),
        Line::from("  r       - Reply (from email view)"),
        Line::from("  R       - Reply all (from email view)"),
        Line::from("  f       - Forward (from email view)"),
        Line::from("  n/p     - Next/previous message in thread (email view)"),
        Line::from("  ?       - Show this help"),
        Line::from("  q       - Quit"),
        Line::from(""),
        Line::from("Compose Mode:"),
        Line::from("  Tab/Shift+Tab - Navigate fields"),
        Line::from("  Enter    - New line in body field"),
        Line::from("  Ctrl+S   - Send email"),
        Line::from("  Esc      - Cancel compose"),
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

fn draw_compose(f: &mut Frame, app: &mut App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // To field
            Constraint::Length(3), // Cc field  
            Constraint::Length(3), // Bcc field
            Constraint::Length(3), // Subject field
            Constraint::Min(1),    // Body field
            Constraint::Length(1), // Instructions
        ])
        .split(area);

    // Helper function to create field style based on whether it's selected
    let field_style = |field: &crate::tui::app::ComposeField| -> Style {
        if std::mem::discriminant(field) == std::mem::discriminant(&app.compose_form.current_field) {
            Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
        } else {
            Style::default()
        }
    };

    // To field
    let to_paragraph = Paragraph::new(format!("To: {}_", app.compose_form.to))
        .block(Block::default()
            .borders(Borders::ALL)
            .title("To")
            .border_style(field_style(&crate::tui::app::ComposeField::To))
        )
        .wrap(Wrap { trim: false });
    f.render_widget(to_paragraph, chunks[0]);

    // Cc field
    let cc_paragraph = Paragraph::new(format!("Cc: {}_", app.compose_form.cc))
        .block(Block::default()
            .borders(Borders::ALL)
            .title("Cc")
            .border_style(field_style(&crate::tui::app::ComposeField::Cc))
        )
        .wrap(Wrap { trim: false });
    f.render_widget(cc_paragraph, chunks[1]);

    // Bcc field
    let bcc_paragraph = Paragraph::new(format!("Bcc: {}_", app.compose_form.bcc))
        .block(Block::default()
            .borders(Borders::ALL)
            .title("Bcc")
            .border_style(field_style(&crate::tui::app::ComposeField::Bcc))
        )
        .wrap(Wrap { trim: false });
    f.render_widget(bcc_paragraph, chunks[2]);

    // Subject field
    let subject_paragraph = Paragraph::new(format!("Subject: {}_", app.compose_form.subject))
        .block(Block::default()
            .borders(Borders::ALL)
            .title("Subject")
            .border_style(field_style(&crate::tui::app::ComposeField::Subject))
        )
        .wrap(Wrap { trim: false });
    f.render_widget(subject_paragraph, chunks[3]);

    // Body field
    let body_text = if matches!(app.compose_form.current_field, crate::tui::app::ComposeField::Body) {
        format!("{}_", app.compose_form.body)
    } else {
        app.compose_form.body.clone()
    };
    
    let body_paragraph = Paragraph::new(body_text)
        .block(Block::default()
            .borders(Borders::ALL)
            .title("Body")
            .border_style(field_style(&crate::tui::app::ComposeField::Body))
        )
        .wrap(Wrap { trim: true });
    f.render_widget(body_paragraph, chunks[4]);

    // Instructions
    let instructions = match app.compose_form.mode {
        crate::tui::app::ComposeMode::New => "New Email - Tab/Shift+Tab: switch fields, Enter: newline in body, Ctrl+S: send, Esc: cancel",
        crate::tui::app::ComposeMode::Reply(_) => "Reply - Tab/Shift+Tab: switch fields, Enter: newline in body, Ctrl+S: send, Esc: cancel",
        crate::tui::app::ComposeMode::ReplyAll(_) => "Reply All - Tab/Shift+Tab: switch fields, Enter: newline in body, Ctrl+S: send, Esc: cancel",
        crate::tui::app::ComposeMode::Forward(_) => "Forward - Tab/Shift+Tab: switch fields, Enter: newline in body, Ctrl+S: send, Esc: cancel",
    };
    
    let instructions_paragraph = Paragraph::new(instructions)
        .style(Style::default().fg(Color::Gray));
    f.render_widget(instructions_paragraph, chunks[5]);
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