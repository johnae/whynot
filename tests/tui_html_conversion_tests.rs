use whynot::tui::app::App;
use whynot::client::create_client;
use whynot::config::Config;
use whynot::thread::Message;
use whynot::body::{BodyPart, BodyContent};
use whynot::common::{Headers, CryptoInfo};
use std::sync::Arc;

#[tokio::test]
async fn test_tui_html_conversion_basic() {
    // Create a mock client config for testing
    let config = Config::default();
    let client_config = config.to_client_config().unwrap();
    let client = create_client(client_config).unwrap();
    let client = Arc::from(client) as Arc<dyn whynot::client::NotmuchClient>;
    
    // Create TUI app with HTML converter (no mail sender for this test)
    let app = App::new(client, None).await.unwrap();
    
    // Create a mock message with HTML content
    let html_content = r#"
        <html>
            <body>
                <h1>Test Email Subject</h1>
                <p>This is a paragraph with <strong>bold text</strong> and <em>italic text</em>.</p>
                <ul>
                    <li>First item</li>
                    <li>Second item</li>
                </ul>
                <a href="https://example.com">A link</a>
            </body>
        </html>
    "#;
    
    let message = Message {
        id: "test-email-id".to_string(),
        is_match: true,
        excluded: false,
        filename: vec!["/test/path".to_string()],
        timestamp: 1234567890,
        date_relative: "1 hour ago".to_string(),
        tags: vec!["test".to_string()],
        duplicate: None,
        body: vec![
            BodyPart {
                id: 1,
                content_type: "text/html".to_string(),
                content: BodyContent::Text(html_content.to_string()),
                content_disposition: None,
                content_id: None,
                filename: None,
                content_transfer_encoding: None,
                content_length: None,
            }
        ],
        headers: Headers {
            subject: "Test Subject".to_string(),
            from: "test@example.com".to_string(),
            to: "user@example.com".to_string(),
            reply_to: None,
            date: "2025-01-01T00:00:00Z".to_string(),
            additional: std::collections::HashMap::new(),
        },
        crypto: CryptoInfo::default(),
    };
    
    // Process the email body
    let processed_body = app.process_email_body(&message).await;
    
    // Verify that HTML was converted to text
    assert!(processed_body.is_some());
    let body_text = processed_body.unwrap();
    
    // Check that HTML tags were removed/converted
    assert!(!body_text.contains("<html>"));
    assert!(!body_text.contains("<body>"));
    assert!(!body_text.contains("<h1>"));
    assert!(!body_text.contains("<p>"));
    
    // Check that text content is preserved
    assert!(body_text.contains("Test Email Subject"));
    assert!(body_text.contains("This is a paragraph"));
    assert!(body_text.contains("bold text"));
    assert!(body_text.contains("italic text"));
    assert!(body_text.contains("First item"));
    assert!(body_text.contains("Second item"));
    assert!(body_text.contains("A link"));
}

#[tokio::test]
async fn test_tui_plain_text_passthrough() {
    // Create a mock client config for testing
    let config = Config::default();
    let client_config = config.to_client_config().unwrap();
    let client = create_client(client_config).unwrap();
    let client = Arc::from(client) as Arc<dyn whynot::client::NotmuchClient>;
    
    // Create TUI app
    let app = App::new(client, None).await.unwrap();
    
    let plain_text = "This is plain text content.\nWith multiple lines.\nAnd no HTML tags.";
    
    let message = Message {
        id: "test-email-id".to_string(),
        is_match: true,
        excluded: false,
        filename: vec!["/test/path".to_string()],
        timestamp: 1234567890,
        date_relative: "1 hour ago".to_string(),
        tags: vec!["test".to_string()],
        duplicate: None,
        body: vec![
            BodyPart {
                id: 1,
                content_type: "text/plain".to_string(),
                content: BodyContent::Text(plain_text.to_string()),
                content_disposition: None,
                content_id: None,
                filename: None,
                content_transfer_encoding: None,
                content_length: None,
            }
        ],
        headers: Headers {
            subject: "Test Subject".to_string(),
            from: "test@example.com".to_string(),
            to: "user@example.com".to_string(),
            reply_to: None,
            date: "2025-01-01T00:00:00Z".to_string(),
            additional: std::collections::HashMap::new(),
        },
        crypto: CryptoInfo::default(),
    };
    
    // Process the email body
    let processed_body = app.process_email_body(&message).await;
    
    // Verify that plain text is passed through unchanged
    assert!(processed_body.is_some());
    let body_text = processed_body.unwrap();
    assert_eq!(body_text, plain_text);
}

#[tokio::test]
async fn test_tui_mixed_content_prefers_plain_text() {
    // Create a mock client config for testing
    let config = Config::default();
    let client_config = config.to_client_config().unwrap();
    let client = create_client(client_config).unwrap();
    let client = Arc::from(client) as Arc<dyn whynot::client::NotmuchClient>;
    
    // Create TUI app
    let app = App::new(client, None).await.unwrap();
    
    let plain_text = "This is the plain text version.";
    let html_content = "<html><body><p>This is the HTML version.</p></body></html>";
    
    let message = Message {
        id: "test-email-id".to_string(),
        is_match: true,
        excluded: false,
        filename: vec!["/test/path".to_string()],
        timestamp: 1234567890,
        date_relative: "1 hour ago".to_string(),
        tags: vec!["test".to_string()],
        duplicate: None,
        body: vec![
            BodyPart {
                id: 1,
                content_type: "text/plain".to_string(),
                content: BodyContent::Text(plain_text.to_string()),
                content_disposition: None,
                content_id: None,
                filename: None,
                content_transfer_encoding: None,
                content_length: None,
            },
            BodyPart {
                id: 2,
                content_type: "text/html".to_string(),
                content: BodyContent::Text(html_content.to_string()),
                content_disposition: None,
                content_id: None,
                filename: None,
                content_transfer_encoding: None,
                content_length: None,
            }
        ],
        headers: Headers {
            subject: "Test Subject".to_string(),
            from: "test@example.com".to_string(),
            to: "user@example.com".to_string(),
            reply_to: None,
            date: "2025-01-01T00:00:00Z".to_string(),
            additional: std::collections::HashMap::new(),
        },
        crypto: CryptoInfo::default(),
    };
    
    // Process the email body
    let processed_body = app.process_email_body(&message).await;
    
    // Verify that plain text is preferred over HTML
    assert!(processed_body.is_some());
    let body_text = processed_body.unwrap();
    assert_eq!(body_text, plain_text);
    assert!(!body_text.contains("HTML version"));
}