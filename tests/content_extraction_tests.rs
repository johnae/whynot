#![cfg(feature = "test-utils")]

use whynot::client::NotmuchClient;
use whynot::test_utils::mbox::{EmailMessage, MboxBuilder};
use whynot::test_utils::notmuch::TestNotmuch;
use whynot::web::content_renderer::render_message_content;

#[tokio::test]
async fn test_plain_text_content_extraction() {
    let test_notmuch = TestNotmuch::new().await.unwrap();

    // Create an email with plain text content
    let plain_text_body = "Hello,\n\nThis is a plain text email message.\n\nBest regards,\nJohn";

    let mbox = MboxBuilder::new()
        .add_message(
            EmailMessage::new("Plain Text Test")
                .with_from("sender@example.com")
                .with_to(vec!["recipient@example.com".to_string()])
                .with_body(plain_text_body),
        )
        .build();

    test_notmuch.add_mbox(&mbox).await.unwrap();

    let client = test_notmuch.client();

    // Search for the message
    let results = client.search("*").await.unwrap();
    assert_eq!(results.len(), 1);

    // Get the thread
    let thread_id = results[0].thread_id();
    let thread = client.show(&format!("thread:{}", thread_id)).await.unwrap();

    let messages = thread.get_messages();
    assert_eq!(messages.len(), 1);

    let message = &messages[0];

    // Test the old get_text_content method
    let old_content = message.get_text_content();
    assert!(
        old_content.is_some(),
        "get_text_content should return content"
    );
    assert!(old_content.unwrap().contains("plain text email message"));

    // Test the new content renderer
    let rendered = render_message_content(message);
    assert!(rendered.has_plain(), "Should have plain text content");
    assert!(
        rendered.has_html(),
        "Should have HTML content (converted from plain)"
    );

    let plain_content = rendered.plain.as_ref().unwrap();
    assert!(plain_content.contains("plain text email message"));
    assert!(plain_content.contains("Best regards"));

    let html_content = rendered.html.as_ref().unwrap();
    assert!(html_content.contains("plain text email message"));
    assert!(html_content.contains("<br>"));
}

#[tokio::test]
async fn test_html_content_extraction() {
    let test_notmuch = TestNotmuch::new().await.unwrap();

    // Create an email with HTML content
    let html_body = r#"<html>
<head><title>Test Email</title></head>
<body>
<h1>Important Announcement</h1>
<p>This is an <strong>HTML email</strong> with <em>formatting</em>.</p>
<ul>
<li>First item</li>
<li>Second item</li>
</ul>
<p>Visit our <a href="https://example.com">website</a> for more info.</p>
</body>
</html>"#;

    let mbox = MboxBuilder::new()
        .add_message(
            EmailMessage::new("HTML Test")
                .with_from("marketing@example.com")
                .with_to(vec!["user@example.com".to_string()])
                .with_html_body(html_body),
        )
        .build();

    test_notmuch.add_mbox(&mbox).await.unwrap();

    let client = test_notmuch.client();

    // Search for the message
    let results = client.search("*").await.unwrap();
    assert_eq!(results.len(), 1);

    // Get the thread
    let thread_id = results[0].thread_id();
    let thread = client.show(&format!("thread:{}", thread_id)).await.unwrap();

    let messages = thread.get_messages();
    assert_eq!(messages.len(), 1);

    let message = &messages[0];

    // Test the content renderer
    let rendered = render_message_content(message);
    assert!(rendered.has_html(), "Should have HTML content");

    let html_content = rendered.html.as_ref().unwrap();
    assert!(html_content.contains("Important Announcement"));
    assert!(html_content.contains("<strong>HTML email</strong>"));
    assert!(html_content.contains("<em>formatting</em>"));
    assert!(html_content.contains("<ul>"));
    assert!(html_content.contains("<li>First item</li>"));

    // HTML should be sanitized - dangerous elements should be removed
    assert!(!html_content.contains("<script>"));
    assert!(!html_content.contains("javascript:"));
}

#[tokio::test]
async fn test_multipart_alternative_content_extraction() {
    let test_notmuch = TestNotmuch::new().await.unwrap();

    // Create an email with both plain text and HTML versions
    let plain_text =
        "This is the plain text version.\n\nIt contains the same information as the HTML version.";
    let html_text = r#"<html><body>
<p>This is the <strong>HTML version</strong>.</p>
<p>It contains the same information as the plain text version.</p>
</body></html>"#;

    let mbox = MboxBuilder::new()
        .add_message(
            EmailMessage::new("Multipart Alternative Test")
                .with_from("sender@example.com")
                .with_to(vec!["recipient@example.com".to_string()])
                .with_multipart_alternative_body(plain_text, html_text),
        )
        .build();

    test_notmuch.add_mbox(&mbox).await.unwrap();

    let client = test_notmuch.client();

    // Search for the message
    let results = client.search("*").await.unwrap();
    assert_eq!(results.len(), 1);

    // Get the thread
    let thread_id = results[0].thread_id();
    let thread = client.show(&format!("thread:{}", thread_id)).await.unwrap();

    let messages = thread.get_messages();
    assert_eq!(messages.len(), 1);

    let message = &messages[0];

    // Test the content renderer
    let rendered = render_message_content(message);
    assert!(rendered.has_html(), "Should have HTML content");
    assert!(rendered.has_plain(), "Should have plain text content");

    let plain_content = rendered.plain.as_ref().unwrap();
    assert!(plain_content.contains("plain text version"));

    let html_content = rendered.html.as_ref().unwrap();
    assert!(html_content.contains("<strong>HTML version</strong>"));
    assert!(html_content.contains("<p>"));
}

