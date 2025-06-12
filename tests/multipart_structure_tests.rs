//! Tests for proper multipart email structure in markdown composition
//!
//! These tests ensure that emails with both text and HTML parts use the correct
//! MIME multipart structure for optimal email client compatibility.

use whynot::mail_sender::message::ComposableMessage;

#[test]
fn test_plain_text_only_email_structure() {
    // Plain text emails should not have multipart structure
    let message = ComposableMessage::builder()
        .to("test@example.com".to_string())
        .subject("Plain text test".to_string())
        .body("This is plain text content.".to_string())
        .build()
        .expect("Failed to build plain text message");

    let rfc822_bytes = message.to_rfc822().unwrap();
    let rfc822 = std::str::from_utf8(&rfc822_bytes).unwrap();
    
    // Should NOT contain multipart headers
    assert!(!rfc822.contains("multipart/"));
    assert!(!rfc822.contains("boundary="));
    
    // Should contain simple text headers
    assert!(rfc822.contains("Content-Type: text/plain; charset=utf-8"));
    assert!(rfc822.contains("This is plain text content."));
}

#[test]
fn test_markdown_email_uses_multipart_alternative() {
    // CRITICAL TEST: Markdown emails (text + HTML) should use multipart/alternative
    let message = ComposableMessage::builder()
        .to("test@example.com".to_string())
        .subject("Markdown test".to_string())
        .body("# Markdown Header\n\nThis is **bold** text.".to_string())
        .html_body("<h1>Markdown Header</h1>\n<p>This is <strong>bold</strong> text.</p>".to_string())
        .build()
        .expect("Failed to build markdown message");

    let rfc822_bytes = message.to_rfc822().unwrap();
    let rfc822 = std::str::from_utf8(&rfc822_bytes).unwrap();
    
    // CRITICAL: Should use multipart/alternative, NOT multipart/mixed
    assert!(rfc822.contains("Content-Type: multipart/alternative;"));
    assert!(!rfc822.contains("Content-Type: multipart/mixed;"));
    
    // Should contain boundary parameter
    assert!(rfc822.contains("boundary="));
    
    // Should contain both plain text and HTML parts
    assert!(rfc822.contains("Content-Type: text/plain; charset=utf-8"));
    assert!(rfc822.contains("Content-Type: text/html; charset=utf-8"));
    
    // Should contain the actual content
    assert!(rfc822.contains("# Markdown Header"));
    assert!(rfc822.contains("<h1>Markdown Header</h1>"));
    assert!(rfc822.contains("This is **bold** text."));
    assert!(rfc822.contains("This is <strong>bold</strong> text."));
}

#[test]
fn test_attachment_only_email_uses_multipart_mixed() {
    // Emails with only attachments should use multipart/mixed
    let attachment = whynot::mail_sender::message::Attachment {
        filename: "test.txt".to_string(),
        content_type: "text/plain".to_string(),
        data: b"Hello from attachment".to_vec(),
    };

    let message = ComposableMessage::builder()
        .to("test@example.com".to_string())
        .subject("Attachment test".to_string())
        .body("Please see attached file.".to_string())
        .attachment(attachment)
        .build()
        .expect("Failed to build attachment message");

    let rfc822_bytes = message.to_rfc822().unwrap();
    let rfc822 = std::str::from_utf8(&rfc822_bytes).unwrap();
    
    // Should use multipart/mixed for attachments
    assert!(rfc822.contains("Content-Type: multipart/mixed;"));
    assert!(!rfc822.contains("Content-Type: multipart/alternative;"));
    
    // Should contain text part and attachment
    assert!(rfc822.contains("Content-Type: text/plain; charset=utf-8"));
    assert!(rfc822.contains("Content-Disposition: attachment"));
    assert!(rfc822.contains("filename=\"test.txt\""));
}

