#![cfg(feature = "test-utils")]

use std::path::PathBuf;
use whynot::client::{ClientConfig, create_client};

#[test]
fn test_create_remote_client() {
    let config = ClientConfig::Remote {
        host: "example.com".to_string(),
        user: Some("testuser".to_string()),
        port: Some(2222),
        identity_file: Some(PathBuf::from("/home/user/.ssh/id_rsa")),
        notmuch_path: None,
    };

    let client = create_client(config);
    assert!(client.is_ok());
}

#[test]
fn test_create_remote_client_minimal() {
    let config = ClientConfig::Remote {
        host: "mail.example.com".to_string(),
        user: None,
        port: None,
        identity_file: None,
        notmuch_path: None,
    };

    let client = create_client(config);
    assert!(client.is_ok());
}

// Manual testing instructions for RemoteClient:
//
// To test the RemoteClient functionality, you need:
// 1. A remote server with notmuch installed
// 2. SSH access to that server
// 3. A notmuch database on the remote server
//
// Example test code:
// ```rust
// use whynot::client::{ClientConfig, NotmuchClient, create_client};
//
// #[tokio::main]
// async fn main() {
//     let config = ClientConfig::Remote {
//         host: "your-server.com".to_string(),
//         user: Some("your-username".to_string()),
//         port: None,
//         identity_file: Some("/home/you/.ssh/id_rsa".into()),
//         notmuch_path: None,
//     };
//
//     let client = create_client(config).unwrap();
//
//     // Test search
//     let results = client.search("*").await.unwrap();
//     println!("Found {} messages", results.len());
//
//     // Test show
//     if !results.is_empty() {
//         let thread_id = results[0].thread_id();
//         let thread = client.show(&format!("thread:{}", thread_id)).await.unwrap();
//         println!("Thread has {} messages", thread.get_messages().len());
//     }
// }
// ```
