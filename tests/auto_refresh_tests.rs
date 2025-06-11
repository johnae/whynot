use std::net::SocketAddr;
use whynot::client::{ClientConfig, create_client};
use whynot::web::{AppState, WebConfig, create_app};

async fn spawn_test_server() -> (SocketAddr, AppState) {
    let client = create_client(ClientConfig::local()).unwrap();
    let config = WebConfig {
        bind_address: ([127, 0, 0, 1], 0).into(), // Use port 0 for random port
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
async fn test_refresh_query_endpoint_exists() {
    let (addr, _state) = spawn_test_server().await;

    let client = reqwest::Client::new();

    let response = client
        .get(format!("http://{}/api/refresh-query", addr))
        .send()
        .await
        .unwrap();

    // Endpoint should exist and return a response (not 404)
    assert_ne!(response.status(), 404, "The /api/refresh-query endpoint should exist");
}

#[tokio::test]
async fn test_refresh_query_returns_json() {
    let (addr, _state) = spawn_test_server().await;

    let client = reqwest::Client::new();

    let response = client
        .get(format!("http://{}/api/refresh-query", addr))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), 200);
    assert_eq!(
        response.headers().get("content-type").unwrap(),
        "application/json"
    );
}

#[tokio::test]
async fn test_refresh_query_accepts_query_parameter() {
    let (addr, _state) = spawn_test_server().await;

    let client = reqwest::Client::new();

    let response = client
        .get(format!("http://{}/api/refresh-query?q=tag:inbox", addr))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), 200);
    
    let body: serde_json::Value = response.json().await.unwrap();
    assert!(body.get("messages").is_some());
}

#[tokio::test]
async fn test_refresh_query_response_structure() {
    let (addr, _state) = spawn_test_server().await;

    let client = reqwest::Client::new();

    let response = client
        .get(format!("http://{}/api/refresh-query?q=tag:inbox", addr))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), 200);
    
    let body: serde_json::Value = response.json().await.unwrap();
    
    // Should contain messages array
    assert!(body.get("messages").is_some());
    assert!(body["messages"].is_array());
    
    // Should contain timestamp for caching/comparison
    assert!(body.get("timestamp").is_some());
}

#[cfg(feature = "test-utils")]
#[tokio::test]
async fn test_refresh_query_returns_current_messages() {
    use whynot::test_utils::mbox::{EmailMessage, MboxBuilder};
    use whynot::test_utils::notmuch::TestNotmuch;

    let test_notmuch = TestNotmuch::new().await.unwrap();

    // Add test messages
    let mbox = MboxBuilder::new()
        .add_message(
            EmailMessage::new("First message")
                .with_from("sender1@example.com")
                .with_body("This is the first message"),
        )
        .add_message(
            EmailMessage::new("Second message")
                .with_from("sender2@example.com")
                .with_body("This is the second message"),
        )
        .build();

    test_notmuch.add_mbox(&mbox).await.unwrap();

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

    let client = reqwest::Client::new();

    let response = client
        .get(format!("http://{}/api/refresh-query?q=*", addr))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), 200);
    
    let body: serde_json::Value = response.json().await.unwrap();
    let messages = body["messages"].as_array().unwrap();
    
    // Should contain both messages
    assert_eq!(messages.len(), 2);
    
    // Should contain message subjects in the response
    let response_text = serde_json::to_string(&body).unwrap();
    assert!(response_text.contains("First message"));
    assert!(response_text.contains("Second message"));
}

#[tokio::test]
async fn test_inbox_contains_auto_refresh_javascript() {
    let (addr, _state) = spawn_test_server().await;

    let client = reqwest::Client::new();

    let response = client
        .get(format!("http://{}/inbox", addr))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), 200);
    let body = response.text().await.unwrap();

    // Should contain JavaScript for auto-refresh
    assert!(body.contains("auto-refresh") || body.contains("autoRefresh"));
    
    // Should contain polling interval configuration
    assert!(body.contains("30000") || body.contains("setInterval"));
    
    // Should contain refresh function
    assert!(body.contains("refreshQuery") || body.contains("refresh"));
}

#[tokio::test]
async fn test_search_page_contains_auto_refresh_javascript() {
    let (addr, _state) = spawn_test_server().await;

    let client = reqwest::Client::new();

    let response = client
        .get(format!("http://{}/search?q=tag:inbox", addr))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), 200);
    let body = response.text().await.unwrap();

    // Should contain JavaScript for auto-refresh
    assert!(body.contains("auto-refresh") || body.contains("autoRefresh"));
    
    // Should contain polling interval configuration
    assert!(body.contains("30000") || body.contains("setInterval"));
    
    // Should contain refresh function
    assert!(body.contains("refreshQuery") || body.contains("refresh"));
}

#[tokio::test]
async fn test_refresh_query_handles_invalid_query() {
    let (addr, _state) = spawn_test_server().await;

    let client = reqwest::Client::new();

    // Test with malformed query
    let response = client
        .get(format!("http://{}/api/refresh-query?q=invalid:query:syntax", addr))
        .send()
        .await
        .unwrap();

    // Should handle gracefully, not crash
    assert!(response.status().is_success() || response.status().is_client_error());
    
    if response.status().is_success() {
        let body: serde_json::Value = response.json().await.unwrap();
        assert!(body.get("messages").is_some());
        assert!(body["messages"].is_array());
    }
}

