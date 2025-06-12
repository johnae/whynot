use whynot::{
    client::{NotmuchClient, TagOperation},
    config::Config,
    error::NotmuchError,
    mail_sender::{ComposableMessage, MailSender},
    thread::Message,
    tui::app::{App, AppState, ComposeField},
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
        Err(NotmuchError::CommandFailed("Mock client".to_string()))
    }

    async fn refresh(&self) -> Result<(), NotmuchError> {
        Err(NotmuchError::CommandFailed("Mock client".to_string()))
    }

    async fn insert(
        &self,
        _message: &[u8],
        _folder: Option<&str>,
        _tags: &[&str],
    ) -> Result<String, NotmuchError> {
        Err(NotmuchError::CommandFailed("Mock client".to_string()))
    }

    async fn config_get(&self, _key: &str) -> Result<String, NotmuchError> {
        Err(NotmuchError::CommandFailed("Mock client".to_string()))
    }

    async fn config_set(&self, _key: &str, _value: &str) -> Result<(), NotmuchError> {
        Err(NotmuchError::CommandFailed("Mock client".to_string()))
    }

    async fn list_tags(&self) -> Result<Vec<String>, NotmuchError> {
        Ok(vec!["inbox".to_string(), "unread".to_string()])
    }

    async fn part(&self, _message_id: &str, _part_id: u32) -> Result<Vec<u8>, NotmuchError> {
        Err(NotmuchError::CommandFailed("Mock client".to_string()))
    }
}

// Simple mock mail sender for testing
struct MockMailSender;

#[async_trait::async_trait]
impl MailSender for MockMailSender {
    async fn send(&self, _message: ComposableMessage) -> Result<String, NotmuchError> {
        Ok("mock-message-id".to_string())
    }

    async fn reply(
        &self,
        _original: &Message,
        _reply: ComposableMessage,
        _reply_all: bool,
    ) -> Result<String, NotmuchError> {
        Ok("mock-reply-id".to_string())
    }

    async fn forward(
        &self,
        _original: &Message,
        _forward: ComposableMessage,
    ) -> Result<String, NotmuchError> {
        Ok("mock-forward-id".to_string())
    }

    async fn test_connection(&self) -> Result<(), NotmuchError> {
        Ok(())
    }

    async fn get_from_address(&self) -> Result<String, NotmuchError> {
        Ok("test@example.com".to_string())
    }
}

#[tokio::test]
async fn test_compose_multiline_body_input() {
    let client = std::sync::Arc::new(MockNotmuchClient);
    let mail_sender = Some(Box::new(MockMailSender) as Box<dyn MailSender>);
    let config = Config::default();
    let mut app = App::new(client as std::sync::Arc<dyn NotmuchClient>, mail_sender, &config)
        .await
        .unwrap();

    // Start composing a new email
    app.start_compose_new();
    assert!(matches!(app.state, AppState::Compose));

    // Navigate to the body field
    app.compose_form.current_field = ComposeField::Body;

    // Type some text and add newlines
    app.compose_handle_char('H');
    app.compose_handle_char('e');
    app.compose_handle_char('l');
    app.compose_handle_char('l');
    app.compose_handle_char('o');

    // Add a newline using Enter
    app.compose_handle_enter();

    app.compose_handle_char('W');
    app.compose_handle_char('o');
    app.compose_handle_char('r');
    app.compose_handle_char('l');
    app.compose_handle_char('d');

    // Add another newline
    app.compose_handle_enter();
    app.compose_handle_enter(); // Empty line

    app.compose_handle_char('B');
    app.compose_handle_char('y');
    app.compose_handle_char('e');

    // Check that the body contains newlines
    let expected_body = "Hello\nWorld\n\nBye";
    assert_eq!(app.compose_form.body, expected_body);

    // Verify that newlines are properly stored
    assert!(app.compose_form.body.contains('\n'));
    let lines: Vec<&str> = app.compose_form.body.lines().collect();
    assert_eq!(lines.len(), 4); // "Hello", "World", "", "Bye"
    assert_eq!(lines[0], "Hello");
    assert_eq!(lines[1], "World");
    assert_eq!(lines[2], "");
    assert_eq!(lines[3], "Bye");
}

#[tokio::test]
async fn test_compose_enter_behavior_in_non_body_fields() {
    let client = std::sync::Arc::new(MockNotmuchClient);
    let mail_sender = Some(Box::new(MockMailSender) as Box<dyn MailSender>);
    let config = Config::default();
    let mut app = App::new(client as std::sync::Arc<dyn NotmuchClient>, mail_sender, &config)
        .await
        .unwrap();

    // Start composing a new email
    app.start_compose_new();
    assert!(matches!(app.state, AppState::Compose));

    // Ensure we start in the To field
    assert!(matches!(app.compose_form.current_field, ComposeField::To));

    // Add some text to the To field
    app.compose_handle_char('t');
    app.compose_handle_char('e');
    app.compose_handle_char('s');
    app.compose_handle_char('t');

    // Press Enter - should move to next field (Cc), not add newline
    app.compose_handle_enter();
    assert!(matches!(app.compose_form.current_field, ComposeField::Cc));

    // Verify no newline was added to the To field
    assert_eq!(app.compose_form.to, "test");
    assert!(!app.compose_form.to.contains('\n'));

    // Test in Subject field
    app.compose_form.current_field = ComposeField::Subject;
    app.compose_handle_char('T');
    app.compose_handle_char('e');
    app.compose_handle_char('s');
    app.compose_handle_char('t');

    // Press Enter - should move to Body field
    app.compose_handle_enter();
    assert!(matches!(app.compose_form.current_field, ComposeField::Body));

    // Verify no newline was added to Subject
    assert_eq!(app.compose_form.subject, "Test");
    assert!(!app.compose_form.subject.contains('\n'));
}

#[tokio::test]
async fn test_compose_backspace_with_multiline_content() {
    let client = std::sync::Arc::new(MockNotmuchClient);
    let mail_sender = Some(Box::new(MockMailSender) as Box<dyn MailSender>);
    let config = Config::default();
    let mut app = App::new(client as std::sync::Arc<dyn NotmuchClient>, mail_sender, &config)
        .await
        .unwrap();

    // Start composing and navigate to body
    app.start_compose_new();
    app.compose_form.current_field = ComposeField::Body;

    // Add some multiline content
    app.compose_handle_char('L');
    app.compose_handle_char('i');
    app.compose_handle_char('n');
    app.compose_handle_char('e');
    app.compose_handle_char(' ');
    app.compose_handle_char('1');
    app.compose_handle_enter();
    app.compose_handle_char('L');
    app.compose_handle_char('i');
    app.compose_handle_char('n');
    app.compose_handle_char('e');
    app.compose_handle_char(' ');
    app.compose_handle_char('2');

    assert_eq!(app.compose_form.body, "Line 1\nLine 2");

    // Test backspace - should remove last character
    app.compose_handle_backspace();
    assert_eq!(app.compose_form.body, "Line 1\nLine ");

    // Remove more characters including the newline
    for _ in 0..5 {
        // Remove "Line "
        app.compose_handle_backspace();
    }
    assert_eq!(app.compose_form.body, "Line 1\n");

    // Remove the newline
    app.compose_handle_backspace();
    assert_eq!(app.compose_form.body, "Line 1");

    // Verify we can still add content
    app.compose_handle_char('!');
    assert_eq!(app.compose_form.body, "Line 1!");
}
