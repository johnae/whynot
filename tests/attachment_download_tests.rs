#![cfg(feature = "test-utils")]

use std::net::SocketAddr;
use whynot::test_utils::mbox::{EmailMessage, MboxBuilder};
use whynot::test_utils::notmuch::TestNotmuch;
use whynot::web::{AppState, WebConfig, create_app};

async fn spawn_test_server_with_notmuch(test_notmuch: TestNotmuch) -> (SocketAddr, TestNotmuch) {
    let config = WebConfig {
        bind_address: ([127, 0, 0, 1], 0).into(),
        base_url: "http://localhost".to_string(),
        items_per_page: 10,
        auto_refresh_interval: 30,
        initial_page_size: 20,
        pagination_size: 10,
        infinite_scroll_enabled: true,
    };

    let state = AppState {
        mail_sender: None,
        user_config: whynot::config::UserConfig::default(),
        client: std::sync::Arc::from(test_notmuch.client()),
        config,
    };

    let app = create_app(state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();

    tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    });

    tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
    (addr, test_notmuch)
}

#[tokio::test]
async fn test_attachment_download_endpoints_fail() {
    let test_notmuch = TestNotmuch::new().await.unwrap();

    // Create an email with a simple text attachment
    let attachment_content =
        b"This is a test document.\nIt has multiple lines.\nAnd should download correctly.";

    let email_with_attachment = EmailMessage::new("Test with Text Attachment")
        .with_from("sender@example.com")
        .with_to(vec!["recipient@example.com".to_string()])
        .with_body("Please find the attached document.")
        .with_attachment("document.txt", "text/plain", attachment_content);

    let mbox = MboxBuilder::new()
        .add_message(email_with_attachment)
        .build();

    test_notmuch.add_mbox(&mbox).await.unwrap();

    let (addr, _test_notmuch) = spawn_test_server_with_notmuch(test_notmuch).await;

    // Get the thread ID
    let client = reqwest::Client::new();
    let response = client
        .get(format!("http://{}/inbox", addr))
        .send()
        .await
        .unwrap();
    let inbox_body = response.text().await.unwrap();

    // Extract thread ID
    let thread_url_pattern = r#"/thread/([a-f0-9]+)"#;
    let re = regex::Regex::new(thread_url_pattern).unwrap();
    let thread_id = re
        .captures(&inbox_body)
        .and_then(|caps| caps.get(1))
        .map(|m| m.as_str())
        .expect("Should find thread ID in inbox");

    // Load thread view to get attachment links
    let response = client
        .get(format!("http://{}/thread/{}", addr, thread_id))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), 200);
    let thread_body = response.text().await.unwrap();

    // Verify attachment is shown in the UI
    assert!(
        thread_body.contains("Attachments"),
        "Should show attachments section"
    );
    assert!(
        thread_body.contains("document.txt"),
        "Should show attachment filename"
    );
    assert!(
        thread_body.contains("text/plain"),
        "Should show attachment type"
    );

    // Extract attachment download link
    let attachment_link_pattern = r#"/attachment/([^"]+)"#;
    let re = regex::Regex::new(attachment_link_pattern).unwrap();
    let attachment_url = re
        .captures(&thread_body)
        .and_then(|caps| caps.get(1))
        .map(|m| m.as_str())
        .expect("Should find attachment download link");

    let full_attachment_url = format!("http://{}/attachment/{}", addr, attachment_url);

    // This test should FAIL initially because attachment downloads don't work properly

    // Try to download the attachment
    let download_response = client.get(&full_attachment_url).send().await.unwrap();

    // EXPECTED TO FAIL: Attachment download should work
    assert_eq!(
        download_response.status(),
        200,
        "Attachment download should return 200 OK. \
         This test should fail initially until attachment downloads are fixed."
    );

    // EXPECTED TO FAIL: Should have proper content-type header
    let content_type = download_response.headers().get("content-type");
    assert!(
        content_type.is_some() && content_type.unwrap() == "text/plain",
        "Attachment should have correct content-type header. \
         This test should fail initially until headers are fixed."
    );

    // EXPECTED TO FAIL: Should have proper content-disposition header
    let content_disposition = download_response.headers().get("content-disposition");
    assert!(
        content_disposition.is_some()
            && content_disposition
                .unwrap()
                .to_str()
                .unwrap()
                .contains("document.txt"),
        "Attachment should have correct content-disposition header with filename. \
         This test should fail initially until headers are fixed."
    );

    // EXPECTED TO FAIL: Content should match original attachment
    let downloaded_content = download_response.bytes().await.unwrap();
    assert_eq!(
        downloaded_content.as_ref(),
        attachment_content,
        "Downloaded content should match original attachment. \
         This test should fail initially until content extraction is fixed."
    );
}

