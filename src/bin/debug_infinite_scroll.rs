use whynot::client::NotmuchClient;
use whynot::web::{AppState, WebConfig, create_app};

#[tokio::main]
async fn main() {
    println!("Creating test server to debug infinite scroll...");

    #[cfg(feature = "test-utils")]
    {
        use whynot::test_utils::mbox::{EmailMessage, MboxBuilder};
        use whynot::test_utils::notmuch::TestNotmuch;

        let test_notmuch = TestNotmuch::new().await.unwrap();

        // Create 15 test messages
        let mut mbox_builder = MboxBuilder::new();
        for i in 1..=15 {
            mbox_builder = mbox_builder.add_message(
                EmailMessage::new(format!("Test message {}", i))
                    .with_from(format!("sender{}@example.com", i))
                    .with_body(format!("This is test message number {}", i)),
            );
        }
        let mbox = mbox_builder.build();
        test_notmuch.add_mbox(&mbox).await.unwrap();

        let config = WebConfig {
            bind_address: ([127, 0, 0, 1], 8082).into(),
            base_url: "http://localhost:8082".to_string(),
            items_per_page: 20,
            auto_refresh_interval: 300,
            initial_page_size: 5, // Small for testing
            pagination_size: 3,   // Small for testing
            infinite_scroll_enabled: true,
        };

        let state = AppState {
            mail_sender: None,
            user_config: whynot::config::UserConfig::default(),
            client: std::sync::Arc::from(test_notmuch.client())
                as std::sync::Arc<dyn NotmuchClient>,
            config,
        };

        let app = create_app(state);

        println!("Starting server on http://127.0.0.1:8082");
        println!("Visit http://127.0.0.1:8082/inbox to test infinite scroll");
        println!("Check browser console for JavaScript debugging output");

        let listener = tokio::net::TcpListener::bind("127.0.0.1:8082")
            .await
            .unwrap();
        axum::serve(listener, app).await.unwrap();
    }

    #[cfg(not(feature = "test-utils"))]
    {
        println!(
            "test-utils feature not enabled. Run with: cargo run --features test-utils --bin debug_infinite_scroll"
        );
    }
}
