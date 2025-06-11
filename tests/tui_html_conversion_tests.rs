use std::sync::Arc;
use whynot::body::{BodyContent, BodyPart};
use whynot::client::create_client;
use whynot::common::{CryptoInfo, Headers};
use whynot::config::Config;
use whynot::thread::Message;
use whynot::tui::app::App;

#[tokio::test]
async fn test_tui_html_conversion_basic() {
    // Create a mock client config for testing
    let config = Config::default();
    let client_config = config.to_client_config().unwrap();
    let client = create_client(client_config).unwrap();
    let client = Arc::from(client) as Arc<dyn whynot::client::NotmuchClient>;

    // Create TUI app with HTML converter (no mail sender for this test)
    let app = App::new(client, None, &config).await.unwrap();

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
        body: vec![BodyPart {
            id: 1,
            content_type: "text/html".to_string(),
            content: BodyContent::Text(html_content.to_string()),
            content_disposition: None,
            content_id: None,
            filename: None,
            content_transfer_encoding: None,
            content_length: None,
        }],
        headers: Headers {
            subject: Some("Test Subject".to_string()),
            from: "test@example.com".to_string(),
            to: Some("user@example.com".to_string()),
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
    let app = App::new(client, None, &config).await.unwrap();

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
        body: vec![BodyPart {
            id: 1,
            content_type: "text/plain".to_string(),
            content: BodyContent::Text(plain_text.to_string()),
            content_disposition: None,
            content_id: None,
            filename: None,
            content_transfer_encoding: None,
            content_length: None,
        }],
        headers: Headers {
            subject: Some("Test Subject".to_string()),
            from: "test@example.com".to_string(),
            to: Some("user@example.com".to_string()),
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
async fn test_tui_multipart_mixed_html_content() {
    // Create a mock client config for testing
    let config = Config::default();
    let client_config = config.to_client_config().unwrap();
    let client = create_client(client_config).unwrap();
    let client = Arc::from(client) as Arc<dyn whynot::client::NotmuchClient>;

    // Create TUI app
    let app = App::new(client, None, &config).await.unwrap();

    // Simplified APCOA-style HTML content (similar structure to the real email)
    let html_content = r#"<!DOCTYPE html>
<html>
<head><title>Payment Success SE</title></head>
<body>
<table>
<tr><td><h1>Ditt kvitto</h1></td></tr>
<tr><td>Tack för att du betalade med APCOA FLOW, <span>Test User</span>.</td></tr>
<tr><td><b>Namn :</b> <span>Test User Name</span></td></tr>
<tr><td><b>Information om din parkering :</b></td></tr>
<tr><td>Hospital Visitor Parking</td></tr>
<tr><td><b>Din parkering :</b> <span>#12345678</span></td></tr>
</table>
</body>
</html>"#;

    // Create message structure that matches APCOA email:
    // Top level multipart/mixed containing HTML part and PDF attachment
    let message = Message {
        id: "test-apcoa-email".to_string(),
        is_match: true,
        excluded: false,
        filename: vec!["/test/path".to_string()],
        timestamp: 1749472819,
        date_relative: "Mon. 14:40".to_string(),
        tags: vec![
            "Invoice".to_string(),
            "attachment".to_string(),
            "inbox".to_string(),
        ],
        duplicate: None,
        body: vec![BodyPart {
            id: 1,
            content_type: "multipart/mixed".to_string(),
            content: BodyContent::Multipart(vec![
                BodyPart {
                    id: 2,
                    content_type: "text/html".to_string(),
                    content: BodyContent::Text(html_content.to_string()),
                    content_disposition: None,
                    content_id: None,
                    filename: None,
                    content_transfer_encoding: None,
                    content_length: None,
                },
                BodyPart {
                    id: 3,
                    content_type: "application/pdf".to_string(),
                    content: BodyContent::Empty,
                    content_disposition: Some("attachment".to_string()),
                    content_id: None,
                    filename: Some("receipt (SE42324486337).pdf".to_string()),
                    content_transfer_encoding: Some("base64".to_string()),
                    content_length: Some(60574),
                },
            ]),
            content_disposition: None,
            content_id: None,
            filename: None,
            content_transfer_encoding: None,
            content_length: None,
        }],
        headers: Headers {
            subject: Some("Parkeringskvitto APCOA FLOW".to_string()),
            from: "donotreply@apcoaflow.com".to_string(),
            to: Some("user@example.com".to_string()),
            reply_to: Some("donotreply@apcoaflow.com".to_string()),
            date: "Mon, 09 Jun 2025 12:40:19 +0000".to_string(),
            additional: std::collections::HashMap::new(),
        },
        crypto: CryptoInfo::default(),
    };

    // Process the email body
    let processed_body = app.process_email_body(&message).await;

    // This should currently fail - the TUI doesn't handle nested multipart content
    // It should find the HTML content within the multipart/mixed container
    assert!(processed_body.is_some());
    let body_text = processed_body.unwrap();

    // Currently this test will FAIL because TUI shows "[No readable content]"
    // After we fix the issue, it should contain the Swedish text
    assert_ne!(body_text, "[No readable content]");
    assert!(body_text.contains("Ditt kvitto"));
    assert!(body_text.contains("Test User"));
    assert!(body_text.contains("Information om din parkering"));
    assert!(body_text.contains("Din parkering"));
    assert!(body_text.contains("#12345678"));
}

#[tokio::test]
async fn test_tui_mixed_content_prefers_plain_text() {
    // Create a mock client config for testing
    let config = Config::default();
    let client_config = config.to_client_config().unwrap();
    let client = create_client(client_config).unwrap();
    let client = Arc::from(client) as Arc<dyn whynot::client::NotmuchClient>;

    // Create TUI app
    let app = App::new(client, None, &config).await.unwrap();

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
            },
        ],
        headers: Headers {
            subject: Some("Test Subject".to_string()),
            from: "test@example.com".to_string(),
            to: Some("user@example.com".to_string()),
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
