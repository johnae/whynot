use whynot::body::BodyContent;
use whynot::client::NotmuchClient;
use whynot::test_utils::{MboxBuilder, TestNotmuch, create_test_message_with_attachment};

#[tokio::test]
async fn test_attachment_download() {
    let test_db = TestNotmuch::new().await.unwrap();
    let client = test_db.client();

    // Create test message with attachment
    let attachment_content = b"This is a test PDF file content";
    let msg = create_test_message_with_attachment(
        "Test with Attachment",
        "sender@example.com",
        "recipient@example.com",
        "This email has an attachment",
        "test.pdf",
        "application/pdf",
        attachment_content,
    );

    // Index the message
    let mbox = MboxBuilder::new().add_message(msg).build();
    test_db.add_mbox(&mbox).await.unwrap();

    // Search for the message
    let results = client.search("from:sender@example.com").await.unwrap();
    assert_eq!(results.len(), 1);

    // Get the thread
    let thread_id = results[0].thread.as_str();
    let thread = client.show(&format!("thread:{}", thread_id)).await.unwrap();
    let messages = thread.get_messages();
    assert_eq!(messages.len(), 1);

    // Check that the message has attachments
    let message = &messages[0];
    assert!(message.has_attachments());

    // Get attachments
    let attachments = message.get_attachments();
    assert_eq!(attachments.len(), 1);

    let attachment = &attachments[0];
    assert_eq!(attachment.filename, Some("test.pdf".to_string()));
    assert_eq!(attachment.content_type, "application/pdf");

    // Verify attachment metadata (content is not included in normal notmuch show output)
    assert_eq!(
        attachment.content,
        BodyContent::Empty,
        "Attachment content should be empty in normal show output - content is retrieved separately"
    );

    // TODO: To get actual attachment content, we would need to:
    // 1. Use notmuch part command: notmuch part --format=raw id:<message-id> <part-id>
    // 2. Or implement a separate attachment content retrieval method
    // For now, we just verify that the attachment metadata is correct
}

#[tokio::test]
async fn test_multiple_attachments() {
    let test_db = TestNotmuch::new().await.unwrap();
    let client = test_db.client();

    // Create a message with multiple attachments
    let msg = whynot::test_utils::create_test_message_with_multiple_attachments(
        "Multiple Attachments",
        "sender@example.com",
        "recipient@example.com",
        "This email has multiple attachments",
        vec![
            ("document.pdf", "application/pdf", b"PDF content".to_vec()),
            ("image.jpg", "image/jpeg", b"JPEG content".to_vec()),
            ("data.csv", "text/csv", b"CSV content".to_vec()),
        ],
    );

    // Index the message
    let mbox = MboxBuilder::new().add_message(msg).build();
    test_db.add_mbox(&mbox).await.unwrap();

    // Search for the message
    let results = client.search("from:sender@example.com").await.unwrap();
    assert_eq!(results.len(), 1);

    // Get the thread
    let thread_id = results[0].thread.as_str();
    let thread = client.show(&format!("thread:{}", thread_id)).await.unwrap();
    let messages = thread.get_messages();
    assert_eq!(messages.len(), 1);

    // Check attachments
    let message = &messages[0];
    assert!(message.has_attachments());

    let attachments = message.get_attachments();
    assert_eq!(attachments.len(), 3);

    // Verify each attachment
    assert_eq!(attachments[0].filename, Some("document.pdf".to_string()));
    assert_eq!(attachments[0].content_type, "application/pdf");

    assert_eq!(attachments[1].filename, Some("image.jpg".to_string()));
    assert_eq!(attachments[1].content_type, "image/jpeg");

    assert_eq!(attachments[2].filename, Some("data.csv".to_string()));
    assert_eq!(attachments[2].content_type, "text/csv");
}
