use reqwest::Client;
use scraper::{Html, Selector};
use std::net::SocketAddr;
use whynot::client::{ClientConfig, create_client};
use whynot::test_utils::{EmailMessage, MboxBuilder, TestNotmuch};
use whynot::web::{AppState, WebConfig, create_app};

/// Test suite demonstrating HTML rendering and CSS isolation issues
///
/// These tests should fail initially, demonstrating the problems described in TODO.md:
/// 1. Overly restrictive CSS isolation
/// 2. Multiple visual frames around content
/// 3. Layout properties being reset too aggressively
/// 4. Content not displaying as intended by sender

async fn spawn_test_server_with_notmuch(test_notmuch: &TestNotmuch) -> SocketAddr {
    let client_config = ClientConfig::Local {
        notmuch_path: None,
        database_path: Some(test_notmuch.database_path().clone()),
        mail_root: None,
    };

    let client = create_client(client_config).unwrap();
    let config = WebConfig {
        bind_address: ([127, 0, 0, 1], 0).into(),
        base_url: "http://localhost".to_string(),
        items_per_page: 10,
    };

    let state = AppState {
        mail_sender: None,
        user_config: whynot::config::UserConfig::default(),
        client: std::sync::Arc::from(client),
        config,
    };

    let app = create_app(state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();

    tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    });

    // Give the server time to start
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

    addr
}

#[tokio::test]
async fn test_basic_css_properties_are_preserved() {
    let test_notmuch = TestNotmuch::new().await.unwrap();

    // Create a simple email with basic CSS that should be preserved
    let html_content = r#"
        <div style="color: red; font-size: 18px; padding: 10px; background-color: lightblue;">
            <h1 style="color: blue; margin: 0;">Important Message</h1>
            <p style="font-weight: bold; line-height: 1.5;">This text should be bold and have proper line spacing.</p>
            <table style="border: 1px solid black; width: 100%;">
                <tr style="background-color: yellow;">
                    <td style="padding: 5px;">Cell 1</td>
                    <td style="padding: 5px;">Cell 2</td>
                </tr>
            </table>
        </div>
    "#;

    let email = EmailMessage::new("CSS Properties Test").with_html_body(html_content);

    let mbox = MboxBuilder::new().add_message(email).build();

    test_notmuch.add_mbox(&mbox).await.unwrap();
    let addr = spawn_test_server_with_notmuch(&test_notmuch).await;

    let client = Client::new();

    // Get search results
    let response = client
        .get(format!("http://{}/search?q=tag:inbox", addr))
        .send()
        .await
        .unwrap();

    let html_content = response.text().await.unwrap();
    let document = Html::parse_document(&html_content);

    // Find the thread link
    let thread_link_selector = Selector::parse("a[href*='/thread/']").unwrap();
    let thread_link = document
        .select(&thread_link_selector)
        .next()
        .expect("Should find a thread link");

    let href = thread_link.value().attr("href").unwrap();
    let thread_url = format!("http://{}{}", addr, href);

    // Get thread view
    let thread_response = client.get(&thread_url).send().await.unwrap();
    let thread_html = thread_response.text().await.unwrap();
    let thread_document = Html::parse_document(&thread_html);

    // Check for iframe-based email rendering
    let iframe_selector = Selector::parse("iframe.email-content-frame").unwrap();
    let iframe = thread_document.select(&iframe_selector).next();

    let email_doc = if let Some(iframe_elem) = iframe {
        // New iframe-based architecture - fetch iframe content
        let src_attr = iframe_elem.value().attr("src").unwrap();
        let iframe_url = format!("http://{}{}", addr, src_attr);
        let iframe_response = client.get(&iframe_url).send().await.unwrap();
        let iframe_html = iframe_response.text().await.unwrap();
        Html::parse_document(&iframe_html)
    } else {
        // Legacy direct HTML rendering
        let email_content_selector = Selector::parse(".email-content").unwrap();
        let email_content = thread_document
            .select(&email_content_selector)
            .next()
            .expect("Should find email content container");

        let content_html = email_content.inner_html();
        Html::parse_document(&content_html)
    };

    // Test 1: Basic color properties should be preserved
    let colored_elements_selector = Selector::parse("[style*='color']").unwrap();
    let colored_elements: Vec<_> = email_doc.select(&colored_elements_selector).collect();

    // This test may fail if CSS isolation is too aggressive
    assert!(
        !colored_elements.is_empty(),
        "EXPECTED FAILURE: Color properties should be preserved for proper email display"
    );

    // Test 2: Layout properties like padding should be preserved
    let padded_elements_selector = Selector::parse("[style*='padding']").unwrap();
    let padded_elements: Vec<_> = email_doc.select(&padded_elements_selector).collect();

    assert!(
        !padded_elements.is_empty(),
        "EXPECTED FAILURE: Padding properties should be preserved for proper spacing"
    );

    // Test 3: Table structure and styling should be maintained
    let table_selector = Selector::parse("table").unwrap();
    let tables: Vec<_> = email_doc.select(&table_selector).collect();
    assert!(!tables.is_empty(), "Table structure should be preserved");

    let table_borders_selector = Selector::parse("table[style*='border']").unwrap();
    let bordered_tables: Vec<_> = email_doc.select(&table_borders_selector).collect();

    assert!(
        !bordered_tables.is_empty(),
        "EXPECTED FAILURE: Table borders should be preserved for data presentation"
    );
}

