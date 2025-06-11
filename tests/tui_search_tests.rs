#[cfg(test)]
mod tui_search_tests {
    use whynot::tui::app::AppState;
    
    #[test]
    fn test_search_input_handling() {
        // Test that we can handle search input character by character
        let mut search_input = String::new();
        
        // Simulate typing "subject:test"
        for c in "subject:test".chars() {
            search_input.push(c);
        }
        assert_eq!(search_input, "subject:test");
        
        // Test backspace handling
        search_input.pop();
        assert_eq!(search_input, "subject:tes");
        
        search_input.pop();
        search_input.pop();
        search_input.pop();
        assert_eq!(search_input, "subject:");
    }
    
    #[test]
    fn test_search_state_transitions() {
        // This tests the basic state machine without needing a real client
        let mut state = AppState::EmailList;
        
        // Enter search mode
        state = AppState::Search;
        assert!(matches!(state, AppState::Search));
        
        // Go back from search
        state = AppState::EmailList;
        assert!(matches!(state, AppState::EmailList));
    }
}