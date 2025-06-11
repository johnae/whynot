//! Tests for the TUI compose mode indicator functionality
//!
//! These tests ensure that the compose UI clearly shows whether the user is in
//! markdown or plain text mode, and that the visual indicators update correctly.

use ratatui::{
    backend::TestBackend,
    Terminal,
};
use std::sync::Arc;
use whynot::{
    client::{NotmuchClient, TagOperation},
    config::Config,
    error::NotmuchError,
    mail_sender::{ComposableMessage, MailSender},
    thread::Message,
    tui::{app::App, ui},
};

// Simple mock client for testing
struct MockNotmuchClient;

#[async_trait::async_trait]
impl NotmuchClient for MockNotmuchClient {
    async fn search(&self, _query: &str) -> Result<Vec<whynot::search::SearchItem>, NotmuchError> {
        Ok(vec![])
    }

    async fn search_paginated(
        &self,
        _query: &str,
        _offset: usize,
        _limit: usize,
    ) -> Result<(Vec<whynot::search::SearchItem>, Option<usize>), NotmuchError> {
        Ok((vec![], Some(0)))
    }

    async fn show(&self, _query: &str) -> Result<whynot::thread::Thread, NotmuchError> {
        Err(NotmuchError::CommandFailed("Mock client".to_string()))
    }

    async fn tag(&self, _query: &str, _tags: &[TagOperation]) -> Result<(), NotmuchError> {
        Ok(())
    }

    async fn refresh(&self) -> Result<(), NotmuchError> {
        Ok(())
    }

    async fn insert(&self, _message: &[u8], _folder: Option<&str>, _tags: &[&str]) -> Result<String, NotmuchError> {
        Ok("test-message-id".to_string())
    }

    async fn config_get(&self, _key: &str) -> Result<String, NotmuchError> {
        Ok("test".to_string())
    }

    async fn config_set(&self, _key: &str, _value: &str) -> Result<(), NotmuchError> {
        Ok(())
    }

    async fn list_tags(&self) -> Result<Vec<String>, NotmuchError> {
        Ok(vec![])
    }

    async fn part(&self, _message_id: &str, _part_id: u32) -> Result<Vec<u8>, NotmuchError> {
        Ok(vec![])
    }
}

// Simple mock mail sender for testing
struct MockMailSender;

#[async_trait::async_trait]
impl MailSender for MockMailSender {
    async fn send(&self, _message: ComposableMessage) -> Result<String, whynot::error::Error> {
        Ok("test-message-id".to_string())
    }

    async fn reply(&self, _original: &Message, _reply: ComposableMessage, _reply_all: bool) -> Result<String, whynot::error::Error> {
        Ok("test-reply-id".to_string())
    }

    async fn forward(&self, _original: &Message, _forward: ComposableMessage) -> Result<String, whynot::error::Error> {
        Ok("test-forward-id".to_string())
    }

    async fn test_connection(&self) -> Result<(), whynot::error::Error> {
        Ok(())
    }

    async fn get_from_address(&self) -> Result<String, whynot::error::Error> {
        Ok("test@example.com".to_string())
    }
}

#[tokio::test]
async fn test_compose_mode_indicator_shows_plain_by_default() {
    // The compose UI should show [Plain] mode by default (when markdown_compose = false)
    let mut config = Config::default();
    config.ui.tui.markdown_compose = Some(false);

    let client = Arc::new(MockNotmuchClient) as Arc<dyn whynot::client::NotmuchClient>;
    let mail_sender = Some(Box::new(MockMailSender) as Box<dyn MailSender>);
    let mut app = App::new(client, mail_sender, &config).await.unwrap();
    
    // Start compose mode
    app.start_compose_new();
    
    // Render the UI to capture the displayed text
    let backend = TestBackend::new(80, 24);
    let mut terminal = Terminal::new(backend).unwrap();
    
    terminal.draw(|f| ui::draw(f, &mut app)).unwrap();
    let buffer = terminal.backend().buffer();
    
    // Convert buffer to string for analysis
    let rendered_text = buffer_to_string(buffer);
    
    // Should show [Plain] mode indicator in body field title
    assert!(rendered_text.contains("Body [Plain]") || rendered_text.contains("[Plain]"), 
           "Should show [Plain] mode indicator. Rendered: {}", rendered_text);
    
    // Should NOT show [Markdown] indicator
    assert!(!rendered_text.contains("[Markdown]"), 
           "Should not show [Markdown] indicator in plain mode. Rendered: {}", rendered_text);
}

