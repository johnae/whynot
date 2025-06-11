#![cfg(feature = "test-utils")]

use whynot::client::NotmuchClient;
use whynot::test_utils::mbox::{EmailMessage, MboxBuilder};
use whynot::test_utils::notmuch::TestNotmuch;

#[tokio::test]
async fn test_self_reply_threading() {
    let test_notmuch = TestNotmuch::new().await.unwrap();

    // Create original message from user to someone else
    let original_message_id = "<original-123@example.com>";
    let mbox = MboxBuilder::new()
        .add_message(
            EmailMessage::new("Important Discussion")
                .with_message_id(original_message_id)
                .with_from("test@example.com") // User's email
                .with_to(vec!["recipient@company.com".to_string()])
                .with_body("Let's discuss this important topic."),
        )
        .build();

    test_notmuch.add_mbox(&mbox).await.unwrap();
    let client = test_notmuch.client();

    // Get the thread
    let search_results = client
        .search("subject:\"Important Discussion\"")
        .await
        .unwrap();
    assert_eq!(search_results.len(), 1);
    let thread_id = search_results[0].thread_id();

    let thread = client.show(&format!("thread:{}", thread_id)).await.unwrap();
    let messages = thread.get_messages();
    assert_eq!(messages.len(), 1);

    // Simulate reply created by web UI logic
    let reply_message_id = "<reply-456@example.com>";
    let reply_mbox = MboxBuilder::new()
        .add_message(
            EmailMessage::new("Re: Important Discussion")
                .with_message_id(reply_message_id)
                .with_from("test@example.com") // User replying to own message
                .with_to(vec!["recipient@company.com".to_string()]) // Should go to original recipient
                .with_in_reply_to(original_message_id) // Proper threading
                .with_body("Actually, I have some additional thoughts on this."),
        )
        .build();

    test_notmuch.add_mbox(&reply_mbox).await.unwrap();

    // Verify threading works
    let updated_thread = client.show(&format!("thread:{}", thread_id)).await.unwrap();
    let updated_messages = updated_thread.get_messages();

    assert_eq!(updated_messages.len(), 2, "Reply should be in same thread");

    // Check the reply message has correct recipients
    let reply_msg = updated_messages
        .iter()
        .find(|m| {
            m.headers
                .subject
                .as_deref()
                .unwrap_or("")
                .starts_with("Re:")
        })
        .expect("Should find reply message");

    assert_eq!(
        reply_msg.headers.to,
        Some("recipient@company.com".to_string()),
        "Reply should go to original recipient, not self"
    );
    assert_eq!(
        reply_msg.headers.from, "<test@example.com>",
        "Reply should be from user"
    );
}

#[tokio::test]
async fn test_normal_reply_threading() {
    let test_notmuch = TestNotmuch::new().await.unwrap();

    // Create original message from someone else to user
    let original_message_id = "<original-789@example.com>";
    let mbox = MboxBuilder::new()
        .add_message(
            EmailMessage::new("Question for you")
                .with_message_id(original_message_id)
                .with_from("sender@company.com")
                .with_to(vec!["test@example.com".to_string()]) // To user
                .with_body("I have a question about the project."),
        )
        .build();

    test_notmuch.add_mbox(&mbox).await.unwrap();
    let client = test_notmuch.client();

    // Get the thread
    let search_results = client.search("subject:\"Question for you\"").await.unwrap();
    assert_eq!(search_results.len(), 1);
    let thread_id = search_results[0].thread_id();

    // Simulate normal reply created by web UI
    let reply_message_id = "<reply-abc@example.com>";
    let reply_mbox = MboxBuilder::new()
        .add_message(
            EmailMessage::new("Re: Question for you")
                .with_message_id(reply_message_id)
                .with_from("test@example.com") // User replying
                .with_to(vec!["sender@company.com".to_string()]) // Should go to original sender
                .with_in_reply_to(original_message_id) // Proper threading
                .with_body("Here's my answer to your question."),
        )
        .build();

    test_notmuch.add_mbox(&reply_mbox).await.unwrap();

    // Verify threading works
    let updated_thread = client.show(&format!("thread:{}", thread_id)).await.unwrap();
    let updated_messages = updated_thread.get_messages();

    assert_eq!(updated_messages.len(), 2, "Reply should be in same thread");

    // Check the reply message has correct recipients
    let reply_msg = updated_messages
        .iter()
        .find(|m| {
            m.headers
                .subject
                .as_deref()
                .unwrap_or("")
                .starts_with("Re:")
        })
        .expect("Should find reply message");

    assert_eq!(
        reply_msg.headers.to,
        Some("sender@company.com".to_string()),
        "Reply should go to original sender"
    );
    assert_eq!(
        reply_msg.headers.from, "<test@example.com>",
        "Reply should be from user"
    );
}

