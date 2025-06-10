use std::sync::Arc;
use whynot::client::{NotmuchClient, TagOperation};
use whynot::error::NotmuchError;
use whynot::web::{AppState, WebConfig, create_app};

// Mock client for testing that doesn't require actual notmuch
struct MockNotmuchClient;

#[async_trait::async_trait]
impl NotmuchClient for MockNotmuchClient {
    async fn search(&self, _query: &str) -> Result<Vec<whynot::search::SearchItem>, NotmuchError> {
        Ok(vec![])
    }

    async fn search_paginated(&self, _query: &str, _offset: usize, _limit: usize) -> Result<(Vec<whynot::search::SearchItem>, Option<usize>), NotmuchError> {
        Ok((vec![], Some(0)))
    }

    async fn show(&self, _query: &str) -> Result<whynot::thread::Thread, NotmuchError> {
        Err(NotmuchError::CommandFailed(
            "Mock client - not implemented".to_string(),
        ))
    }

    async fn tag(&self, _query: &str, _tags: &[TagOperation]) -> Result<(), NotmuchError> {
        Err(NotmuchError::CommandFailed(
            "Mock client - not implemented".to_string(),
        ))
    }

    async fn refresh(&self) -> Result<(), NotmuchError> {
        Err(NotmuchError::CommandFailed(
            "Mock client - not implemented".to_string(),
        ))
    }

    async fn insert(
        &self,
        _message: &[u8],
        _folder: Option<&str>,
        _tags: &[&str],
    ) -> Result<String, NotmuchError> {
        Err(NotmuchError::CommandFailed(
            "Mock client - not implemented".to_string(),
        ))
    }

    async fn config_get(&self, _key: &str) -> Result<String, NotmuchError> {
        Err(NotmuchError::CommandFailed(
            "Mock client - not implemented".to_string(),
        ))
    }

    async fn config_set(&self, _key: &str, _value: &str) -> Result<(), NotmuchError> {
        Err(NotmuchError::CommandFailed(
            "Mock client - not implemented".to_string(),
        ))
    }

    async fn list_tags(&self) -> Result<Vec<String>, NotmuchError> {
        Ok(vec!["inbox".to_string(), "unread".to_string()])
    }

    async fn part(&self, _message_id: &str, _part_id: u32) -> Result<Vec<u8>, NotmuchError> {
        Err(NotmuchError::CommandFailed(
            "Mock client - not implemented".to_string(),
        ))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "whynot=debug,tower_http=info".into()),
        )
        .init();

    println!("Test Email Gallery Server");
    println!("========================");
    println!("This server only serves the test email gallery endpoints:");
    println!("- http://127.0.0.1:8080/test/email-gallery");
    println!("- http://127.0.0.1:8080/test/email-gallery/<email-name>");
    println!();

    // Create mock client for testing
    let client = Arc::new(MockNotmuchClient) as Arc<dyn NotmuchClient>;

    // Create web configuration
    let config = WebConfig {
        bind_address: "127.0.0.1:8080".parse().unwrap(),
        base_url: "http://localhost:8080".to_string(),
        items_per_page: 50,
        auto_refresh_interval: 30,
        initial_page_size: 20,
        pagination_size: 10,
        infinite_scroll_enabled: true,
    };

    let state = AppState {
        client,
        mail_sender: None,
        config: config.clone(),
        user_config: whynot::config::UserConfig::default(),
    };

    // Create the application
    let app = create_app(state);

    // Create the TCP listener
    let listener = tokio::net::TcpListener::bind(config.bind_address).await?;
    let local_addr = listener.local_addr()?;

    println!("Listening on: http://{}", local_addr);
    println!("Press Ctrl+C to stop");
    println!();

    // Serve the application
    axum::serve(listener, app).await?;

    Ok(())
}