#[tokio::test]
async fn test_binary_attachment_download_fails() {
    let test_notmuch = TestNotmuch::new().await.unwrap();

    // Create an email with a binary attachment (simulated PDF)
    let pdf_content = b"%PDF-1.4\n1 0 obj\n<<\n/Type /Catalog\n/Pages 2 0 R\n>>\nendobj\n\nThis is fake PDF content for testing";

    let email_with_pdf = EmailMessage::new("Test with PDF Attachment")
        .with_from("sender@example.com")
        .with_to(vec!["recipient@example.com".to_string()])
        .with_body("Please find the attached PDF document.")
        .with_attachment("report.pdf", "application/pdf", pdf_content);

    let mbox = MboxBuilder::new().add_message(email_with_pdf).build();

    test_notmuch.add_mbox(&mbox).await.unwrap();

    let (addr, _test_notmuch) = spawn_test_server_with_notmuch(test_notmuch).await;

    // Get thread and attachment URL (similar process as above)
    let client = reqwest::Client::new();
    let response = client
        .get(format!("http://{}/inbox", addr))
        .send()
        .await
        .unwrap();
    let inbox_body = response.text().await.unwrap();

    let thread_url_pattern = r#"/thread/([a-f0-9]+)"#;
    let re = regex::Regex::new(thread_url_pattern).unwrap();
    let thread_id = re
        .captures(&inbox_body)
        .and_then(|caps| caps.get(1))
        .map(|m| m.as_str())
        .expect("Should find thread ID");

    let response = client
        .get(format!("http://{}/thread/{}", addr, thread_id))
        .send()
        .await
        .unwrap();
    let thread_body = response.text().await.unwrap();

    // Verify PDF attachment is shown
    assert!(
        thread_body.contains("report.pdf"),
        "Should show PDF filename"
    );
    assert!(
        thread_body.contains("application/pdf"),
        "Should show PDF mime type"
    );
    assert!(thread_body.contains("📄"), "Should show PDF icon");

    // Extract attachment download link
    let attachment_link_pattern = r#"/attachment/([^"]+)"#;
    let re = regex::Regex::new(attachment_link_pattern).unwrap();
    let attachment_url = re
        .captures(&thread_body)
        .and_then(|caps| caps.get(1))
        .map(|m| m.as_str())
        .expect("Should find attachment download link");

    let full_attachment_url = format!("http://{}/attachment/{}", addr, attachment_url);

    // Test binary attachment downloads work correctly

    let download_response = client.get(&full_attachment_url).send().await.unwrap();

    // Binary attachment download should work
    assert_eq!(
        download_response.status(),
        200,
        "Binary attachment download should return 200 OK"
    );

    // Should have correct PDF mime type
    let content_type = download_response.headers().get("content-type");
    assert!(
        content_type.is_some() && content_type.unwrap() == "application/pdf",
        "PDF attachment should have correct content-type"
    );

    // EXPECTED TO FAIL: Should have security headers
    let headers = download_response.headers();
    assert!(
        headers.get("x-content-type-options").is_some() && headers.get("x-frame-options").is_some(),
        "Attachment downloads should have security headers. \
         This test should fail initially until security headers are added."
    );

    // EXPECTED TO FAIL: Binary content should not be corrupted
    let downloaded_content = download_response.bytes().await.unwrap();
    assert_eq!(
        downloaded_content.as_ref(),
        pdf_content,
        "Downloaded binary content should match original without corruption. \
         This test should fail initially until base64 decoding is fixed."
    );
}

