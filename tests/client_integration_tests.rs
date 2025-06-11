#![cfg(feature = "test-utils")]

use whynot::body::BodyContent;
use whynot::client::{NotmuchClient, TagOperation};
use whynot::test_utils::mbox::{EmailMessage, MboxBuilder};
use whynot::test_utils::notmuch::TestNotmuch;

#[tokio::test]
async fn test_search_empty_database() {
    let test_notmuch = TestNotmuch::new().await.unwrap();
    let client = test_notmuch.client();

    let results = client.search("*").await.unwrap();
    assert_eq!(results.len(), 0);
}

#[tokio::test]
async fn test_search_with_single_message() {
    let test_notmuch = TestNotmuch::new().await.unwrap();

    let mbox = MboxBuilder::new()
        .add_message(
            EmailMessage::new("Important Meeting")
                .with_from("boss@company.com")
                .with_to(vec!["employee@company.com".to_string()])
                .with_body("Don't forget about our meeting tomorrow at 3pm."),
        )
        .build();

    test_notmuch.add_mbox(&mbox).await.unwrap();

    let client = test_notmuch.client();
    let results = client.search("*").await.unwrap();

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].subject, "Important Meeting");
    assert!(results[0].authors.contains("boss@company.com"));
}

#[tokio::test]
async fn test_search_with_query() {
    let test_notmuch = TestNotmuch::new().await.unwrap();

    let mbox = MboxBuilder::new()
        .add_message(EmailMessage::new("Project Update").with_body("The project is on track"))
        .add_message(EmailMessage::new("Meeting Notes").with_body("Discussion about deadlines"))
        .add_message(EmailMessage::new("Vacation Request").with_body("I'd like to take time off"))
        .build();

    test_notmuch.add_mbox(&mbox).await.unwrap();

    let client = test_notmuch.client();

    // Search for messages containing "project"
    let results = client.search("project").await.unwrap();
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].subject, "Project Update");

    // Search by subject
    let results = client.search("subject:Meeting").await.unwrap();
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].subject, "Meeting Notes");
}

#[tokio::test]
async fn test_show_thread() {
    let test_notmuch = TestNotmuch::new().await.unwrap();

    let mbox = MboxBuilder::new()
        .add_thread("Discussion Thread", 3)
        .build();

    test_notmuch.add_mbox(&mbox).await.unwrap();

    let client = test_notmuch.client();
    let search_results = client
        .search("subject:\"Discussion Thread\"")
        .await
        .unwrap();

    assert!(!search_results.is_empty());
    let thread_id = search_results[0].thread_id();

    let thread = client.show(&format!("thread:{}", thread_id)).await.unwrap();
    let messages = thread.get_messages();

    assert_eq!(messages.len(), 4); // Original + 3 replies
    assert_eq!(
        messages[0].headers.subject,
        Some("Discussion Thread".to_string())
    );
    assert_eq!(
        messages[1].headers.subject,
        Some("Re: Discussion Thread".to_string())
    );
}

#[tokio::test]
async fn test_tag_operations() {
    let test_notmuch = TestNotmuch::new().await.unwrap();

    let mbox = MboxBuilder::new()
        .add_message(EmailMessage::new("Test Tagging"))
        .build();

    test_notmuch.add_mbox(&mbox).await.unwrap();

    let client = test_notmuch.client();

    // Initial search - should have default tags
    let results = client.search("*").await.unwrap();
    assert_eq!(results.len(), 1);
    let _initial_tags = &results[0].tags;

    // Add tags
    client
        .tag(
            "*",
            &[
                TagOperation::Add("important".to_string()),
                TagOperation::Add("work".to_string()),
            ],
        )
        .await
        .unwrap();

    // Verify tags were added
    let results = client.search("*").await.unwrap();
    assert!(results[0].tags.contains(&"important".to_string()));
    assert!(results[0].tags.contains(&"work".to_string()));

    // Remove a tag
    client
        .tag("*", &[TagOperation::Remove("work".to_string())])
        .await
        .unwrap();

    // Verify tag was removed
    let results = client.search("*").await.unwrap();
    assert!(results[0].tags.contains(&"important".to_string()));
    assert!(!results[0].tags.contains(&"work".to_string()));
}

