//! Tests for TUI markdown keybinding functionality

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use whynot::tui::events::Event;

#[test]
fn test_ctrl_m_is_markdown_toggle() {
    let event = Event::Key(KeyEvent {
        code: KeyCode::Char('m'),
        modifiers: KeyModifiers::CONTROL,
        kind: crossterm::event::KeyEventKind::Press,
        state: crossterm::event::KeyEventState::NONE,
    });
    
    assert!(event.is_markdown_toggle());
}

#[test]
fn test_regular_m_is_not_markdown_toggle() {
    let event = Event::Key(KeyEvent {
        code: KeyCode::Char('m'),
        modifiers: KeyModifiers::NONE,
        kind: crossterm::event::KeyEventKind::Press,
        state: crossterm::event::KeyEventState::NONE,
    });
    
    assert!(!event.is_markdown_toggle());
}

#[test]
fn test_shift_m_is_not_markdown_toggle() {
    let event = Event::Key(KeyEvent {
        code: KeyCode::Char('M'),
        modifiers: KeyModifiers::SHIFT,
        kind: crossterm::event::KeyEventKind::Press,
        state: crossterm::event::KeyEventState::NONE,
    });
    
    assert!(!event.is_markdown_toggle());
}

#[test]
fn test_other_ctrl_keys_not_markdown_toggle() {
    let events = vec![
        Event::Key(KeyEvent {
            code: KeyCode::Char('n'),
            modifiers: KeyModifiers::CONTROL,
            kind: crossterm::event::KeyEventKind::Press,
            state: crossterm::event::KeyEventState::NONE,
        }),
        Event::Key(KeyEvent {
            code: KeyCode::Char('s'),
            modifiers: KeyModifiers::CONTROL,
            kind: crossterm::event::KeyEventKind::Press,
            state: crossterm::event::KeyEventState::NONE,
        }),
        Event::Key(KeyEvent {
            code: KeyCode::Char('c'),
            modifiers: KeyModifiers::CONTROL,
            kind: crossterm::event::KeyEventKind::Press,
            state: crossterm::event::KeyEventState::NONE,
        }),
    ];
    
    for event in events {
        assert!(!event.is_markdown_toggle());
    }
}