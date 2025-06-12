//! Tests for TUI markdown configuration options

use whynot::config::{Config, TuiConfig};

#[test]
fn test_tui_config_markdown_compose_default() {
    let config = TuiConfig::default();
    
    // markdown_compose should default to false for backward compatibility
    assert_eq!(config.markdown_compose, Some(false));
}

#[test]
fn test_tui_config_markdown_compose_can_be_set() {
    let mut config = TuiConfig::default();
    config.markdown_compose = Some(true);
    
    assert_eq!(config.markdown_compose, Some(true));
}

#[test] 
fn test_config_loading_with_markdown_compose() {
    // Test that the config can be parsed from TOML with markdown_compose setting
    let toml_content = r#"
        [ui.tui]
        markdown_compose = true
        styled_text = false
        keybindings = "vim"
    "#;
    
    let config: Config = toml::from_str(toml_content).expect("Failed to parse TOML");
    assert_eq!(config.ui.tui.markdown_compose, Some(true));
    assert_eq!(config.ui.tui.styled_text, Some(false));
    assert_eq!(config.ui.tui.keybindings, Some("vim".to_string()));
}

#[test]
fn test_config_loading_without_markdown_compose() {
    // Test that the config works when markdown_compose is not specified (will be None)
    let toml_content = r#"
        [ui.tui]
        styled_text = true
        keybindings = "emacs"
    "#;
    
    let config: Config = toml::from_str(toml_content).expect("Failed to parse TOML");
    // Should be None when not specified in TOML (defaults apply later in app logic)
    assert_eq!(config.ui.tui.markdown_compose, None);
    assert_eq!(config.ui.tui.styled_text, Some(true));
    assert_eq!(config.ui.tui.keybindings, Some("emacs".to_string()));
}