use std::fs;
use whynot::{search::SearchResult, thread::Thread};

#[test]
fn test_parse_search_example_1() {
    let content = fs::read_to_string("examples/notmuch/search-example-1.md")
        .expect("Failed to read search example 1");

    // Extract JSON from markdown
    let json_start = content.find("```json").expect("JSON block not found") + 7;
    let json_end = content[json_start..]
        .find("```")
        .expect("JSON block end not found");
    let json_str = &content[json_start..json_start + json_end];

    let result: SearchResult =
        serde_json::from_str(json_str).expect("Failed to deserialize search example 1");

    assert!(!result.0.is_empty());
    assert_eq!(result.0[0].thread, "00000000000276db");
}

#[test]
fn test_parse_search_example_2_attachments() {
    let content = fs::read_to_string("examples/notmuch/search-example-2-attachments.md")
        .expect("Failed to read search example 2");

    // Extract JSON from markdown
    let json_start = content.find("```json").expect("JSON block not found") + 7;
    let json_end = content[json_start..]
        .find("```")
        .expect("JSON block end not found");
    let json_str = &content[json_start..json_start + json_end];

    let result: SearchResult =
        serde_json::from_str(json_str).expect("Failed to deserialize search example 2");

    assert!(!result.0.is_empty());
    assert!(result.0[0].tags.contains(&"attachment".to_string()));
}

#[test]
fn test_parse_thread_example_1() {
    let content = fs::read_to_string("examples/notmuch/thread-example-1.md")
        .expect("Failed to read thread example 1");

    // Extract JSON from markdown (find the thread output JSON block)
    let json_marker = "You'll see this output:";
    let marker_pos = content.find(json_marker).expect("Output marker not found");
    let after_marker = &content[marker_pos + json_marker.len()..];

    // Find the JSON block after the marker
    let json_start = after_marker.find("```json").expect("JSON block not found") + 7;
    let json_end = after_marker[json_start..]
        .find("```")
        .expect("JSON block end not found");
    let json_str = &after_marker[json_start..json_start + json_end];

    let thread: Thread =
        serde_json::from_str(json_str).expect("Failed to deserialize thread example 1");

    let messages = thread.get_messages();
    assert!(!messages.is_empty());
    assert_eq!(messages[0].id, "user@example.com");
}

#[test]
fn test_thread_messages_have_expected_fields() {
    let content = fs::read_to_string("examples/notmuch/thread-example-1.md")
        .expect("Failed to read thread example 1");

    // Extract JSON from markdown (find the thread output JSON block)
    let json_marker = "You'll see this output:";
    let marker_pos = content.find(json_marker).expect("Output marker not found");
    let after_marker = &content[marker_pos + json_marker.len()..];

    // Find the JSON block after the marker
    let json_start = after_marker.find("```json").expect("JSON block not found") + 7;
    let json_end = after_marker[json_start..]
        .find("```")
        .expect("JSON block end not found");
    let json_str = &after_marker[json_start..json_start + json_end];

    let thread: Thread =
        serde_json::from_str(json_str).expect("Failed to deserialize thread example 1");

    let messages = thread.get_messages();
    let first_message = messages[0];

    // Verify headers
    assert_eq!(
        first_message.headers.subject,
        Some("Uppföljningsmöte mellan Alice och Bob, Finance Company AB".to_string())
    );
    assert_eq!(
        first_message.headers.from,
        "\"Bob Wilson\" <user@example.com>"
    );
    assert_eq!(
        first_message.headers.to,
        Some("\"user@example.com\" <user@example.com>".to_string())
    );
    assert_eq!(
        first_message.headers.reply_to,
        Some("Bob Wilson <user@example.com>".to_string())
    );

    // Verify duplicate field
    assert_eq!(first_message.duplicate, Some(1));

    // Verify it has attachments
    assert!(first_message.has_attachments());
    let attachments = first_message.get_attachments();
    assert!(!attachments.is_empty());
}