#[tokio::test]
async fn test_multiple_attachments_download_fails() {
    let test_notmuch = TestNotmuch::new().await.unwrap();

    // Create an email with multiple attachments of different types
    let text_content = b"This is a text file content.";
    let csv_content = b"Name,Age,City\nJohn,30,NYC\nJane,25,LA";
    let image_content = b"\x89PNG\r\n\x1a\n\x00\x00\x00\rIHDR\x00\x00\x00\x01\x00\x00\x00\x01"; // Fake PNG header

    let mut email_builder = EmailMessage::new("Test with Multiple Attachments")
        .with_from("sender@example.com")
        .with_to(vec!["recipient@example.com".to_string()])
        .with_body("This email has multiple attachments of different types.");

    // Add multiple attachments
    email_builder = email_builder
        .with_attachment("readme.txt", "text/plain", text_content)
        .with_attachment("data.csv", "text/csv", csv_content)
        .with_attachment("icon.png", "image/png", image_content);

    let mbox = MboxBuilder::new().add_message(email_builder).build();

    test_notmuch.add_mbox(&mbox).await.unwrap();

    let (addr, _test_notmuch) = spawn_test_server_with_notmuch(test_notmuch).await;

    // Get thread view
    let client = reqwest::Client::new();
    let response = client
        .get(format!("http://{}/inbox", addr))
        .send()
        .await
        .unwrap();
    let inbox_body = response.text().await.unwrap();

    let thread_url_pattern = r#"/thread/([a-f0-9]+)"#;
    let re = regex::Regex::new(thread_url_pattern).unwrap();
    let thread_id = re
        .captures(&inbox_body)
        .and_then(|caps| caps.get(1))
        .map(|m| m.as_str())
        .expect("Should find thread ID");

    let response = client
        .get(format!("http://{}/thread/{}", addr, thread_id))
        .send()
        .await
        .unwrap();
    let thread_body = response.text().await.unwrap();

    // Verify all attachments are shown
    assert!(thread_body.contains("readme.txt"), "Should show text file");
    assert!(thread_body.contains("data.csv"), "Should show CSV file");
    assert!(thread_body.contains("icon.png"), "Should show PNG file");

    // EXPECTED TO FAIL: All attachment types should be downloadable
    let attachment_links: Vec<&str> = regex::Regex::new(r#"/attachment/([^"]+)"#)
        .unwrap()
        .captures_iter(&thread_body)
        .filter_map(|caps| caps.get(1))
        .map(|m| m.as_str())
        .collect();

    assert_eq!(
        attachment_links.len(),
        3,
        "Should find download links for all 3 attachments. \
         This test should fail initially until multiple attachment handling is fixed."
    );

    // Test downloading each attachment
    for (i, attachment_url) in attachment_links.iter().enumerate() {
        let full_url = format!("http://{}/attachment/{}", addr, attachment_url);
        let download_response = client.get(&full_url).send().await.unwrap();

        // EXPECTED TO FAIL: Each attachment should download successfully
        assert_eq!(
            download_response.status(),
            200,
            "Attachment {} should download successfully. \
             This test should fail initially until attachment indexing is fixed.",
            i + 1
        );

        // EXPECTED TO FAIL: Each should have correct content type
        let content_type = download_response.headers().get("content-type");
        assert!(
            content_type.is_some(),
            "Attachment {} should have content-type header. \
             This test should fail initially until headers are fixed.",
            i + 1
        );
    }
}

#[tokio::test]
async fn test_attachment_filename_sanitization_fails() {
    let test_notmuch = TestNotmuch::new().await.unwrap();

    // Create an email with dangerous filename
    let dangerous_content = b"This file has a dangerous name.";

    let email_with_dangerous_filename = EmailMessage::new("Test with Dangerous Filename")
        .with_from("attacker@example.com")
        .with_to(vec!["victim@example.com".to_string()])
        .with_body("This email has an attachment with a dangerous filename.")
        .with_attachment("../../../etc/passwd", "text/plain", dangerous_content);

    let mbox = MboxBuilder::new()
        .add_message(email_with_dangerous_filename)
        .build();

    test_notmuch.add_mbox(&mbox).await.unwrap();

    let (addr, _test_notmuch) = spawn_test_server_with_notmuch(test_notmuch).await;

    // Get thread view and download link
    let client = reqwest::Client::new();
    let response = client
        .get(format!("http://{}/inbox", addr))
        .send()
        .await
        .unwrap();
    let inbox_body = response.text().await.unwrap();

    let thread_url_pattern = r#"/thread/([a-f0-9]+)"#;
    let re = regex::Regex::new(thread_url_pattern).unwrap();
    let thread_id = re
        .captures(&inbox_body)
        .and_then(|caps| caps.get(1))
        .map(|m| m.as_str())
        .expect("Should find thread ID");

    let response = client
        .get(format!("http://{}/thread/{}", addr, thread_id))
        .send()
        .await
        .unwrap();
    let thread_body = response.text().await.unwrap();

    // Extract download link
    let attachment_link_pattern = r#"/attachment/([^"]+)"#;
    let re = regex::Regex::new(attachment_link_pattern).unwrap();
    let attachment_url = re
        .captures(&thread_body)
        .and_then(|caps| caps.get(1))
        .map(|m| m.as_str())
        .expect("Should find attachment download link");

    let full_url = format!("http://{}/attachment/{}", addr, attachment_url);

    let download_response = client.get(&full_url).send().await.unwrap();

    // EXPECTED TO FAIL: Download should work despite dangerous filename
    assert_eq!(
        download_response.status(),
        200,
        "Download should work even with dangerous filename. \
         This test should fail initially until filename sanitization is improved."
    );

    // EXPECTED TO FAIL: Filename should be sanitized in content-disposition header
    let content_disposition = download_response.headers().get("content-disposition");
    if let Some(disposition) = content_disposition {
        let disposition_str = disposition.to_str().unwrap();
        assert!(
            !disposition_str.contains("../") && !disposition_str.contains("/etc/passwd"),
            "Filename should be sanitized to prevent path traversal. \
             This test should fail initially until proper sanitization is implemented. \
             Got: {}",
            disposition_str
        );

        // Should have a safe filename
        assert!(
            disposition_str.contains("filename=")
                && (disposition_str.contains("passwd") || disposition_str.contains("attachment")),
            "Should have a safe filename without path traversal components. \
             This test should fail initially until sanitization is implemented."
        );
    } else {
        panic!("Content-disposition header should be present for attachment downloads");
    }
}