#[tokio::test]
async fn test_html_sanitization() {
    let test_notmuch = TestNotmuch::new().await.unwrap();

    // Create an email with potentially dangerous HTML content
    let dangerous_html = r#"<html><body>
<p>This looks innocent: <script>alert('XSS!');</script></p>
<p><a href="javascript:alert('dangerous')">Click me</a></p>
<p><img src="http://example.com/image.jpg" onerror="alert('xss')"></p>
<p style="background: url('javascript:alert(1)')">Styled text</p>
<iframe src="http://evil.com"></iframe>
<object data="http://evil.com/malware.swf"></object>
</body></html>"#;

    let mbox = MboxBuilder::new()
        .add_message(
            EmailMessage::new("Dangerous HTML Test")
                .with_from("attacker@evil.com")
                .with_to(vec!["victim@example.com".to_string()])
                .with_html_body(dangerous_html),
        )
        .build();

    test_notmuch.add_mbox(&mbox).await.unwrap();

    let client = test_notmuch.client();

    // Search for the message
    let results = client.search("*").await.unwrap();
    assert_eq!(results.len(), 1);

    // Get the thread
    let thread_id = results[0].thread_id();
    let thread = client.show(&format!("thread:{}", thread_id)).await.unwrap();

    let messages = thread.get_messages();
    assert_eq!(messages.len(), 1);

    let message = &messages[0];

    // Test the content renderer
    let rendered = render_message_content(message);
    assert!(rendered.has_html(), "Should have HTML content");

    let html_content = rendered.html.as_ref().unwrap();

    // Verify dangerous elements are removed
    assert!(!html_content.contains("<script>"));
    assert!(!html_content.contains("javascript:"));
    assert!(!html_content.contains("onerror="));
    assert!(!html_content.contains("<iframe"));
    assert!(!html_content.contains("<object"));

    // Verify safe content is preserved
    assert!(html_content.contains("This looks innocent:"));
    assert!(html_content.contains("<p>"));
    assert!(html_content.contains("<img"));
    assert!(html_content.contains("src=\"http://example.com/image.jpg\""));
}

#[tokio::test]
async fn test_empty_content_handling() {
    let test_notmuch = TestNotmuch::new().await.unwrap();

    // Create an email with no body content
    let mbox = MboxBuilder::new()
        .add_message(
            EmailMessage::new("Empty Content Test")
                .with_from("sender@example.com")
                .with_to(vec!["recipient@example.com".to_string()]), // No body content
        )
        .build();

    test_notmuch.add_mbox(&mbox).await.unwrap();

    let client = test_notmuch.client();

    // Search for the message
    let results = client.search("*").await.unwrap();
    assert_eq!(results.len(), 1);

    // Get the thread
    let thread_id = results[0].thread_id();
    let thread = client.show(&format!("thread:{}", thread_id)).await.unwrap();

    let messages = thread.get_messages();
    assert_eq!(messages.len(), 1);

    let message = &messages[0];

    // Test the old get_text_content method
    let old_content = message.get_text_content();
    // Empty messages might still have some default content, so we just check it exists
    if let Some(content) = old_content {
        // If there is content, it should be minimal/default content
        assert!(
            content.len() < 100,
            "Empty message should have minimal content, got: '{}'",
            content
        );
    }

    // Test the content renderer
    let _rendered = render_message_content(message);
    // Even "empty" messages might have minimal default content
    // The important thing is that the renderer doesn't crash and handles empty content gracefully
}

#[tokio::test]
async fn test_web_thread_view_content_display() {
    let test_notmuch = TestNotmuch::new().await.unwrap();

    // Create multiple messages with different content types
    let mbox = MboxBuilder::new()
        .add_message(
            EmailMessage::new("Plain Text Message")
                .with_from("user1@example.com")
                .with_to(vec!["user2@example.com".to_string()])
                .with_body("This is a plain text message."),
        )
        .add_message(
            EmailMessage::new("HTML Message")
                .with_from("user2@example.com")
                .with_to(vec!["user1@example.com".to_string()])
                .with_html_body("<p>This is an <strong>HTML</strong> message.</p>"),
        )
        .build();

    test_notmuch.add_mbox(&mbox).await.unwrap();

    let client = test_notmuch.client();

    // Search for messages
    let results = client.search("*").await.unwrap();
    assert_eq!(results.len(), 2);

    // Test each message
    for result in results {
        let thread = client
            .show(&format!("thread:{}", result.thread_id()))
            .await
            .unwrap();
        let messages = thread.get_messages();
        assert_eq!(messages.len(), 1);

        let message = &messages[0];
        let rendered = render_message_content(message);

        // Every message should have some form of content after rendering
        assert!(
            rendered.has_html() || rendered.has_plain(),
            "Message '{}' should have either HTML or plain content",
            message.headers.subject.as_deref().unwrap_or("(No subject)")
        );

        // If it has HTML, it should be sanitized
        if let Some(html) = &rendered.html {
            assert!(!html.contains("<script>"));
            assert!(!html.contains("javascript:"));
        }

        // The content should not be empty
        if let Some(content) = rendered.get_primary_content() {
            assert!(!content.trim().is_empty(), "Content should not be empty");
        }
    }
}