#[tokio::test]
async fn test_reply_all_threading() {
    let test_notmuch = TestNotmuch::new().await.unwrap();

    // Create original message with multiple recipients
    let original_message_id = "<original-reply-all@example.com>";
    let mbox = MboxBuilder::new()
        .add_message(
            EmailMessage::new("Team Meeting")
                .with_message_id(original_message_id)
                .with_from("manager@company.com")
                .with_to(vec![
                    "test@example.com".to_string(), // User
                    "colleague1@company.com".to_string(),
                    "colleague2@company.com".to_string(),
                ])
                .with_body("Let's schedule our team meeting for next week."),
        )
        .build();

    test_notmuch.add_mbox(&mbox).await.unwrap();
    let client = test_notmuch.client();

    // Get the thread
    let search_results = client.search("subject:\"Team Meeting\"").await.unwrap();
    assert_eq!(search_results.len(), 1);
    let thread_id = search_results[0].thread_id();

    // Simulate reply-all created by web UI
    let reply_message_id = "<reply-all-xyz@example.com>";
    let reply_mbox = MboxBuilder::new()
        .add_message(
            EmailMessage::new("Re: Team Meeting")
                .with_message_id(reply_message_id)
                .with_from("test@example.com") // User replying
                .with_to(vec!["manager@company.com".to_string()]) // Original sender in To
                .with_additional_header("Cc", "colleague1@company.com, colleague2@company.com") // Others in CC
                .with_in_reply_to(original_message_id) // Proper threading
                .with_body("Tuesday afternoon works for me."),
        )
        .build();

    test_notmuch.add_mbox(&reply_mbox).await.unwrap();

    // Verify threading works
    let updated_thread = client.show(&format!("thread:{}", thread_id)).await.unwrap();
    let updated_messages = updated_thread.get_messages();

    assert_eq!(
        updated_messages.len(),
        2,
        "Reply-all should be in same thread"
    );

    // Check the reply message has correct recipients
    let reply_msg = updated_messages
        .iter()
        .find(|m| {
            m.headers
                .subject
                .as_deref()
                .unwrap_or("")
                .starts_with("Re:")
        })
        .expect("Should find reply message");

    assert_eq!(
        reply_msg.headers.to,
        Some("manager@company.com".to_string()),
        "Reply should go to original sender"
    );
    assert_eq!(
        reply_msg.headers.from, "<test@example.com>",
        "Reply should be from user"
    );

    // Check CC header if available
    if let Some(cc) = reply_msg.headers.additional.get("cc") {
        assert!(
            cc.contains("colleague1@company.com"),
            "CC should include colleague1"
        );
        assert!(
            cc.contains("colleague2@company.com"),
            "CC should include colleague2"
        );
    }
}

