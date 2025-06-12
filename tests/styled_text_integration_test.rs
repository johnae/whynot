use std::sync::Arc;
use whynot::body::{BodyContent, BodyPart};
use whynot::client::create_client;
use whynot::common::{CryptoInfo, Headers};
use whynot::config::Config;
use whynot::thread::Message;
use whynot::tui::app::App;

#[tokio::test]
async fn test_styled_text_vs_plain_text() {
    // Create a config with styled text DISABLED
    let mut config_plain = Config::default();
    config_plain.ui.tui.styled_text = Some(false);
    let client_config = config_plain.to_client_config().unwrap();
    let client = create_client(client_config).unwrap();
    let client = Arc::from(client) as Arc<dyn whynot::client::NotmuchClient>;
    let app_plain = App::new(client.clone(), None, &config_plain).await.unwrap();

    // Create a config with styled text ENABLED
    let mut config_styled = Config::default();
    config_styled.ui.tui.styled_text = Some(true);
    let app_styled = App::new(client.clone(), None, &config_styled)
        .await
        .unwrap();

    // Create a mock message with HTML content that should show styling differences
    let html_content =
        r#"<b>Bold text</b> and <i>italic text</i> and <a href="https://example.com">a link</a>"#;

    let body_part = BodyPart {
        id: 1,
        content_type: "text/html".to_string(),
        content: BodyContent::Text(html_content.to_string()),
        filename: None,
        content_disposition: None,
        content_id: None,
        content_transfer_encoding: None,
        content_length: None,
    };

    let message = Message {
        id: "test".to_string(),
        is_match: true,
        excluded: false,
        filename: vec!["/path/to/message".to_string()],
        timestamp: 1234567890,
        date_relative: "today".to_string(),
        tags: vec!["inbox".to_string()],
        duplicate: None,
        body: vec![body_part],
        crypto: CryptoInfo::default(),
        headers: Headers {
            date: "Test Date".to_string(),
            from: "test@example.com".to_string(),
            to: Some("user@example.com".to_string()),
            subject: Some("Test Subject".to_string()),
            reply_to: None,
            additional: std::collections::HashMap::new(),
        },
    };

    // Process with plain text (should have no styling)
    let plain_result = app_plain.process_email_body_styled(&message).await;

    // Process with styled text (should have styling)
    let styled_result = app_styled.process_email_body_styled(&message).await;

    assert!(plain_result.is_some());
    assert!(styled_result.is_some());

    let plain_text = plain_result.unwrap();
    let styled_text = styled_result.unwrap();

    // Both should have content
    assert!(!plain_text.lines.is_empty());
    assert!(!styled_text.lines.is_empty());

    // Print results for manual verification
    println!("=== PLAIN TEXT RESULT ===");
    println!("Lines: {}", plain_text.lines.len());
    for (i, line) in plain_text.lines.iter().enumerate() {
        println!("Line {}: {} spans", i, line.spans.len());
        for (j, span) in line.spans.iter().enumerate() {
            println!("  Span {}: '{}' style: {:?}", j, span.content, span.style);
        }
    }

    println!("\n=== STYLED TEXT RESULT ===");
    println!("Lines: {}", styled_text.lines.len());
    for (i, line) in styled_text.lines.iter().enumerate() {
        println!("Line {}: {} spans", i, line.spans.len());
        for (j, span) in line.spans.iter().enumerate() {
            println!("  Span {}: '{}' style: {:?}", j, span.content, span.style);
        }
    }

    // Styled version should have multiple spans with different styles
    let styled_spans = &styled_text.lines[0].spans;
    assert!(
        styled_spans.len() > 1,
        "Styled text should have multiple spans"
    );

    // Look for bold text span
    let has_bold = styled_spans.iter().any(|span| {
        span.content.contains("Bold text")
            && span
                .style
                .add_modifier
                .contains(ratatui::style::Modifier::BOLD)
    });
    assert!(has_bold, "Should have bold text span");

    // Look for italic text span
    let has_italic = styled_spans.iter().any(|span| {
        span.content.contains("italic text")
            && span
                .style
                .add_modifier
                .contains(ratatui::style::Modifier::ITALIC)
    });
    assert!(has_italic, "Should have italic text span");

    // Look for link span (cyan + underlined)
    let has_link = styled_spans.iter().any(|span| {
        span.content.contains("a link")
            && span.style.fg == Some(ratatui::style::Color::Cyan)
            && span
                .style
                .add_modifier
                .contains(ratatui::style::Modifier::UNDERLINED)
    });
    assert!(has_link, "Should have styled link span");
}

#[tokio::test]
async fn test_plain_text_unchanged() {
    // Test that plain text emails work the same with or without styled text enabled

    let mut config_styled = Config::default();
    config_styled.ui.tui.styled_text = Some(true);
    let client_config = config_styled.to_client_config().unwrap();
    let client = create_client(client_config).unwrap();
    let client = Arc::from(client) as Arc<dyn whynot::client::NotmuchClient>;
    let app = App::new(client, None, &config_styled).await.unwrap();

    let plain_content = "This is plain text with no HTML tags.";

    let body_part = BodyPart {
        id: 1,
        content_type: "text/plain".to_string(),
        content: BodyContent::Text(plain_content.to_string()),
        filename: None,
        content_disposition: None,
        content_id: None,
        content_transfer_encoding: None,
        content_length: None,
    };

    let message = Message {
        id: "test".to_string(),
        is_match: true,
        excluded: false,
        filename: vec!["/path/to/message".to_string()],
        timestamp: 1234567890,
        date_relative: "today".to_string(),
        tags: vec!["inbox".to_string()],
        duplicate: None,
        body: vec![body_part],
        crypto: CryptoInfo::default(),
        headers: Headers {
            date: "Test Date".to_string(),
            from: "test@example.com".to_string(),
            to: Some("user@example.com".to_string()),
            subject: Some("Test Subject".to_string()),
            reply_to: None,
            additional: std::collections::HashMap::new(),
        },
    };

    let result = app.process_email_body_styled(&message).await;
    assert!(result.is_some());

    let text = result.unwrap();
    assert_eq!(text.lines.len(), 1);
    assert_eq!(text.lines[0].spans.len(), 1);
    assert_eq!(text.lines[0].spans[0].content, plain_content);
    // Plain text should have default styling
    assert_eq!(
        text.lines[0].spans[0].style,
        ratatui::style::Style::default()
    );
}