#[tokio::test]
async fn test_compose_mode_indicator_shows_markdown_when_enabled() {
    // The compose UI should show [Markdown] mode when markdown_compose = true
    let mut config = Config::default();
    config.ui.tui.markdown_compose = Some(true);

    let client = Arc::new(MockNotmuchClient) as Arc<dyn whynot::client::NotmuchClient>;
    let mail_sender = Some(Box::new(MockMailSender) as Box<dyn MailSender>);
    let mut app = App::new(client, mail_sender, &config).await.unwrap();
    
    // Start compose mode (should start in markdown mode due to config)
    app.start_compose_new();
    
    // Render the UI
    let backend = TestBackend::new(80, 24);
    let mut terminal = Terminal::new(backend).unwrap();
    
    terminal.draw(|f| ui::draw(f, &mut app)).unwrap();
    let buffer = terminal.backend().buffer();
    let rendered_text = buffer_to_string(buffer);
    
    // Should show [Markdown] mode indicator
    assert!(rendered_text.contains("Body [Markdown]") || rendered_text.contains("[Markdown]"), 
           "Should show [Markdown] mode indicator. Rendered: {}", rendered_text);
    
    // Should NOT show [Plain] indicator
    assert!(!rendered_text.contains("[Plain]"), 
           "Should not show [Plain] indicator in markdown mode. Rendered: {}", rendered_text);
}

#[tokio::test]
async fn test_mode_indicator_updates_when_toggled() {
    // The mode indicator should change when user toggles with Ctrl+M
    let mut config = Config::default();
    config.ui.tui.markdown_compose = Some(false); // Start in plain mode

    let client = Arc::new(MockNotmuchClient) as Arc<dyn whynot::client::NotmuchClient>;
    let mail_sender = Some(Box::new(MockMailSender) as Box<dyn MailSender>);
    let mut app = App::new(client, mail_sender, &config).await.unwrap();
    
    app.start_compose_new();
    
    // Initial state: should show [Plain]
    let backend = TestBackend::new(80, 24);
    let mut terminal = Terminal::new(backend).unwrap();
    terminal.draw(|f| ui::draw(f, &mut app)).unwrap();
    let initial_text = buffer_to_string(terminal.backend().buffer());
    
    assert!(initial_text.contains("[Plain]") || initial_text.contains("Body [Plain]"),
           "Should initially show [Plain] mode. Rendered: {}", initial_text);
    
    // Toggle to markdown mode
    app.toggle_compose_markdown_mode();
    
    // After toggle: should show [Markdown]
    let backend2 = TestBackend::new(80, 24);
    let mut terminal2 = Terminal::new(backend2).unwrap();
    terminal2.draw(|f| ui::draw(f, &mut app)).unwrap();
    let toggled_text = buffer_to_string(terminal2.backend().buffer());
    
    assert!(toggled_text.contains("[Markdown]") || toggled_text.contains("Body [Markdown]"),
           "Should show [Markdown] mode after toggle. Rendered: {}", toggled_text);
    
    // Toggle back to plain mode
    app.toggle_compose_markdown_mode();
    
    // After second toggle: should show [Plain] again
    let backend3 = TestBackend::new(80, 24);
    let mut terminal3 = Terminal::new(backend3).unwrap();
    terminal3.draw(|f| ui::draw(f, &mut app)).unwrap();
    let final_text = buffer_to_string(terminal3.backend().buffer());
    
    assert!(final_text.contains("[Plain]") || final_text.contains("Body [Plain]"),
           "Should show [Plain] mode after second toggle. Rendered: {}", final_text);
}