#[tokio::test]
async fn test_search_with_attachments() {
    let test_notmuch = TestNotmuch::new().await.unwrap();

    let mbox = MboxBuilder::new()
        .add_message(EmailMessage::new("Document Attached").with_attachment(
            "report.pdf",
            "application/pdf",
            b"PDF content here",
        ))
        .add_message(EmailMessage::new("No Attachments"))
        .build();

    test_notmuch.add_mbox(&mbox).await.unwrap();

    let client = test_notmuch.client();

    // Search for messages with attachments
    let results = client.search("tag:attachment").await.unwrap();
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].subject, "Document Attached");

    // Show the message to verify attachment details
    let search_results = client
        .search("subject:\"Document Attached\"")
        .await
        .unwrap();
    let thread_id = search_results[0].thread_id();
    let thread = client.show(&format!("thread:{}", thread_id)).await.unwrap();
    let messages = thread.get_messages();

    assert_eq!(messages.len(), 1);
    assert!(messages[0].has_attachments());

    let attachments = messages[0].get_attachments();
    assert_eq!(attachments.len(), 1);
    assert_eq!(attachments[0].filename.as_ref().unwrap(), "report.pdf");
}

#[tokio::test]
async fn test_complex_thread_structure() {
    let test_notmuch = TestNotmuch::new().await.unwrap();

    // Create a more complex thread structure
    let thread_root_id = "<root@example.com>";
    let reply1_id = "<reply1@example.com>";
    let reply2_id = "<reply2@example.com>";

    let mbox = MboxBuilder::new()
        .add_message(
            EmailMessage::new("Complex Thread")
                .with_message_id(thread_root_id)
                .with_from("alice@example.com"),
        )
        .add_message(
            EmailMessage::new("Re: Complex Thread")
                .with_message_id(reply1_id)
                .with_in_reply_to(thread_root_id)
                .with_from("bob@example.com"),
        )
        .add_message(
            EmailMessage::new("Re: Complex Thread")
                .with_message_id(reply2_id)
                .with_in_reply_to(reply1_id)
                .with_from("charlie@example.com"),
        )
        .build();

    test_notmuch.add_mbox(&mbox).await.unwrap();

    let client = test_notmuch.client();
    let search_results = client.search("subject:\"Complex Thread\"").await.unwrap();

    assert_eq!(search_results.len(), 1);
    let thread_id = search_results[0].thread_id();

    let thread = client.show(&format!("thread:{}", thread_id)).await.unwrap();
    let messages = thread.get_messages();

    assert_eq!(messages.len(), 3);

    // Verify the thread structure
    assert_eq!(messages[0].headers.from, "<alice@example.com>");
    assert_eq!(messages[1].headers.from, "<bob@example.com>");
    assert_eq!(messages[2].headers.from, "<charlie@example.com>");
}

#[tokio::test]
async fn test_date_search() {
    let test_notmuch = TestNotmuch::new().await.unwrap();

    use chrono::{Duration, Utc};

    let yesterday = Utc::now() - Duration::days(1);
    let last_week = Utc::now() - Duration::days(7);

    let mbox = MboxBuilder::new()
        .add_message(EmailMessage::new("Recent Message").with_date(yesterday))
        .add_message(EmailMessage::new("Old Message").with_date(last_week))
        .build();

    test_notmuch.add_mbox(&mbox).await.unwrap();

    let client = test_notmuch.client();

    // Search for messages from the last 3 days
    let results = client.search("date:3days..").await.unwrap();
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].subject, "Recent Message");

    // Search for all messages
    let results = client.search("date:10days..").await.unwrap();
    assert_eq!(results.len(), 2);
}

#[tokio::test]
async fn test_config_operations() {
    let test_notmuch = TestNotmuch::new().await.unwrap();
    let client = test_notmuch.client();

    // Set a config value
    client.config_set("test.key", "test value").await.unwrap();

    // Get the config value
    let value = client.config_get("test.key").await.unwrap();
    assert_eq!(value, "test value");

    // Get user email (set during setup)
    let email = client.config_get("user.primary_email").await.unwrap();
    assert_eq!(email, "test@example.com");
}

#[tokio::test]
async fn test_insert_message() {
    let test_notmuch = TestNotmuch::new().await.unwrap();
    let client = test_notmuch.client();

    // Create a simple email message
    let message = EmailMessage::new("Inserted Message")
        .with_from("sender@test.com")
        .with_to(vec!["recipient@test.com".to_string()])
        .with_body("This message was inserted directly.");

    let message_bytes = message.to_mbox_entry();

    // Remove the mbox From line for insert
    let email_content = message_bytes
        .split(|&b| b == b'\n')
        .skip(1)
        .collect::<Vec<_>>()
        .join(&b'\n');

    // Insert the message
    let _result = client
        .insert(&email_content, Some("inbox"), &["new", "unread"])
        .await
        .unwrap();

    // Verify the message was inserted
    let results = client.search("subject:\"Inserted Message\"").await.unwrap();
    assert_eq!(results.len(), 1);
    assert!(results[0].tags.contains(&"new".to_string()));
    assert!(results[0].tags.contains(&"unread".to_string()));
    assert!(results[0].tags.contains(&"inbox".to_string()));
}

