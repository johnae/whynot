//! Basic integration tests for TUI compose functionality

use whynot::tui::app::{ComposeField, ComposeForm, ComposeMode};

#[test]
fn test_compose_form_defaults() {
    let form = ComposeForm::default();

    assert!(matches!(form.mode, ComposeMode::New));
    assert!(matches!(form.current_field, ComposeField::To));
    assert_eq!(form.to, "");
    assert_eq!(form.cc, "");
    assert_eq!(form.bcc, "");
    assert_eq!(form.subject, "");
    assert_eq!(form.body, "");
}

#[test]
fn test_compose_field_enum() {
    // Test that we can pattern match on compose fields
    let field = ComposeField::To;
    match field {
        ComposeField::To => assert!(true),
        _ => assert!(false, "Expected To field"),
    }

    let field = ComposeField::Body;
    match field {
        ComposeField::Body => assert!(true),
        _ => assert!(false, "Expected Body field"),
    }
}

#[test]
fn test_compose_mode_enum() {
    // Test that we can create different compose modes
    let mode = ComposeMode::New;
    assert!(matches!(mode, ComposeMode::New));

    let mode = ComposeMode::Reply("test-id".to_string());
    assert!(matches!(mode, ComposeMode::Reply(_)));

    let mode = ComposeMode::ReplyAll("test-id".to_string());
    assert!(matches!(mode, ComposeMode::ReplyAll(_)));

    let mode = ComposeMode::Forward("test-id".to_string());
    assert!(matches!(mode, ComposeMode::Forward(_)));
}
