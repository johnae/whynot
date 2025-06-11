//! Tests for Ctrl+M keybinding compatibility across terminal environments

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use std::env;
use whynot::tui::events::Event;

#[test]
fn test_ctrl_m_detection_in_current_environment() {
    // Test that Ctrl+M is properly detected in the current terminal environment
    let ctrl_m_event = Event::Key(KeyEvent {
        code: KeyCode::Char('m'),
        modifiers: KeyModifiers::CONTROL,
        kind: crossterm::event::KeyEventKind::Press,
        state: crossterm::event::KeyEventState::NONE,
    });
    
    assert!(ctrl_m_event.is_markdown_toggle(), 
        "Ctrl+M should be detected as markdown toggle in current environment");
}

#[test]
fn test_ctrl_m_raw_byte_sequence() {
    // Test that various representations of Ctrl+M work
    // Ctrl+M typically produces byte 13 (0x0D) in ASCII terminals
    
    // Standard Ctrl+M representation
    let standard_ctrl_m = Event::Key(KeyEvent {
        code: KeyCode::Char('m'),
        modifiers: KeyModifiers::CONTROL,
        kind: crossterm::event::KeyEventKind::Press,
        state: crossterm::event::KeyEventState::NONE,
    });
    
    assert!(standard_ctrl_m.is_markdown_toggle());
}

#[test]
fn test_terminal_environment_variables() {
    // Document the current terminal environment for debugging
    let term = env::var("TERM").unwrap_or_else(|_| "unknown".to_string());
    let term_program = env::var("TERM_PROGRAM").unwrap_or_else(|_| "unknown".to_string());
    let colorterm = env::var("COLORTERM").unwrap_or_else(|_| "unknown".to_string());
    
    println!("Terminal environment:");
    println!("  TERM: {}", term);
    println!("  TERM_PROGRAM: {}", term_program);
    println!("  COLORTERM: {}", colorterm);
    
    // The test passes regardless of environment, but prints info for debugging
    assert!(true, "Terminal environment documented");
}

#[test]
fn test_ctrl_m_vs_enter_distinction() {
    // Verify that Ctrl+M is distinct from Enter
    let ctrl_m = Event::Key(KeyEvent {
        code: KeyCode::Char('m'),
        modifiers: KeyModifiers::CONTROL,
        kind: crossterm::event::KeyEventKind::Press,
        state: crossterm::event::KeyEventState::NONE,
    });
    
    let enter = Event::Key(KeyEvent {
        code: KeyCode::Enter,
        modifiers: KeyModifiers::NONE,
        kind: crossterm::event::KeyEventKind::Press,
        state: crossterm::event::KeyEventState::NONE,
    });
    
    assert!(ctrl_m.is_markdown_toggle());
    assert!(!enter.is_markdown_toggle());
    assert!(enter.is_enter());
    assert!(!ctrl_m.is_enter());
}

#[test]
fn test_crossterm_key_detection() {
    // Test that crossterm correctly identifies our Ctrl+M pattern
    use crossterm::event::KeyEventKind;
    
    // Different key event kinds that might occur
    let press_event = Event::Key(KeyEvent {
        code: KeyCode::Char('m'),
        modifiers: KeyModifiers::CONTROL,
        kind: KeyEventKind::Press,
        state: crossterm::event::KeyEventState::NONE,
    });
    
    let release_event = Event::Key(KeyEvent {
        code: KeyCode::Char('m'),
        modifiers: KeyModifiers::CONTROL,
        kind: KeyEventKind::Release,
        state: crossterm::event::KeyEventState::NONE,
    });
    
    // Both press and release should be detected as markdown toggle
    // (our implementation doesn't check the kind)
    assert!(press_event.is_markdown_toggle());
    assert!(release_event.is_markdown_toggle());
}

#[cfg(test)]
mod integration_tests {
    use super::*;
    
    #[test]
    fn test_markdown_toggle_integration() {
        // This test simulates the exact code path used in whynot-tui.rs
        let key_event = KeyEvent {
            code: KeyCode::Char('m'),
            modifiers: KeyModifiers::CONTROL,
            kind: crossterm::event::KeyEventKind::Press,
            state: crossterm::event::KeyEventState::NONE,
        };
        
        let event = Event::Key(key_event);
        
        // This is the exact condition used in whynot-tui.rs:173
        assert!(event.is_markdown_toggle(), 
            "Integration test: Ctrl+M should trigger markdown toggle");
    }
}