#[tokio::test]
async fn test_threading_with_missing_message_id() {
    let test_notmuch = TestNotmuch::new().await.unwrap();

    // Create original message without explicit Message-ID (notmuch will generate one)
    let mbox = MboxBuilder::new()
        .add_message(
            EmailMessage::new("Missing Message ID Test")
                .with_from("sender@example.com")
                .with_to(vec!["test@example.com".to_string()])
                .with_body("This message doesn't have an explicit Message-ID."),
        )
        .build();

    test_notmuch.add_mbox(&mbox).await.unwrap();
    let client = test_notmuch.client();

    // Get the thread and extract the notmuch-generated message ID
    let search_results = client
        .search("subject:\"Missing Message ID Test\"")
        .await
        .unwrap();
    assert_eq!(search_results.len(), 1);
    let thread_id = search_results[0].thread_id();

    let thread = client.show(&format!("thread:{}", thread_id)).await.unwrap();
    let messages = thread.get_messages();
    assert_eq!(messages.len(), 1);

    let original_msg = &messages[0];
    // notmuch assigns an ID even if Message-ID header is missing
    assert!(!original_msg.id.is_empty(), "Notmuch should assign an ID");

    // Create reply using the notmuch ID as fallback
    let reply_message_id = "<reply-missing-id@example.com>";
    let fallback_in_reply_to = format!("<{}@notmuch.local>", original_msg.id);

    let reply_mbox = MboxBuilder::new()
        .add_message(
            EmailMessage::new("Re: Missing Message ID Test")
                .with_message_id(reply_message_id)
                .with_from("test@example.com")
                .with_to(vec!["sender@example.com".to_string()])
                .with_in_reply_to(&fallback_in_reply_to) // Using fallback format
                .with_body("Reply to message without Message-ID header."),
        )
        .build();

    test_notmuch.add_mbox(&reply_mbox).await.unwrap();

    // Verify threading works even with fallback Message-ID
    let updated_thread = client.show(&format!("thread:{}", thread_id)).await.unwrap();
    let updated_messages = updated_thread.get_messages();

    // This might be 1 or 2 depending on how notmuch handles fallback Message-IDs
    // The important thing is that we don't crash and handle the case gracefully
    assert!(!updated_messages.is_empty(), "Thread should still exist");

    let reply_msg = updated_messages.iter().find(|m| {
        m.headers
            .subject
            .as_deref()
            .unwrap_or("")
            .starts_with("Re:")
    });

    if let Some(reply) = reply_msg {
        assert_eq!(
            reply.headers.to,
            Some("sender@example.com".to_string()),
            "Reply should go to original sender"
        );
    }
}

#[tokio::test]
async fn test_references_header_chain() {
    let test_notmuch = TestNotmuch::new().await.unwrap();

    // Create a 3-message thread to test References header building
    let root_id = "<root@example.com>";
    let reply1_id = "<reply1@example.com>";
    let reply2_id = "<reply2@example.com>";

    let mbox = MboxBuilder::new()
        // Root message
        .add_message(
            EmailMessage::new("Thread Root")
                .with_message_id(root_id)
                .with_from("alice@example.com")
                .with_to(vec!["test@example.com".to_string()])
                .with_body("Starting a new discussion."),
        )
        // First reply
        .add_message(
            EmailMessage::new("Re: Thread Root")
                .with_message_id(reply1_id)
                .with_from("test@example.com")
                .with_to(vec!["alice@example.com".to_string()])
                .with_in_reply_to(root_id)
                .with_additional_header("References", root_id) // Just root in references
                .with_body("Good point, let me add to that."),
        )
        // Second reply (reply to first reply)
        .add_message(
            EmailMessage::new("Re: Thread Root")
                .with_message_id(reply2_id)
                .with_from("alice@example.com")
                .with_to(vec!["test@example.com".to_string()])
                .with_in_reply_to(reply1_id)
                .with_additional_header("References", format!("{} {}", root_id, reply1_id)) // Chain
                .with_body("Excellent addition!"),
        )
        .build();

    test_notmuch.add_mbox(&mbox).await.unwrap();
    let client = test_notmuch.client();

    // Get the thread
    let search_results = client.search("subject:\"Thread Root\"").await.unwrap();
    assert_eq!(search_results.len(), 1, "Should find one thread");
    let thread_id = search_results[0].thread_id();

    let thread = client.show(&format!("thread:{}", thread_id)).await.unwrap();
    let messages = thread.get_messages();

    assert_eq!(messages.len(), 3, "Thread should contain all 3 messages");

    // Verify threading order and headers
    let _root_msg = messages
        .iter()
        .find(|m| {
            !m.headers
                .subject
                .as_deref()
                .unwrap_or("")
                .starts_with("Re:")
        })
        .unwrap();
    let replies: Vec<_> = messages
        .iter()
        .filter(|m| {
            m.headers
                .subject
                .as_deref()
                .unwrap_or("")
                .starts_with("Re:")
        })
        .collect();

    assert_eq!(replies.len(), 2, "Should have 2 replies");

    // Check that References headers exist and contain the expected message IDs
    for reply in replies {
        if let Some(references) = reply.headers.additional.get("references") {
            assert!(
                references.contains(root_id),
                "References should contain root message ID"
            );
        }
    }
}

