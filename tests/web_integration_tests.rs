use std::net::SocketAddr;
use whynot::client::{ClientConfig, NotmuchClient, create_client};
use whynot::web::{AppState, WebConfig, create_app};

async fn spawn_test_server() -> (SocketAddr, AppState) {
    let client = create_client(ClientConfig::local()).unwrap();
    let config = WebConfig {
        bind_address: ([127, 0, 0, 1], 0).into(), // Use port 0 for random port
        base_url: "http://localhost".to_string(),
        items_per_page: 10,
    };

    let state = AppState {
        mail_sender: None,
        user_config: whynot::config::UserConfig::default(),
        client: std::sync::Arc::from(client),
        config,
    };

    let app = create_app(state.clone());

    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();

    tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    });

    // Give the server a moment to start
    tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;

    (addr, state)
}

#[tokio::test]
async fn test_root_redirects_to_inbox() {
    let (addr, _state) = spawn_test_server().await;

    let client = reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .unwrap();

    let response = client
        .get(format!("http://{}/", addr))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), 303); // SEE_OTHER
    assert_eq!(response.headers().get("location").unwrap(), "/inbox");
}

#[tokio::test]
async fn test_inbox_returns_ok() {
    let (addr, _state) = spawn_test_server().await;

    let response = reqwest::get(format!("http://{}/inbox", addr))
        .await
        .unwrap();

    assert_eq!(response.status(), 200); // OK
    let body = response.text().await.unwrap();
    assert!(body.contains("Inbox"));
}

#[cfg(feature = "test-utils")]
#[tokio::test]
async fn test_inbox_displays_messages() {
    use whynot::test_utils::mbox::{EmailMessage, MboxBuilder};
    use whynot::test_utils::notmuch::TestNotmuch;

    let test_notmuch = TestNotmuch::new().await.unwrap();

    // Add test messages
    let mbox = MboxBuilder::new()
        .add_message(
            EmailMessage::new("Test message 1")
                .with_from("test@example.com")
                .with_body("This is the first test message"),
        )
        .add_message(
            EmailMessage::new("Test message 2")
                .with_from("sender@example.com")
                .with_body("This is the second test message"),
        )
        .add_message(
            EmailMessage::new("Test message 3")
                .with_from("another@example.com")
                .with_body("This is the third test message"),
        )
        .build();

    test_notmuch.add_mbox(&mbox).await.unwrap();

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

    let response = reqwest::get(format!("http://{}/inbox", addr))
        .await
        .unwrap();

    assert_eq!(response.status(), 200);
    let body = response.text().await.unwrap();

    // Should contain message subjects
    assert!(body.contains("Test message 1"));
    assert!(body.contains("Test message 2"));
    assert!(body.contains("Test message 3"));

    // Should display sender information
    assert!(body.contains("@example.com"));

    // Should be proper HTML
    assert!(body.contains("<!DOCTYPE html>"));
    assert!(body.contains("<html"));
}

#[tokio::test]
async fn test_theme_switching() {
    let (addr, _state) = spawn_test_server().await;

    // Use client that doesn't follow redirects to test the actual response
    let client = reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .unwrap();

    // Check default theme is light
    let response = client
        .get(format!("http://{}/inbox", addr))
        .send()
        .await
        .unwrap();
    let body = response.text().await.unwrap();
    assert!(body.contains(r#"data-theme="light""#));

    // Toggle theme to dark
    let response = client
        .post(format!("http://{}/settings/theme", addr))
        .send()
        .await
        .unwrap();
    assert_eq!(response.status(), 303);
    let cookie_header = response
        .headers()
        .get("set-cookie")
        .unwrap()
        .to_str()
        .unwrap();
    assert!(cookie_header.contains("theme=dark"));

    // Check theme is now dark when we send the cookie
    let response = client
        .get(format!("http://{}/inbox", addr))
        .header("Cookie", "theme=dark")
        .send()
        .await
        .unwrap();
    let body = response.text().await.unwrap();
    assert!(body.contains(r#"data-theme="dark""#));

    // Toggle back to light
    let response = client
        .post(format!("http://{}/settings/theme", addr))
        .header("Cookie", "theme=dark")
        .send()
        .await
        .unwrap();
    assert_eq!(response.status(), 303);
    let cookie_header = response
        .headers()
        .get("set-cookie")
        .unwrap()
        .to_str()
        .unwrap();
    assert!(cookie_header.contains("theme=light"));
}

#[tokio::test]
async fn test_tags_endpoint() {
    let (addr, _state) = spawn_test_server().await;

    let client = reqwest::Client::new();

    let response = client
        .get(format!("http://{}/tags", addr))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), 200);
    assert_eq!(
        response.headers().get("content-type").unwrap(),
        "application/json"
    );

    let body: serde_json::Value = response.json().await.unwrap();
    assert!(body.get("tags").is_some());

    let tags = body["tags"].as_array().unwrap();
    // Empty database should return empty tag list
    assert_eq!(tags.len(), 0);
}

#[tokio::test]
async fn test_search_with_tag_filter() {
    let (addr, _state) = spawn_test_server().await;

    let client = reqwest::Client::new();

    // Test search with single tag filter
    let response = client
        .get(format!("http://{}/search?tag=inbox", addr))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), 200);
    let body = response.text().await.unwrap();
    assert!(body.contains("class=\"inbox\""));
}

