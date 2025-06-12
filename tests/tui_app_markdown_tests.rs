//! Tests for TUI App markdown toggle functionality

use whynot::tui::app::{AppState, ComposeForm, ComposeMode};

#[test]
fn test_compose_form_markdown_mode_field_exists() {
    let mut form = ComposeForm::default();
    
    // Should default to false
    assert_eq!(form.markdown_mode, false);
    
    // Should be able to toggle
    form.markdown_mode = true;
    assert_eq!(form.markdown_mode, true);
}

#[test]
fn test_compose_form_with_explicit_markdown_mode() {
    let form = ComposeForm {
        mode: ComposeMode::New,
        markdown_mode: true,
        to: "test@example.com".to_string(),
        subject: "Test".to_string(),
        body: "# Hello World\n\nThis is **markdown**.".to_string(),
        ..Default::default()
    };
    
    assert_eq!(form.markdown_mode, true);
    assert_eq!(form.mode, ComposeMode::New);
    assert_eq!(form.to, "test@example.com");
    assert!(form.body.contains("# Hello World"));
}

#[test]
fn test_app_state_compare() {
    // Test that AppState comparison works for our tests
    assert_eq!(AppState::Compose, AppState::Compose);
    assert_eq!(AppState::EmailList, AppState::EmailList);
    assert_ne!(AppState::Compose, AppState::EmailList);
}

#[test]
fn test_compose_mode_compare() {
    // Test that ComposeMode comparison works for our tests
    assert_eq!(ComposeMode::New, ComposeMode::New);
    assert_eq!(ComposeMode::Reply("id1".to_string()), ComposeMode::Reply("id1".to_string()));
    assert_ne!(ComposeMode::New, ComposeMode::Reply("id".to_string()));
}