#[tokio::test]
async fn test_refresh_query_default_query() {
    let (addr, _state) = spawn_test_server().await;

    let client = reqwest::Client::new();

    // Test without query parameter - should default to inbox
    let response = client
        .get(format!("http://{}/api/refresh-query", addr))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), 200);
    
    let body: serde_json::Value = response.json().await.unwrap();
    assert!(body.get("messages").is_some());
    assert!(body["messages"].is_array());
}

#[cfg(feature = "test-utils")]
#[tokio::test]
async fn test_refresh_query_preserves_thread_ids() {
    use whynot::test_utils::mbox::{EmailMessage, MboxBuilder};
    use whynot::test_utils::notmuch::TestNotmuch;

    let test_notmuch = TestNotmuch::new().await.unwrap();

    // Add test messages with specific content
    let mbox = MboxBuilder::new()
        .add_message(
            EmailMessage::new("Test message for thread ID")
                .with_from("sender@example.com")
                .with_body("This message should have a valid thread ID"),
        )
        .build();

    test_notmuch.add_mbox(&mbox).await.unwrap();

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

    let client = reqwest::Client::new();

    let response = client
        .get(format!("http://{}/api/refresh-query?q=*", addr))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), 200);
    
    let body: serde_json::Value = response.json().await.unwrap();
    let messages = body["messages"].as_array().unwrap();
    
    // Should have our test message
    assert_eq!(messages.len(), 1);
    
    let message = &messages[0];
    
    // Critical test: thread field must be present and non-empty
    assert!(message.get("thread").is_some(), "Message should have 'thread' field");
    let thread_id = message["thread"].as_str().unwrap();
    assert!(!thread_id.is_empty(), "Thread ID should not be empty");
    assert!(thread_id.len() > 5, "Thread ID should be a valid notmuch thread ID");
    
    // Verify other essential fields for building links
    assert!(message.get("subject").is_some(), "Message should have 'subject' field");
    assert!(message.get("authors").is_some(), "Message should have 'authors' field");
}

#[cfg(feature = "test-utils")]
#[tokio::test]
async fn test_auto_refresh_javascript_generates_correct_thread_links() {
    use whynot::test_utils::mbox::{EmailMessage, MboxBuilder};
    use whynot::test_utils::notmuch::TestNotmuch;
    use scraper::{Html, Selector};

    let test_notmuch = TestNotmuch::new().await.unwrap();

    // Add test message
    let mbox = MboxBuilder::new()
        .add_message(
            EmailMessage::new("Test Thread Link Generation")
                .with_from("test@example.com")
                .with_body("Body content for link test"),
        )
        .build();

    test_notmuch.add_mbox(&mbox).await.unwrap();

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

    let client = reqwest::Client::new();

    // First, verify the initial page renders correctly
    let initial_response = client
        .get(format!("http://{}/inbox", addr))
        .send()
        .await
        .unwrap();

    assert_eq!(initial_response.status(), 200);
    let initial_html = initial_response.text().await.unwrap();
    
    // Parse the HTML and check that thread links are present
    let document = Html::parse_document(&initial_html);
    let link_selector = Selector::parse("a[href*='/thread/']").unwrap();
    let links: Vec<_> = document.select(&link_selector).collect();
    
    // We should have at least one thread link in the initial render
    assert!(!links.is_empty(), "Initial page should contain thread links");
    
    // Get the href of the first link to compare later
    let initial_href = links[0].value().attr("href").unwrap();
    assert!(initial_href.starts_with("/thread/"), "Link should start with /thread/");
    assert!(initial_href.len() > "/thread/".len(), "Thread ID should not be empty in link");
    
    // Check that the JavaScript contains the problematic code that needs fixing
    assert!(initial_html.contains("updateMessageList"), "Page should contain updateMessageList function");
    
    // This is the specific bug we're testing for:
    // The JavaScript likely uses message.thread_id instead of message.thread
    if initial_html.contains("message.thread_id") {
        // This would be the bug - JavaScript trying to access thread_id property
        panic!("JavaScript contains 'message.thread_id' which would cause broken links after refresh");
    }
    
    // Now test what happens after an auto-refresh
    let refresh_response = client
        .get(format!("http://{}/api/refresh-query?q=*", addr))
        .send()
        .await
        .unwrap();

    assert_eq!(refresh_response.status(), 200);
    let refresh_data: serde_json::Value = refresh_response.json().await.unwrap();
    let messages = refresh_data["messages"].as_array().unwrap();
    
    // Verify the API response has the correct structure for JavaScript to use
    assert_eq!(messages.len(), 1);
    let message = &messages[0];
    
    // The critical issue: JavaScript needs to access the 'thread' field, not 'thread_id'
    assert!(message.get("thread").is_some(), "API should return 'thread' field");
    let thread_id = message["thread"].as_str().unwrap();
    
    // Verify this is the same thread ID as in the initial page
    let expected_thread_path = format!("/thread/{}", thread_id);
    assert_eq!(initial_href, expected_thread_path, 
        "Thread ID from API should match the one in initial page links");
}