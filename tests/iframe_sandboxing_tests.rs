#![cfg(feature = "test-utils")]

use scraper::{Html, Selector};
use std::net::SocketAddr;
use whynot::test_utils::mbox::{EmailMessage, MboxBuilder};
use whynot::test_utils::notmuch::TestNotmuch;
use whynot::web::{AppState, WebConfig, create_app};

async fn spawn_test_server_with_notmuch(test_notmuch: TestNotmuch) -> (SocketAddr, TestNotmuch) {
    let config = WebConfig {
        bind_address: ([127, 0, 0, 1], 0).into(),
        base_url: "http://localhost".to_string(),
        items_per_page: 10,
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
async fn test_email_content_renders_in_iframe() {
    let test_notmuch = TestNotmuch::new().await.unwrap();

    // Create test email with HTML content
    let mbox = MboxBuilder::new()
        .add_message(
            EmailMessage::new("Test HTML Email")
                .with_from("test@example.com")
                .with_to(vec!["recipient@example.com".to_string()])
                .with_html_body("<h1>Test HTML Email</h1><p>This is HTML content</p>"),
        )
        .build();

    test_notmuch.add_mbox(&mbox).await.unwrap();

    let (addr, _test_notmuch) = spawn_test_server_with_notmuch(test_notmuch).await;

    // Get the thread ID from inbox
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

    // Fetch thread page
    let thread_response = client
        .get(format!("http://{}/thread/{}", addr, thread_id))
        .send()
        .await
        .unwrap();
    let thread_html = thread_response.text().await.unwrap();
    let thread_doc = Html::parse_document(&thread_html);

    // Check for iframe element
    let iframe_selector = Selector::parse("iframe.email-content-frame").unwrap();
    let iframe = thread_doc.select(&iframe_selector).next();
    assert!(
        iframe.is_some(),
        "Email content should be rendered in an iframe"
    );

    // Check iframe attributes
    let iframe_elem = iframe.unwrap();
    let sandbox_attr = iframe_elem.value().attr("sandbox");
    assert!(
        sandbox_attr.is_some(),
        "Iframe should have sandbox attribute"
    );
    assert_eq!(
        sandbox_attr.unwrap(),
        "allow-same-origin allow-popups allow-popups-to-escape-sandbox",
        "Iframe should have sandbox policy allowing user-initiated link navigation"
    );

    let src_attr = iframe_elem.value().attr("src");
    assert!(src_attr.is_some(), "Iframe should have src attribute");
    assert!(
        src_attr.unwrap().contains("/email-frame/"),
        "Iframe src should point to email-frame endpoint"
    );
}

#[tokio::test]
async fn test_iframe_content_endpoint_serves_sanitized_html() {
    let test_notmuch = TestNotmuch::new().await.unwrap();

    // Create test email with potentially dangerous HTML
    let mbox = MboxBuilder::new()
        .add_message(
            EmailMessage::new("Test Subject")
                .with_from("test@example.com")
                .with_to(vec!["recipient@example.com".to_string()])
                .with_html_body(
                    r#"
                    <h1>Test Email</h1>
                    <script>alert('XSS')</script>
                    <style>body { background: red; }</style>
                    <p style="color: blue;">Safe content</p>
                    <img src="http://external.com/image.jpg" />
                    <a href="http://phishing.com">Click me</a>
                "#,
                ),
        )
        .build();

    test_notmuch.add_mbox(&mbox).await.unwrap();

    let (addr, _test_notmuch) = spawn_test_server_with_notmuch(test_notmuch).await;

    // Get thread ID
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
        .expect("Should find thread ID in inbox");

    // Fetch email frame content directly
    let frame_url = format!("http://{}/email-frame/{}/0", addr, thread_id);
    let frame_response = client.get(&frame_url).send().await.unwrap();
    let frame_html = frame_response.text().await.unwrap();
    let frame_doc = Html::parse_document(&frame_html);

    // Verify no script tags
    let script_selector = Selector::parse("script").unwrap();
    assert_eq!(
        frame_doc.select(&script_selector).count(),
        0,
        "Script tags should be removed"
    );

    // Verify safe content is preserved
    let p_selector = Selector::parse("p").unwrap();
    let p_elem = frame_doc
        .select(&p_selector)
        .next()
        .expect("P tag should be preserved");
    assert_eq!(p_elem.text().collect::<String>(), "Safe content");

    // Verify images are rewritten to proxy
    let img_selector = Selector::parse("img").unwrap();
    let img_elem = frame_doc
        .select(&img_selector)
        .next()
        .expect("Image should be preserved");
    let img_src = img_elem.value().attr("src").unwrap();
    assert!(
        img_src.starts_with("/image_proxy?url="),
        "Image src should be rewritten to proxy"
    );

    // Verify links are rewritten
    let a_selector = Selector::parse("a").unwrap();
    let a_elem = frame_doc
        .select(&a_selector)
        .next()
        .expect("Link should be preserved");
    let a_href = a_elem.value().attr("href").unwrap();
    assert!(
        a_href.starts_with("/redirect?url="),
        "Links should be rewritten to redirect endpoint"
    );
}

#[tokio::test]
async fn test_iframe_has_csp_headers() {
    let test_notmuch = TestNotmuch::new().await.unwrap();

    let mbox = MboxBuilder::new()
        .add_message(
            EmailMessage::new("Test Subject")
                .with_from("test@example.com")
                .with_to(vec!["recipient@example.com".to_string()])
                .with_html_body("<p>Test content</p>"),
        )
        .build();

    test_notmuch.add_mbox(&mbox).await.unwrap();

    let (addr, _test_notmuch) = spawn_test_server_with_notmuch(test_notmuch).await;

    // Get thread ID
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
        .expect("Should find thread ID in inbox");

    // Fetch email frame and check headers
    let frame_url = format!("http://{}/email-frame/{}/0", addr, thread_id);
    let frame_response = client.get(&frame_url).send().await.unwrap();

    // Check CSP header
    let csp_header = frame_response.headers().get("Content-Security-Policy");
    assert!(csp_header.is_some(), "Email frame should have CSP header");

    let csp_value = csp_header.unwrap().to_str().unwrap();
    assert!(
        csp_value.contains("script-src 'none'"),
        "CSP should block all scripts"
    );
    assert!(
        csp_value.contains("img-src 'self'"),
        "CSP should restrict image sources"
    );
}

#[tokio::test]
async fn test_email_styles_isolated_in_iframe() {
    let test_notmuch = TestNotmuch::new().await.unwrap();

    // Create email with styles that would affect parent if not isolated
    let mbox = MboxBuilder::new()
        .add_message(
            EmailMessage::new("Test Subject")
                .with_from("test@example.com")
                .with_to(vec!["recipient@example.com".to_string()])
                .with_html_body(
                    r#"
                    <style>
                        body { background: red !important; }
                        .email-content { display: none !important; }
                    </style>
                    <p>Email content</p>
                "#,
                ),
        )
        .build();

    test_notmuch.add_mbox(&mbox).await.unwrap();

    let (addr, _test_notmuch) = spawn_test_server_with_notmuch(test_notmuch).await;

    // Get thread ID
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
        .expect("Should find thread ID in inbox");

    // Fetch thread page
    let thread_response = client
        .get(format!("http://{}/thread/{}", addr, thread_id))
        .send()
        .await
        .unwrap();
    let thread_html = thread_response.text().await.unwrap();

    // Verify parent page styles are not affected
    assert!(
        !thread_html.contains("background: red"),
        "Parent page should not have red background"
    );

    // Verify iframe exists and contains the email
    let thread_doc = Html::parse_document(&thread_html);
    let iframe_selector = Selector::parse("iframe.email-content-frame").unwrap();
    let iframe = thread_doc.select(&iframe_selector).next();
    assert!(iframe.is_some(), "Email should be in iframe");
}

#[tokio::test]
async fn test_iframe_prevents_javascript_execution() {
    let test_notmuch = TestNotmuch::new().await.unwrap();

    // Create email with various JavaScript attempts
    let mbox = MboxBuilder::new()
        .add_message(
            EmailMessage::new("Test Subject")
                .with_from("test@example.com")
                .with_to(vec!["recipient@example.com".to_string()])
                .with_html_body(
                    r#"
                    <script>window.parent.location = 'http://evil.com';</script>
                    <img src="x" onerror="alert('XSS')" />
                    <a href="javascript:alert('XSS')">Click</a>
                    <div onmouseover="alert('XSS')">Hover me</div>
                "#,
                ),
        )
        .build();

    test_notmuch.add_mbox(&mbox).await.unwrap();

    let (addr, _test_notmuch) = spawn_test_server_with_notmuch(test_notmuch).await;

    // Get thread ID
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
        .expect("Should find thread ID in inbox");

    // Fetch email frame content
    let frame_url = format!("http://{}/email-frame/{}/0", addr, thread_id);
    let frame_response = client.get(&frame_url).send().await.unwrap();
    let frame_html = frame_response.text().await.unwrap();

    // Verify no script tags
    assert!(
        !frame_html.contains("<script"),
        "Script tags should be removed"
    );

    // Verify event handlers are removed
    assert!(
        !frame_html.contains("onerror="),
        "Event handlers should be removed"
    );
    assert!(
        !frame_html.contains("onmouseover="),
        "Event handlers should be removed"
    );

    // Verify javascript: URLs are sanitized
    assert!(
        !frame_html.contains("javascript:"),
        "JavaScript URLs should be removed"
    );
}
