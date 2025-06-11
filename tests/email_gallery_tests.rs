use std::fs;
use whynot::thread::Thread;
use whynot::web::content_renderer::render_message_content;

#[test]
fn test_problematic_email_files_exist() {
    // Check that all problematic email files exist and are readable
    let email_names = vec![
        "bilprovningen",
        "stockholm-film-festival",
        "max-dead-rising",
        "rubygems-notice",
        "medium-article",
    ];

    for email_name in email_names {
        let path = format!("examples/problematic-emails/{}.json", email_name);
        println!("Testing email file: {}", path);

        // File should exist
        assert!(
            fs::metadata(&path).is_ok(),
            "Email file {} should exist",
            path
        );

        // File should be readable
        let content =
            fs::read_to_string(&path).unwrap_or_else(|_| panic!("Should be able to read {}", path));
        assert!(
            !content.is_empty(),
            "Email file {} should not be empty",
            path
        );

        // Should be valid JSON in notmuch format - expecting [[[message, []]]]
        let result: Result<Thread, _> = serde_json::from_str(&content);
        assert!(
            result.is_ok(),
            "Email file {} should contain valid notmuch JSON: {:?}",
            path,
            result.err()
        );

        let thread = result.unwrap();
        let messages = thread.get_messages();
        assert!(
            !messages.is_empty(),
            "Email file {} should contain at least one message",
            path
        );

        println!("✓ {} is valid", email_name);
    }
}

#[test]
fn test_email_content_rendering() {
    // Test that we can render content from the bilprovningen email
    let path = "examples/problematic-emails/bilprovningen.json";
    let content = fs::read_to_string(path).expect("Should be able to read bilprovningen.json");

    let thread: Thread = serde_json::from_str(&content).expect("Should parse bilprovningen.json");

    let messages = thread.get_messages();
    let message = messages.first().expect("Should have at least one message");

    // Test content rendering
    let rendered = render_message_content(message);

    // Should have HTML content since this is an HTML email
    assert!(
        rendered.has_html(),
        "Bilprovningen email should have HTML content"
    );

    let html = rendered.html.as_ref().expect("Should have HTML content");
    assert!(!html.is_empty(), "HTML content should not be empty");

    // Should contain expected anonymized text from the email
    assert!(
        html.contains("Car Service blir Cykelprovningen"),
        "Should contain the main heading text"
    );

    println!("✓ Email content rendering works");
    println!("  HTML content length: {} characters", html.len());
    println!("  First 100 chars: {}", &html[..100.min(html.len())]);
}

#[test]
fn test_all_emails_render_successfully() {
    // Test that all problematic emails can be rendered without errors
    let email_names = vec![
        "bilprovningen",
        "stockholm-film-festival",
        "max-dead-rising",
        "rubygems-notice",
        "medium-article",
    ];

    for email_name in email_names {
        let path = format!("examples/problematic-emails/{}.json", email_name);
        let content =
            fs::read_to_string(&path).unwrap_or_else(|_| panic!("Should read {}", email_name));

        let thread: Thread = serde_json::from_str(&content)
            .unwrap_or_else(|_| panic!("Should parse {}", email_name));

        let messages = thread.get_messages();
        let message = messages
            .first()
            .unwrap_or_else(|| panic!("Should have message in {}", email_name));

        // Test rendering doesn't panic
        let rendered = render_message_content(message);

        // Should have some form of content
        assert!(
            rendered.has_html() || rendered.has_plain(),
            "Email {} should have either HTML or plain text content",
            email_name
        );

        if rendered.has_html() {
            let html = rendered.html.as_ref().unwrap();
            assert!(
                !html.is_empty(),
                "HTML content in {} should not be empty",
                email_name
            );

            // Basic sanitization check - should not contain script tags
            assert!(
                !html.contains("<script"),
                "HTML content in {} should not contain script tags",
                email_name
            );
        }

        println!("✓ {} renders successfully", email_name);
    }
}

#[test]
fn test_email_subject_extraction() {
    // Test that we can extract subjects from all emails
    let expected_subjects = vec![
        ("bilprovningen", "Car Service blir Cykelprovningen"),
        (
            "stockholm-film-festival",
            "2 DAGAR KVAR! Nordens största Drive-In bio",
        ),
        ("max-dead-rising", "Varning: De döda har återuppstått!"),
        ("rubygems-notice", "Review New GemRegistry.org Policies"),
        ("medium-article", "Why we love puzzles"),
    ];

    for (email_name, expected_subject) in expected_subjects {
        let path = format!("examples/problematic-emails/{}.json", email_name);
        let content =
            fs::read_to_string(&path).unwrap_or_else(|_| panic!("Should read {}", email_name));

        let thread: Thread = serde_json::from_str(&content)
            .unwrap_or_else(|_| panic!("Should parse {}", email_name));

        let messages = thread.get_messages();
        let message = messages
            .first()
            .unwrap_or_else(|| panic!("Should have message in {}", email_name));

        let subject = &message.headers.subject;

        // Check if subject contains the expected text (partial match since full subjects may vary)
        assert!(
            subject.as_deref().unwrap_or("").contains(expected_subject)
                || subject
                    .as_deref()
                    .unwrap_or("")
                    .to_lowercase()
                    .contains(&expected_subject.to_lowercase()),
            "Subject in {} should contain '{}', but got '{}'",
            email_name,
            expected_subject,
            subject.as_deref().unwrap_or("(No subject)")
        );

        println!(
            "✓ {} has subject: '{}'",
            email_name,
            subject.as_deref().unwrap_or("(No subject)")
        );
    }
}