#[tokio::test]
async fn test_wrapper_container_count() {
    let test_notmuch = TestNotmuch::new().await.unwrap();

    // Create simple content that shouldn't need many wrappers
    let html_content = r#"
        <div class="simple-email">
            <h1>Simple Email</h1>
            <p>This should not have excessive wrapping divs around it.</p>
        </div>
    "#;

    let email = EmailMessage::new("Wrapper Container Test").with_html_body(html_content);

    let mbox = MboxBuilder::new().add_message(email).build();

    test_notmuch.add_mbox(&mbox).await.unwrap();
    let addr = spawn_test_server_with_notmuch(&test_notmuch).await;

    let client = Client::new();

    // Navigate to thread view
    let response = client
        .get(format!("http://{}/search?q=tag:inbox", addr))
        .send()
        .await
        .unwrap();

    let html_content = response.text().await.unwrap();
    let document = Html::parse_document(&html_content);

    let thread_link_selector = Selector::parse("a[href*='/thread/']").unwrap();
    let thread_link = document
        .select(&thread_link_selector)
        .next()
        .expect("Should find a thread link");

    let href = thread_link.value().attr("href").unwrap();
    let thread_url = format!("http://{}{}", addr, href);

    let thread_response = client.get(&thread_url).send().await.unwrap();
    let thread_html = thread_response.text().await.unwrap();

    // Test for excessive wrapper containers that create visual frames
    let wrapper_selectors = [
        ".email-content div",
        ".email-content > div",
        ".email-content div[class]",
        ".email-content div[style]",
    ];

    let thread_document = Html::parse_document(&thread_html);

    for selector_str in wrapper_selectors {
        let selector = Selector::parse(selector_str).unwrap();
        let wrapper_count = thread_document.select(&selector).count();

        // This test may fail if there are too many wrapper containers
        assert!(
            wrapper_count <= 5,
            "EXPECTED FAILURE: Found {} wrapper containers with selector '{}'. Too many wrappers may create visual frames.",
            wrapper_count,
            selector_str
        );
    }

    // Test for CSS that might create visual frames
    let frame_creating_css = [
        "[style*='border']",
        "[style*='box-shadow']",
        "[style*='outline']",
    ];

    for css_selector in frame_creating_css {
        let selector = Selector::parse(css_selector).unwrap();
        let frame_elements = thread_document.select(&selector).count();

        // This might indicate multiple visual frames being created
        if frame_elements > 3 {
            eprintln!(
                "WARNING: Found {} elements with frame-creating CSS: {}",
                frame_elements, css_selector
            );
        }
    }
}

