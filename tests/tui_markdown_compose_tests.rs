//! Tests for TUI markdown compose functionality

use whynot::tui::app::{ComposeForm, ComposeMode};

#[test]
fn test_compose_form_has_markdown_mode_field() {
    let mut form = ComposeForm::default();
    
    // Should start with markdown_mode as false by default
    assert_eq!(form.markdown_mode, false);
    
    // Should be able to toggle markdown_mode
    form.markdown_mode = true;
    assert_eq!(form.markdown_mode, true);
    
    form.markdown_mode = false;
    assert_eq!(form.markdown_mode, false);
}

#[test]
fn test_compose_form_markdown_mode_with_different_modes() {
    // Test that markdown_mode works with all compose modes
    let new_form = ComposeForm {
        mode: ComposeMode::New,
        markdown_mode: true,
        ..Default::default()
    };
    assert_eq!(new_form.markdown_mode, true);
    
    let reply_form = ComposeForm {
        mode: ComposeMode::Reply("test-id".to_string()),
        markdown_mode: false,
        ..Default::default()
    };
    assert_eq!(reply_form.markdown_mode, false);
    
    let reply_all_form = ComposeForm {
        mode: ComposeMode::ReplyAll("test-id".to_string()),
        markdown_mode: true,
        ..Default::default()
    };
    assert_eq!(reply_all_form.markdown_mode, true);
    
    let forward_form = ComposeForm {
        mode: ComposeMode::Forward("test-id".to_string()),
        markdown_mode: false,
        ..Default::default()
    };
    assert_eq!(forward_form.markdown_mode, false);
}

#[test]
fn test_compose_form_preserves_other_fields_with_markdown() {
    let mut form = ComposeForm {
        to: "test@example.com".to_string(),
        cc: "cc@example.com".to_string(),
        subject: "Test Subject".to_string(),
        body: "Test body content".to_string(),
        markdown_mode: true,
        ..Default::default()
    };
    
    // Verify all fields are preserved when markdown_mode is set
    assert_eq!(form.to, "test@example.com");
    assert_eq!(form.cc, "cc@example.com");
    assert_eq!(form.subject, "Test Subject");
    assert_eq!(form.body, "Test body content");
    assert_eq!(form.markdown_mode, true);
    
    // Toggle markdown_mode and verify other fields are unchanged
    form.markdown_mode = false;
    assert_eq!(form.to, "test@example.com");
    assert_eq!(form.cc, "cc@example.com");
    assert_eq!(form.subject, "Test Subject");
    assert_eq!(form.body, "Test body content");
    assert_eq!(form.markdown_mode, false);
}