#[tokio::test]
async fn test_search_with_multiple_filters() {
    let (addr, _state) = spawn_test_server().await;

    let client = reqwest::Client::new();

    // Test search with text query and tag filter combined
    let response = client
        .get(format!("http://{}/search?q=test%20AND%20tag:unread", addr))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), 200);
    let body = response.text().await.unwrap();
    assert!(body.contains("class=\"inbox\""));
}

#[tokio::test]
async fn test_active_filters_display() {
    let (addr, _state) = spawn_test_server().await;

    let client = reqwest::Client::new();

    // Test search with tag filter shows active filter
    let response = client
        .get(format!("http://{}/search?q=tag:important", addr))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), 200);
    let body = response.text().await.unwrap();
    assert!(body.contains("class=\"active-filters\""));
    assert!(body.contains("important"));

    // Test search with text query shows active filter
    let response = client
        .get(format!("http://{}/search?q=hello%20world", addr))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), 200);
    let body = response.text().await.unwrap();
    assert!(body.contains("class=\"active-filters\""));
    assert!(body.contains("hello world"));
}

#[tokio::test]
async fn test_tag_filter_ui_elements() {
    let (addr, _state) = spawn_test_server().await;

    let client = reqwest::Client::new();

    let response = client
        .get(format!("http://{}/inbox", addr))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), 200);
    let body = response.text().await.unwrap();

    // Check for tag filtering UI elements
    assert!(body.contains("filter-toggle"));
    assert!(body.contains("Filter by Tags"));
    assert!(body.contains("tag-filter-dropdown"));
    assert!(body.contains("tag-list"));
    assert!(body.contains("loadTags()"));
    assert!(body.contains("toggleTagFilter()"));
}

#[tokio::test]
async fn test_search_form_functionality() {
    let (addr, _state) = spawn_test_server().await;

    let client = reqwest::Client::new();

    let response = client
        .get(format!("http://{}/inbox", addr))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), 200);
    let body = response.text().await.unwrap();

    // Check for search form elements
    assert!(body.contains(r#"action="/search""#));
    assert!(body.contains(r#"name="q""#));
    assert!(body.contains("search-input"));
    assert!(body.contains("search-button"));
    assert!(body.contains("Search"));
}

#[cfg(feature = "test-utils")]
#[tokio::test]
async fn test_thread_view_displays_message_content() {
    use whynot::test_utils::mbox::{EmailMessage, MboxBuilder};
    use whynot::test_utils::notmuch::TestNotmuch;

    let test_notmuch = TestNotmuch::new().await.unwrap();

    // Add a test message with specific content
    let message_body = "This is the actual content of the email.\nIt has multiple lines.\nAnd should be displayed in the thread view.";
    let mbox = MboxBuilder::new()
        .add_message(
            EmailMessage::new("Test Email Content Display")
                .with_from("sender@example.com")
                .with_to(vec!["recipient@example.com".to_string()])
                .with_body(message_body),
        )
        .build();

    test_notmuch.add_mbox(&mbox).await.unwrap();

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

    // First, get the thread ID
    let client = test_notmuch.client();
    let search_results = client
        .search("subject:\"Test Email Content Display\"")
        .await
        .unwrap();
    assert_eq!(search_results.len(), 1);
    let thread_id = search_results[0].thread_id();

    // Now fetch the thread view
    let response = reqwest::get(format!("http://{}/thread/{}", addr, thread_id))
        .await
        .unwrap();

    assert_eq!(response.status(), 200);
    let body = response.text().await.unwrap();

    // Verify the page structure
    assert!(body.contains("Test Email Content Display")); // Subject
    assert!(body.contains("sender@example.com")); // From
    assert!(body.contains("recipient@example.com")); // To

    // Check that the placeholder is NOT present
    assert!(
        !body.contains("[Message content display coming soon]"),
        "Thread view should display actual message content, not placeholder"
    );

    // Verify the actual message content is displayed
    assert!(
        body.contains("This is the actual content of the email")
            || body.contains("message-content"),
        "Thread view should display the message body content"
    );
}