#[tokio::test]
async fn test_css_reset_impact() {
    let test_notmuch = TestNotmuch::new().await.unwrap();

    // Create content with layout-critical CSS properties
    let html_content = r#"
        <div style="display: flex; justify-content: space-between; gap: 20px;">
            <div style="flex: 1; background-color: lightgray; padding: 15px;">
                <h2 style="font-size: 20px; margin: 0;">Left Column</h2>
                <p style="line-height: 1.6;">Content in the left column</p>
            </div>
            <div style="flex: 1; background-color: lightblue; padding: 15px;">
                <h2 style="font-size: 20px; margin: 0;">Right Column</h2>
                <p style="line-height: 1.6;">Content in the right column</p>
            </div>
        </div>
    "#;

    let email = EmailMessage::new("CSS Reset Impact Test").with_html_body(html_content);

    let mbox = MboxBuilder::new().add_message(email).build();

    test_notmuch.add_mbox(&mbox).await.unwrap();
    let addr = spawn_test_server_with_notmuch(&test_notmuch).await;

    let client = Client::new();

    // Navigate to thread view
    let response = client
        .get(format!("http://{}/search?q=tag:inbox", addr))
        .send()
        .await
        .unwrap();

    let html_content = response.text().await.unwrap();
    let document = Html::parse_document(&html_content);

    let thread_link_selector = Selector::parse("a[href*='/thread/']").unwrap();
    let thread_link = document
        .select(&thread_link_selector)
        .next()
        .expect("Should find a thread link");

    let href = thread_link.value().attr("href").unwrap();
    let thread_url = format!("http://{}{}", addr, href);

    let thread_response = client.get(&thread_url).send().await.unwrap();
    let thread_html = thread_response.text().await.unwrap();
    let thread_document = Html::parse_document(&thread_html);

    // Check for iframe-based email rendering
    let iframe_selector = Selector::parse("iframe.email-content-frame").unwrap();
    let iframe = thread_document.select(&iframe_selector).next();

    let email_doc = if let Some(iframe_elem) = iframe {
        // New iframe-based architecture - fetch iframe content
        let src_attr = iframe_elem.value().attr("src").unwrap();
        let iframe_url = format!("http://{}{}", addr, src_attr);
        let iframe_response = client.get(&iframe_url).send().await.unwrap();
        let iframe_html = iframe_response.text().await.unwrap();
        Html::parse_document(&iframe_html)
    } else {
        // Legacy direct HTML rendering
        let email_content_selector = Selector::parse(".email-content").unwrap();
        let email_content = thread_document
            .select(&email_content_selector)
            .next()
            .expect("Should find email content");

        let content_html = email_content.inner_html();
        Html::parse_document(&content_html)
    };

    // Test 1: Flexbox properties should be preserved for layout
    let flex_container_selector = Selector::parse("[style*='display: flex']").unwrap();
    let flex_containers: Vec<_> = email_doc.select(&flex_container_selector).collect();

    assert!(
        !flex_containers.is_empty(),
        "EXPECTED FAILURE: Flexbox display property should be preserved for proper layout"
    );

    // Test 2: Flex item properties should be preserved
    let flex_item_selector = Selector::parse("[style*='flex: 1']").unwrap();
    let flex_items: Vec<_> = email_doc.select(&flex_item_selector).collect();

    assert!(
        flex_items.len() >= 2,
        "EXPECTED FAILURE: Flex item properties should be preserved (expected 2 items, found {})",
        flex_items.len()
    );

    // Test 3: Background colors should be preserved for visual distinction
    let background_selector = Selector::parse("[style*='background-color']").unwrap();
    let background_elements: Vec<_> = email_doc.select(&background_selector).collect();

    assert!(
        !background_elements.is_empty(),
        "EXPECTED FAILURE: Background colors should be preserved for visual layout"
    );

    // Test 4: Typography should not be reset to browser defaults
    let font_size_selector = Selector::parse("[style*='font-size']").unwrap();
    let font_sized_elements: Vec<_> = email_doc.select(&font_size_selector).collect();

    assert!(
        !font_sized_elements.is_empty(),
        "EXPECTED FAILURE: Font sizes should be preserved for proper typography hierarchy"
    );
}

