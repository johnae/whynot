//! A Rust library for interacting with the notmuch email indexer.
//!
//! This crate provides:
//! - Comprehensive types for deserializing all notmuch JSON formats
//! - A unified client interface for executing notmuch commands locally or remotely
//! - Support for search results, email threads, messages, and attachments
//! - Test utilities for integration testing with temporary notmuch databases
//!
//! # Version Compatibility
//!
//! This library is compatible with notmuch 0.14 and later:
//! - **0.14+**: Core functionality (search, show, tag)
//! - **0.18+**: Insert command support
//! - **0.32+**: Configuration API
//!
//! # Quick Start
//!
//! ```no_run
//! use whynot::client::{create_client, ClientConfig, NotmuchClient};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Create a local client
//!     let client = create_client(ClientConfig::local())?;
//!     
//!     // Search for messages
//!     let results = client.search("tag:unread").await?;
//!     for item in results {
//!         println!("Thread: {} - {}", item.thread_id(), item.subject);
//!     }
//!     
//!     Ok(())
//! }
//! ```

pub mod body;
pub mod client;
pub mod common;
pub mod error;
pub mod search;
pub mod thread;
pub mod web;

#[cfg(any(test, feature = "test-utils"))]
pub mod test_utils;
