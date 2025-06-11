use clap::Parser;
use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::{io, sync::Arc, time::Duration};
use whynot::{
    client::create_client,
    config::{Config, CliArgs},
    mail_sender::create_mail_sender,
    tui::{app::App, events::EventHandler, ui},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Parse CLI arguments and load configuration
    let cli_args = CliArgs::parse();
    let config = Config::load(cli_args)?;
    
    // Create client configuration from unified config
    let client_config = config.to_client_config()?;
    
    // Create the notmuch client
    let client = create_client(client_config)?;
    let client = Arc::from(client) as Arc<dyn whynot::client::NotmuchClient>;
    
    // Create the mail sender (optional if not configured)
    let mail_sender = match config.to_mail_sender_config() {
        Ok(mail_sender_config) => {
            match create_mail_sender(mail_sender_config) {
                Ok(sender) => Some(sender),
                Err(e) => {
                    eprintln!("Warning: Mail sending not configured: {}", e);
                    None
                }
            }
        }
        Err(e) => {
            eprintln!("Warning: Mail sending configuration incomplete: {}", e);
            None
        }
    };
    
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app and event handler
    let mut app = App::new(client, mail_sender).await?;
    let event_handler = EventHandler::new(Duration::from_millis(250));

    // Initialize the app
    app.initialize().await?;

    // Main application loop
    let result = run_app(&mut terminal, &mut app, &event_handler).await;

    // Restore terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    if let Err(err) = result {
        eprintln!("Application error: {}", err);
    }

    Ok(())
}

async fn run_app<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    app: &mut App,
    event_handler: &EventHandler,
) -> Result<(), Box<dyn std::error::Error>> {
    loop {
        // Draw the UI
        terminal.draw(|f| ui::draw(f, app))?;

        // Handle events
        match event_handler.next()? {
            whynot::tui::Event::Key(key) => {
                let event = whynot::tui::Event::Key(key);
                
                // Global quit handling
                if event.is_quit() {
                    app.quit();
                    break;
                }

                // State-specific event handling
                match app.state {
                    whynot::tui::app::AppState::EmailList => {
                        if event.is_up() {
                            app.navigate_up();
                        } else if event.is_down() {
                            app.navigate_down();
                        } else if event.is_enter() {
                            if let Err(e) = app.open_selected_email().await {
                                app.set_status(format!("Error opening email: {}", e));
                            }
                        } else if event.is_search() {
                            app.enter_search_mode();
                        } else if event.is_compose() {
                            app.start_compose_new();
                        } else if event.is_help() {
                            app.show_help();
                        }
                    }
                    whynot::tui::app::AppState::EmailView => {
                        if event.is_back() {
                            app.go_back();
                        } else if event.is_up() {
                            app.scroll_up();
                        } else if event.is_down() {
                            app.scroll_down();
                        } else if event.is_page_up() {
                            app.page_up();
                        } else if event.is_page_down() {
                            app.page_down();
                        } else if event.is_top() {
                            app.scroll_to_top();
                        } else if event.is_bottom() {
                            // For now, use a reasonable estimate for content height
                            app.scroll_to_bottom(1000);
                        } else if event.is_next_message() {
                            if let Err(e) = app.next_message_in_thread().await {
                                app.set_status(format!("Error navigating to next message: {}", e));
                            }
                        } else if event.is_prev_message() {
                            if let Err(e) = app.prev_message_in_thread().await {
                                app.set_status(format!("Error navigating to previous message: {}", e));
                            }
                        } else if event.is_reply() {
                            app.start_compose_reply(false);
                        } else if event.is_reply_all() {
                            app.start_compose_reply(true);
                        } else if event.is_forward() {
                            app.start_compose_forward();
                        } else if event.is_help() {
                            app.show_help();
                        }
                    }
                    whynot::tui::app::AppState::Help => {
                        // Any key closes help
                        app.go_back();
                    }
                    whynot::tui::app::AppState::Search => {
                        match key.code {
                            crossterm::event::KeyCode::Esc => {
                                app.go_back();
                            }
                            crossterm::event::KeyCode::Enter => {
                                if let Err(e) = app.execute_search().await {
                                    app.set_status(format!("Search error: {}", e));
                                } else {
                                    app.set_status(format!("Search results for: {}", app.search_query));
                                }
                            }
                            crossterm::event::KeyCode::Backspace => {
                                app.handle_search_backspace();
                            }
                            crossterm::event::KeyCode::Char(c) => {
                                app.handle_search_char(c);
                            }
                            _ => {}
                        }
                    }
                    whynot::tui::app::AppState::Compose => {
                        match key.code {
                            crossterm::event::KeyCode::Esc => {
                                app.go_back();
                            }
                            crossterm::event::KeyCode::Tab => {
                                app.compose_next_field();
                            }
                            crossterm::event::KeyCode::BackTab => {
                                app.compose_prev_field();
                            }
                            crossterm::event::KeyCode::Enter => {
                                // Handle Enter key in compose mode
                                app.compose_handle_enter();
                            }
                            crossterm::event::KeyCode::Char('s') if key.modifiers.contains(crossterm::event::KeyModifiers::CONTROL) => {
                                if let Err(e) = app.send_composed_email().await {
                                    app.set_status(format!("Send error: {}", e));
                                }
                            }
                            crossterm::event::KeyCode::Backspace => {
                                app.compose_handle_backspace();
                            }
                            crossterm::event::KeyCode::Char(c) => {
                                app.compose_handle_char(c);
                            }
                            _ => {}
                        }
                    }
                }
            }
            whynot::tui::Event::Resize(_, _) => {
                // Terminal was resized, redraw will happen on next loop
            }
            whynot::tui::Event::Tick => {
                // Clear status message after some time
                if app.status_message.is_some() {
                    // TODO: Implement timed status clearing
                }
            }
        }

        if app.should_quit {
            break;
        }
    }

    Ok(())
}