#[tokio::test]
async fn test_compose_instructions_mention_ctrl_m_toggle() {
    // The compose instructions should mention Ctrl+M for toggling markdown mode
    let mut config = Config::default();
    config.ui.tui.markdown_compose = Some(false);

    let client = Arc::new(MockNotmuchClient) as Arc<dyn whynot::client::NotmuchClient>;
    let mail_sender = Some(Box::new(MockMailSender) as Box<dyn MailSender>);
    let mut app = App::new(client, mail_sender, &config).await.unwrap();
    
    app.start_compose_new();
    
    let backend = TestBackend::new(80, 24);
    let mut terminal = Terminal::new(backend).unwrap();
    terminal.draw(|f| ui::draw(f, &mut app)).unwrap();
    let rendered_text = buffer_to_string(terminal.backend().buffer());
    
    // Should mention Ctrl+M toggle in instructions
    assert!(rendered_text.contains("Ctrl+M") && (rendered_text.contains("markdown") || rendered_text.contains("mode")),
           "Should mention Ctrl+M toggle in instructions. Rendered: {}", rendered_text);
}

#[tokio::test]
async fn test_mode_indicator_persists_across_field_navigation() {
    // The mode indicator should remain visible when navigating between compose fields
    let mut config = Config::default();
    config.ui.tui.markdown_compose = Some(true); // Start in markdown mode

    let client = Arc::new(MockNotmuchClient) as Arc<dyn whynot::client::NotmuchClient>;
    let mail_sender = Some(Box::new(MockMailSender) as Box<dyn MailSender>);
    let mut app = App::new(client, mail_sender, &config).await.unwrap();
    
    app.start_compose_new();
    
    // Check indicator is visible in To field (initial field)
    let backend1 = TestBackend::new(80, 24);
    let mut terminal1 = Terminal::new(backend1).unwrap();
    terminal1.draw(|f| ui::draw(f, &mut app)).unwrap();
    let to_field_text = buffer_to_string(terminal1.backend().buffer());
    
    assert!(to_field_text.contains("[Markdown]"),
           "Should show [Markdown] indicator when in To field. Rendered: {}", to_field_text);
    
    // Navigate to Body field
    app.compose_next_field(); // To -> Cc
    app.compose_next_field(); // Cc -> Bcc  
    app.compose_next_field(); // Bcc -> Subject
    app.compose_next_field(); // Subject -> Body
    
    // Check indicator is still visible in Body field
    let backend2 = TestBackend::new(80, 24);
    let mut terminal2 = Terminal::new(backend2).unwrap();
    terminal2.draw(|f| ui::draw(f, &mut app)).unwrap();
    let body_field_text = buffer_to_string(terminal2.backend().buffer());
    
    assert!(body_field_text.contains("[Markdown]"),
           "Should show [Markdown] indicator when in Body field. Rendered: {}", body_field_text);
}

#[tokio::test] 
async fn test_different_compose_modes_show_indicator() {
    // Mode indicator should work consistently in different compose scenarios
    let mut config = Config::default();
    config.ui.tui.markdown_compose = Some(false);

    let client = Arc::new(MockNotmuchClient) as Arc<dyn whynot::client::NotmuchClient>;
    let mail_sender = Some(Box::new(MockMailSender) as Box<dyn MailSender>);
    let mut app = App::new(client, mail_sender, &config).await.unwrap();
    
    // Test New mode with markdown enabled
    app.start_compose_new();
    app.toggle_compose_markdown_mode(); // Enable markdown
    
    let backend1 = TestBackend::new(80, 24);
    let mut terminal1 = Terminal::new(backend1).unwrap();
    terminal1.draw(|f| ui::draw(f, &mut app)).unwrap();
    let new_text = buffer_to_string(terminal1.backend().buffer());
    
    assert!(new_text.contains("[Markdown]"),
           "Should show [Markdown] indicator in New mode. Rendered: {}", new_text);
    
    // Test that starting a new compose resets to default mode
    app.start_compose_new();
    // Should reset to config default (false in this test)
    
    let backend2 = TestBackend::new(80, 24);
    let mut terminal2 = Terminal::new(backend2).unwrap();
    terminal2.draw(|f| ui::draw(f, &mut app)).unwrap();
    let second_text = buffer_to_string(terminal2.backend().buffer());
    
    assert!(second_text.contains("[Plain]"),
           "Should reset to [Plain] mode in new compose sessions. Rendered: {}", second_text);
}

/// Helper function to convert terminal buffer to string for text analysis
fn buffer_to_string(buffer: &ratatui::buffer::Buffer) -> String {
    let mut result = String::new();
    let area = buffer.area();
    
    for y in 0..area.height {
        for x in 0..area.width {
            let cell = &buffer[(x, y)];
            result.push_str(&cell.symbol());
        }
        result.push('\n');
    }
    
    result
}