#[tokio::test]
async fn test_multiple_tag_operations() {
    let test_notmuch = TestNotmuch::new().await.unwrap();

    let mbox = MboxBuilder::new()
        .add_message(EmailMessage::new("Message 1"))
        .add_message(EmailMessage::new("Message 2"))
        .add_message(EmailMessage::new("Message 3"))
        .build();

    test_notmuch.add_mbox(&mbox).await.unwrap();

    let client = test_notmuch.client();

    // Tag different messages differently
    client
        .tag(
            "subject:\"Message 1\"",
            &[
                TagOperation::Add("priority:high".to_string()),
                TagOperation::Add("project:alpha".to_string()),
            ],
        )
        .await
        .unwrap();

    client
        .tag(
            "subject:\"Message 2\"",
            &[
                TagOperation::Add("priority:low".to_string()),
                TagOperation::Add("project:beta".to_string()),
            ],
        )
        .await
        .unwrap();

    // Search by tags
    let high_priority = client.search("tag:priority:high").await.unwrap();
    assert_eq!(high_priority.len(), 1);
    assert_eq!(high_priority[0].subject, "Message 1");

    let beta_project = client.search("tag:project:beta").await.unwrap();
    assert_eq!(beta_project.len(), 1);
    assert_eq!(beta_project[0].subject, "Message 2");
}

#[tokio::test]
async fn test_list_tags() {
    let test_notmuch = TestNotmuch::new().await.unwrap();

    let mbox = MboxBuilder::new()
        .add_message(EmailMessage::new("Message 1"))
        .add_message(EmailMessage::new("Message 2"))
        .build();

    test_notmuch.add_mbox(&mbox).await.unwrap();

    let client = test_notmuch.client();

    // Add some custom tags
    client
        .tag(
            "subject:\"Message 1\"",
            &[
                TagOperation::Add("important".to_string()),
                TagOperation::Add("work".to_string()),
            ],
        )
        .await
        .unwrap();

    client
        .tag(
            "subject:\"Message 2\"",
            &[
                TagOperation::Add("personal".to_string()),
                TagOperation::Add("urgent".to_string()),
            ],
        )
        .await
        .unwrap();

    // List all tags
    let tags = client.list_tags().await.unwrap();

    // Should contain default tags plus our custom tags
    assert!(tags.contains(&"inbox".to_string()));
    assert!(tags.contains(&"unread".to_string()));
    assert!(tags.contains(&"important".to_string()));
    assert!(tags.contains(&"work".to_string()));
    assert!(tags.contains(&"personal".to_string()));
    assert!(tags.contains(&"urgent".to_string()));

    // Tags should be unique
    let mut sorted_tags = tags.clone();
    sorted_tags.sort();
    sorted_tags.dedup();
    assert_eq!(tags.len(), sorted_tags.len());
}

#[tokio::test]
async fn test_list_tags_empty_database() {
    let test_notmuch = TestNotmuch::new().await.unwrap();
    let client = test_notmuch.client();

    // Empty database should return empty tag list
    let tags = client.list_tags().await.unwrap();
    assert_eq!(tags.len(), 0);
}