#[tokio::test]
async fn test_security_vs_fidelity_balance() {
    let test_notmuch = TestNotmuch::new().await.unwrap();

    // Create content with both legitimate styling and potentially dangerous content
    let html_content = r#"
        <div style="max-width: 600px; margin: 0 auto; font-family: Arial;">
            <h1 style="color: navy; text-align: center;">Legitimate Email Content</h1>
            <script>alert('This should be removed');</script>
            <p style="color: red; font-weight: bold;">Important notice text</p>
            <style>body { display: none; }</style>
            <div style="background: yellow; padding: 10px; border: 1px solid orange;">
                Warning box with legitimate styling
            </div>
            <iframe src="javascript:alert('bad')"></iframe>
        </div>
    "#;

    let email = EmailMessage::new("Security vs Fidelity Test").with_html_body(html_content);

    let mbox = MboxBuilder::new().add_message(email).build();

    test_notmuch.add_mbox(&mbox).await.unwrap();
    let addr = spawn_test_server_with_notmuch(&test_notmuch).await;

    let client = Client::new();

    // Navigate to thread view
    let response = client
        .get(format!("http://{}/search?q=tag:inbox", addr))
        .send()
        .await
        .unwrap();

    let html_content = response.text().await.unwrap();
    let document = Html::parse_document(&html_content);

    let thread_link_selector = Selector::parse("a[href*='/thread/']").unwrap();
    let thread_link = document
        .select(&thread_link_selector)
        .next()
        .expect("Should find a thread link");

    let href = thread_link.value().attr("href").unwrap();
    let thread_url = format!("http://{}{}", addr, href);

    let thread_response = client.get(&thread_url).send().await.unwrap();
    let thread_html = thread_response.text().await.unwrap();

    // Parse the HTML and check for iframe-based email rendering
    let thread_document = Html::parse_document(&thread_html);
    let email_content_selector = Selector::parse(".email-content").unwrap();
    let email_content = thread_document
        .select(&email_content_selector)
        .next()
        .expect("Should find email content");

    let email_html = email_content.inner_html();

    // Check that email is now rendered in an iframe (new architecture)
    let iframe_selector = Selector::parse("iframe.email-content-frame").unwrap();
    let iframe = thread_document.select(&iframe_selector).next();

    if iframe.is_some() {
        // New iframe-based architecture
        let iframe_elem = iframe.unwrap();
        let sandbox_attr = iframe_elem.value().attr("sandbox");
        assert!(
            sandbox_attr.is_some(),
            "Iframe should have sandbox attribute"
        );
        assert_eq!(
            sandbox_attr.unwrap(),
            "allow-same-origin allow-popups allow-popups-to-escape-sandbox",
            "Iframe should have proper sandbox with link navigation"
        );

        let src_attr = iframe_elem.value().attr("src").unwrap();
        assert!(
            src_attr.contains("/email-frame/"),
            "Iframe should point to email-frame endpoint"
        );

        // Fetch the iframe content to test sanitization
        let iframe_url = format!("http://{}{}", addr, src_attr);
        let iframe_response = client.get(&iframe_url).send().await.unwrap();
        let iframe_html = iframe_response.text().await.unwrap();

        // Security tests on iframe content
        assert!(
            !iframe_html.contains("<script"),
            "Script tags should be removed from iframe content"
        );
        assert!(
            !iframe_html.contains("alert("),
            "JavaScript should be removed from iframe content"
        );
        assert!(
            !iframe_html.contains("javascript:"),
            "JavaScript URLs should be removed from iframe content"
        );

        // Fidelity tests - check that legitimate styling is preserved in iframe
        let iframe_doc = Html::parse_document(&iframe_html);

        let colored_text_selector = Selector::parse("[style*='color: red']").unwrap();
        let colored_text: Vec<_> = iframe_doc.select(&colored_text_selector).collect();

        assert!(
            !colored_text.is_empty(),
            "EXPECTED FAILURE: Legitimate color styling should be preserved in iframe"
        );

        let warning_box_selector = Selector::parse("[style*='background: yellow']").unwrap();
        let warning_boxes: Vec<_> = iframe_doc.select(&warning_box_selector).collect();

        assert!(
            !warning_boxes.is_empty(),
            "EXPECTED FAILURE: Legitimate background styling should be preserved in iframe"
        );

        let bordered_elements_selector = Selector::parse("[style*='border']").unwrap();
        let bordered_elements: Vec<_> = iframe_doc.select(&bordered_elements_selector).collect();

        assert!(
            !bordered_elements.is_empty(),
            "EXPECTED FAILURE: Legitimate border styling should be preserved in iframe"
        );
    } else {
        // Legacy direct HTML rendering (fallback for compatibility)
        // Security tests - these should pass (dangerous content removed from EMAIL CONTENT only)
        assert!(
            !email_html.contains("<script"),
            "Script tags should be removed from email content"
        );
        assert!(
            !email_html.contains("alert("),
            "JavaScript should be removed from email content"
        );
        assert!(
            !email_html.contains("<iframe"),
            "Dangerous iframes should be removed from email content"
        );
        assert!(
            !email_html.contains("javascript:"),
            "JavaScript URLs should be removed from email content"
        );

        // Fidelity tests - these may fail if CSS isolation is too aggressive
        let email_doc = Html::parse_document(&email_html);

        // Legitimate styling should be preserved
        let colored_text_selector = Selector::parse("[style*='color: red']").unwrap();
        let colored_text: Vec<_> = email_doc.select(&colored_text_selector).collect();

        assert!(
            !colored_text.is_empty(),
            "EXPECTED FAILURE: Legitimate color styling should be preserved"
        );

        let warning_box_selector = Selector::parse("[style*='background: yellow']").unwrap();
        let warning_boxes: Vec<_> = email_doc.select(&warning_box_selector).collect();

        assert!(
            !warning_boxes.is_empty(),
            "EXPECTED FAILURE: Legitimate background styling should be preserved"
        );

        let bordered_elements_selector = Selector::parse("[style*='border']").unwrap();
        let bordered_elements: Vec<_> = email_doc.select(&bordered_elements_selector).collect();

        assert!(
            !bordered_elements.is_empty(),
            "EXPECTED FAILURE: Legitimate border styling should be preserved"
        );
    }
}