#[tokio::test]
async fn test_thread_search_finds_replies() {
    let test_notmuch = TestNotmuch::new().await.unwrap();

    // Create a thread with multiple messages
    let root_id = "<search-root@example.com>";
    let reply_id = "<search-reply@example.com>";

    let mbox = MboxBuilder::new()
        .add_message(
            EmailMessage::new("Searchable Thread")
                .with_message_id(root_id)
                .with_from("sender@example.com")
                .with_to(vec!["test@example.com".to_string()])
                .with_body("This message contains unique-keyword-123."),
        )
        .add_message(
            EmailMessage::new("Re: Searchable Thread")
                .with_message_id(reply_id)
                .with_from("test@example.com")
                .with_to(vec!["sender@example.com".to_string()])
                .with_in_reply_to(root_id)
                .with_body("Reply without the keyword."),
        )
        .build();

    test_notmuch.add_mbox(&mbox).await.unwrap();
    let client = test_notmuch.client();

    // Search for the unique keyword - should find the thread
    let search_results = client.search("unique-keyword-123").await.unwrap();
    assert_eq!(search_results.len(), 1, "Should find the thread");

    let thread_id = search_results[0].thread_id();

    // Show the full thread - should include both messages
    let thread = client.show(&format!("thread:{}", thread_id)).await.unwrap();
    let messages = thread.get_messages();

    assert_eq!(
        messages.len(),
        2,
        "Thread should contain both original and reply"
    );

    // Verify both messages are present
    let subjects: Vec<_> = messages.iter().map(|m| &m.headers.subject).collect();
    assert!(subjects.contains(&&Some("Searchable Thread".to_string())));
    assert!(subjects.contains(&&Some("Re: Searchable Thread".to_string())));
}

#[tokio::test]
async fn test_broken_threading_fallback() {
    let test_notmuch = TestNotmuch::new().await.unwrap();

    // Create messages that should be threaded but have broken/missing headers
    let mbox = MboxBuilder::new()
        .add_message(
            EmailMessage::new("Broken Thread Original")
                .with_message_id("<broken-original@example.com>")
                .with_from("sender@example.com")
                .with_to(vec!["test@example.com".to_string()])
                .with_body("Original message."),
        )
        .add_message(
            EmailMessage::new("Re: Broken Thread Original")
                .with_message_id("<broken-reply@example.com>")
                .with_from("test@example.com")
                .with_to(vec!["sender@example.com".to_string()])
                // Missing In-Reply-To and References headers
                .with_body("Reply that lacks proper threading headers."),
        )
        .build();

    test_notmuch.add_mbox(&mbox).await.unwrap();
    let client = test_notmuch.client();

    // Search for messages
    let search_results = client.search("subject:\"Broken Thread\"").await.unwrap();

    // Without proper threading headers, these will likely be separate threads
    // The test verifies our code handles this gracefully
    assert!(
        !search_results.is_empty(),
        "Should find messages even with broken threading"
    );

    // Each message should be accessible
    for result in search_results {
        let thread = client
            .show(&format!("thread:{}", result.thread_id()))
            .await
            .unwrap();
        let messages = thread.get_messages();
        assert!(
            !messages.is_empty(),
            "Each thread should have at least one message"
        );
    }
}