#[tokio::test]
async fn test_list_tags_with_remote_client() {
    // This test verifies that list_tags works correctly with both local and remote clients
    let test_notmuch = TestNotmuch::new().await.unwrap();

    // Add messages with various tags
    let mbox = MboxBuilder::new()
        .add_message(
            EmailMessage::new("Work Email")
                .with_from("boss@company.com")
                .with_to(vec!["employee@company.com".to_string()]),
        )
        .add_message(EmailMessage::new("Personal Email").with_from("friend@gmail.com"))
        .add_message(EmailMessage::new("Newsletter").with_from("news@service.com"))
        .build();

    test_notmuch.add_mbox(&mbox).await.unwrap();

    let client = test_notmuch.client();

    // Add custom tags to messages
    client
        .tag(
            "subject:\"Work Email\"",
            &[
                TagOperation::Add("work".to_string()),
                TagOperation::Add("important".to_string()),
                TagOperation::Add("action-required".to_string()),
            ],
        )
        .await
        .unwrap();

    client
        .tag(
            "subject:\"Personal Email\"",
            &[
                TagOperation::Add("personal".to_string()),
                TagOperation::Add("friends".to_string()),
            ],
        )
        .await
        .unwrap();

    client
        .tag(
            "subject:Newsletter",
            &[
                TagOperation::Add("newsletter".to_string()),
                TagOperation::Add("subscriptions".to_string()),
                TagOperation::Remove("inbox".to_string()),
            ],
        )
        .await
        .unwrap();

    // List all tags - should include default and custom tags
    let tags = client.list_tags().await.unwrap();

    // Log the tags for debugging
    eprintln!("Found {} tags: {:?}", tags.len(), tags);

    // Verify we have tags
    assert!(!tags.is_empty(), "Expected to find tags but got empty list");

    // Verify expected tags are present
    assert!(tags.contains(&"inbox".to_string()), "Expected 'inbox' tag");
    assert!(
        tags.contains(&"unread".to_string()),
        "Expected 'unread' tag"
    );
    assert!(tags.contains(&"work".to_string()), "Expected 'work' tag");
    assert!(
        tags.contains(&"important".to_string()),
        "Expected 'important' tag"
    );
    assert!(
        tags.contains(&"action-required".to_string()),
        "Expected 'action-required' tag"
    );
    assert!(
        tags.contains(&"personal".to_string()),
        "Expected 'personal' tag"
    );
    assert!(
        tags.contains(&"friends".to_string()),
        "Expected 'friends' tag"
    );
    assert!(
        tags.contains(&"newsletter".to_string()),
        "Expected 'newsletter' tag"
    );
    assert!(
        tags.contains(&"subscriptions".to_string()),
        "Expected 'subscriptions' tag"
    );
}

#[tokio::test]
async fn test_message_content_retrieval() {
    // Test that message body content is properly retrieved
    let test_notmuch = TestNotmuch::new().await.unwrap();

    let plain_text_body =
        "This is a plain text email message.\nWith multiple lines.\nAnd some content.";

    let mbox = MboxBuilder::new()
        .add_message(
            EmailMessage::new("Plain Text Email")
                .with_from("sender@example.com")
                .with_body(plain_text_body),
        )
        .build();

    test_notmuch.add_mbox(&mbox).await.unwrap();

    let client = test_notmuch.client();

    // Test plain text message
    let search_results = client.search("subject:\"Plain Text Email\"").await.unwrap();
    assert_eq!(search_results.len(), 1);
    let thread_id = search_results[0].thread_id();

    let thread = client.show(&format!("thread:{}", thread_id)).await.unwrap();
    let messages = thread.get_messages();
    assert_eq!(messages.len(), 1);

    let message = &messages[0];
    assert_eq!(
        message.headers.subject,
        Some("Plain Text Email".to_string())
    );

    // Check that body parts exist
    assert!(!message.body.is_empty(), "Message should have body parts");

    // Find the text content
    let mut found_text_content = false;
    for part in &message.body {
        if part.content_type.starts_with("text/plain") {
            if let BodyContent::Text(ref content) = part.content {
                assert!(
                    content.contains("This is a plain text email message"),
                    "Expected to find plain text content in body"
                );
                found_text_content = true;
            }
        }
    }
    assert!(
        found_text_content,
        "Did not find expected text/plain content in message body"
    );
}

#[tokio::test]
async fn test_notmuch_search_tags_output_format() {
    // Test the actual notmuch command output format for tags
    let test_notmuch = TestNotmuch::new().await.unwrap();

    // Add messages with various tags
    let mbox = MboxBuilder::new()
        .add_message(EmailMessage::new("Test 1"))
        .add_message(EmailMessage::new("Test 2"))
        .build();

    test_notmuch.add_mbox(&mbox).await.unwrap();

    let client = test_notmuch.client();

    // Add custom tags
    client
        .tag(
            "*",
            &[
                TagOperation::Add("custom1".to_string()),
                TagOperation::Add("custom2".to_string()),
            ],
        )
        .await
        .unwrap();

    // Now test the raw command output
    use std::process::Stdio;
    use tokio::process::Command;

    let db_path = test_notmuch.database_path();
    let config_path = db_path.join("config");
    let output = Command::new("notmuch")
        .env("NOTMUCH_DATABASE", db_path)
        .env("NOTMUCH_CONFIG", &config_path)
        .args(["search", "--output=tags", "--format=json", "*"])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .await
        .unwrap();

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    eprintln!("notmuch search --output=tags stdout: {}", stdout);
    eprintln!("notmuch search --output=tags stderr: {}", stderr);

    // Verify it's valid JSON
    let parsed: Result<Vec<String>, _> = serde_json::from_str(&stdout);
    assert!(
        parsed.is_ok(),
        "Output should be valid JSON array of strings"
    );
}
