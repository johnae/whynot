//! Interactive test for Ctrl+M keybinding functionality
//! This test focuses on the event detection and basic functionality testing.

#[cfg(test)]
mod interactive_tests {
    use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
    use whynot::{
        tui::{app::{ComposeForm, ComposeMode}, events::Event}
    };

    #[test]
    fn test_compose_form_markdown_toggle() {
        // Test the basic ComposeForm markdown toggle functionality
        let mut form = ComposeForm {
            mode: ComposeMode::New,
            to: String::new(),
            cc: String::new(),
            bcc: String::new(),
            subject: String::new(),
            body: String::new(),
            current_field: whynot::tui::app::ComposeField::Body,
            markdown_mode: false, // Start disabled
        };
        
        // Initially disabled
        assert!(!form.markdown_mode);
        
        // Toggle on
        form.markdown_mode = !form.markdown_mode;
        assert!(form.markdown_mode);
        
        // Toggle off
        form.markdown_mode = !form.markdown_mode;
        assert!(!form.markdown_mode);
    }

    #[test]
    fn test_ctrl_m_event_detection_comprehensive() {
        // Test various key combinations to ensure only Ctrl+M is detected
        let test_cases = vec![
            // Should match Ctrl+M
            (KeyCode::Char('m'), KeyModifiers::CONTROL, true),
            
            // Should NOT match
            (KeyCode::Char('m'), KeyModifiers::NONE, false),
            (KeyCode::Char('m'), KeyModifiers::SHIFT, false),
            (KeyCode::Char('m'), KeyModifiers::ALT, false),
            (KeyCode::Char('M'), KeyModifiers::CONTROL, false), // Capital M
            (KeyCode::Char('n'), KeyModifiers::CONTROL, false), // Ctrl+N
            (KeyCode::Enter, KeyModifiers::NONE, false),        // Enter
            (KeyCode::Enter, KeyModifiers::CONTROL, false),     // Ctrl+Enter
        ];
        
        for (key_code, modifiers, should_match) in test_cases {
            let event = Event::Key(KeyEvent {
                code: key_code,
                modifiers,
                kind: crossterm::event::KeyEventKind::Press,
                state: crossterm::event::KeyEventState::NONE,
            });
            
            assert_eq!(
                event.is_markdown_toggle(), 
                should_match,
                "Failed for {:?} with {:?}", key_code, modifiers
            );
        }
    }
}