#[test]
fn test_complex_email_nested_multipart_structure() {
    // COMPLEX CASE: text + HTML + attachments should use nested structure:
    // multipart/mixed containing multipart/alternative + attachments
    let attachment = whynot::mail_sender::message::Attachment {
        filename: "document.pdf".to_string(),
        content_type: "application/pdf".to_string(),
        data: b"PDF content here".to_vec(),
    };

    let message = ComposableMessage::builder()
        .to("test@example.com".to_string())
        .subject("Complex email test".to_string())
        .body("# Report\n\nSee attached **document**.".to_string())
        .html_body("<h1>Report</h1>\n<p>See attached <strong>document</strong>.</p>".to_string())
        .attachment(attachment)
        .build()
        .expect("Failed to build complex message");

    let rfc822_bytes = message.to_rfc822().unwrap();
    let rfc822 = std::str::from_utf8(&rfc822_bytes).unwrap();
    
    // Top level should be multipart/mixed (for attachments)
    assert!(rfc822.contains("Content-Type: multipart/mixed;"));
    
    // Should contain nested multipart/alternative for text choices
    assert!(rfc822.contains("Content-Type: multipart/alternative;"));
    
    // Should contain both text formats
    assert!(rfc822.contains("Content-Type: text/plain; charset=utf-8"));
    assert!(rfc822.contains("Content-Type: text/html; charset=utf-8"));
    
    // Should contain attachment
    assert!(rfc822.contains("Content-Disposition: attachment"));
    assert!(rfc822.contains("filename=\"document.pdf\""));
    assert!(rfc822.contains("Content-Type: application/pdf"));
    
    // Should contain actual content
    assert!(rfc822.contains("# Report"));
    assert!(rfc822.contains("<h1>Report</h1>"));
}

#[test]
fn test_multipart_boundary_uniqueness() {
    // Each message should have unique boundaries to avoid conflicts
    let message1 = ComposableMessage::builder()
        .to("test@example.com".to_string())
        .subject("Test 1".to_string())
        .body("Text content".to_string())
        .html_body("<p>HTML content</p>".to_string())
        .build()
        .expect("Failed to build message 1");

    let message2 = ComposableMessage::builder()
        .to("test@example.com".to_string())
        .subject("Test 2".to_string())
        .body("Different text".to_string())
        .html_body("<p>Different HTML</p>".to_string())
        .build()
        .expect("Failed to build message 2");

    let rfc822_1_bytes = message1.to_rfc822().unwrap();
    let rfc822_1 = std::str::from_utf8(&rfc822_1_bytes).unwrap();
    let rfc822_2_bytes = message2.to_rfc822().unwrap();
    let rfc822_2 = std::str::from_utf8(&rfc822_2_bytes).unwrap();
    
    // Extract boundary values (they should be different)
    let boundary1 = extract_boundary(rfc822_1).expect("Should have boundary in message 1");
    let boundary2 = extract_boundary(rfc822_2).expect("Should have boundary in message 2");
    
    assert_ne!(boundary1, boundary2, "Boundaries should be unique between messages");
}

#[test]
fn test_mime_version_header_presence() {
    // Multipart messages should include MIME-Version header
    let message = ComposableMessage::builder()
        .to("test@example.com".to_string())
        .subject("MIME test".to_string())
        .body("Plain text".to_string())
        .html_body("<p>HTML text</p>".to_string())
        .build()
        .expect("Failed to build MIME message");

    let rfc822_bytes = message.to_rfc822().unwrap();
    let rfc822 = std::str::from_utf8(&rfc822_bytes).unwrap();
    
    // Should contain MIME version header
    assert!(rfc822.contains("MIME-Version: 1.0"));
}

/// Helper function to extract boundary value from RFC822 message
fn extract_boundary(rfc822: &str) -> Option<String> {
    for line in rfc822.lines() {
        if line.starts_with("Content-Type:") && line.contains("boundary=") {
            if let Some(boundary_part) = line.split("boundary=").nth(1) {
                // Remove quotes and extract boundary value
                let boundary = boundary_part.trim_matches('"').split(';').next()?;
                return Some(boundary.to_string());
            }
        }
    }
    None
}