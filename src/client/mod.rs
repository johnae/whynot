//! Notmuch client implementations for local and remote command execution.
//!
//! This module provides a unified interface for interacting with the notmuch
//! email indexer, supporting both local command execution and remote execution
//! via SSH.
//!
//! # Overview
//!
//! The client module consists of:
//! - `NotmuchClient` trait - The main interface for all client implementations
//! - `LocalClient` - Executes notmuch commands locally
//! - `RemoteClient` - Executes notmuch commands on a remote host via SSH
//! - `ClientConfig` - Configuration for creating clients
//! - `TagOperation` - Represents tag add/remove operations
//!
//! # Version Compatibility
//!
//! This client library is designed to work with notmuch 0.14 and later, which
//! introduced stable JSON output formats. Specific version requirements:
//!
//! - **0.14+**: Basic functionality (search, show, tag)
//!   - JSON output format for search and show commands
//!   - Core tagging operations
//!
//! - **0.18+**: Insert command support
//!   - `notmuch insert` for adding messages directly
//!   - Folder specification support
//!
//! - **0.32+**: Configuration API
//!   - `notmuch config get/set` commands
//!   - Programmatic configuration access
//!
//! The client will work with newer versions of notmuch as they maintain
//! backward compatibility with the JSON output format.
//!
//! # Examples
//!
//! ```no_run
//! # use whynot::client::{create_client, ClientConfig, NotmuchClient, TagOperation};
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! // Create a local client
//! let config = ClientConfig::Local {
//!     notmuch_path: None,
//!     database_path: None,
//!     mail_root: None,
//! };
//! let client = create_client(config)?;
//!
//! // Search for unread messages
//! let unread = client.search("tag:unread").await?;
//! println!("Found {} unread threads", unread.len());
//!
//! // Mark first thread as read
//! if let Some(first) = unread.first() {
//!     client.tag(
//!         &format!("thread:{}", first.thread_id()),
//!         &[TagOperation::Remove("unread".to_string())]
//!     ).await?;
//! }
//! # Ok(())
//! # }
//! ```

use async_trait::async_trait;
use std::fmt;

use crate::error::Result;
use crate::search::SearchItem;
use crate::thread::Thread;

pub mod config;
pub mod local;
pub mod remote;

pub use config::ClientConfig;
pub use local::LocalClient;
pub use remote::RemoteClient;

/// Represents a tag operation to be performed on messages.
///
/// Tag operations are used with the `tag()` method to add or remove
/// tags from messages matching a query. Multiple operations can be
/// performed in a single call.
///
/// # Examples
///
/// ```
/// # use whynot::client::TagOperation;
/// // Add a tag
/// let add_important = TagOperation::Add("important".to_string());
///
/// // Remove a tag
/// let remove_unread = TagOperation::Remove("unread".to_string());
///
/// // The Display implementation formats for notmuch command line
/// assert_eq!(add_important.to_string(), "+important");
/// assert_eq!(remove_unread.to_string(), "-unread");
/// ```
#[derive(Debug, Clone)]
pub enum TagOperation {
    /// Add a tag to matching messages
    Add(String),
    /// Remove a tag from matching messages
    Remove(String),
}

impl fmt::Display for TagOperation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TagOperation::Add(tag) => write!(f, "+{}", tag),
            TagOperation::Remove(tag) => write!(f, "-{}", tag),
        }
    }
}

/// A client for interacting with notmuch email indexer.
///
/// This trait provides a unified interface for executing notmuch commands
/// either locally or remotely via SSH. All operations are async and return
/// strongly-typed results using the types from iteration 1.
///
/// # Version Compatibility
///
/// This client is tested with notmuch 0.32 and later. It relies on the JSON
/// output format which has been stable since notmuch 0.14. The following
/// features require specific versions:
/// - Basic search/show: 0.14+
/// - Insert command: 0.18+
/// - Configuration API: 0.32+
///
/// # Examples
///
/// ```no_run
/// # use whynot::client::{NotmuchClient, ClientConfig, create_client};
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// // Create a local client
/// let config = ClientConfig::Local {
///     notmuch_path: None,
///     database_path: None,
///     mail_root: None,
/// };
/// let client = create_client(config)?;
///
/// // Search for messages
/// let results = client.search("from:alice@example.com").await?;
/// for item in results {
///     println!("Subject: {}", item.subject);
/// }
/// # Ok(())
/// # }
/// ```
#[async_trait]
pub trait NotmuchClient: Send + Sync {
    /// Search for messages matching a query.
    ///
    /// This executes `notmuch search --format=json` with the given query string.
    /// The query syntax follows the notmuch search term syntax.
    ///
    /// # Arguments
    ///
    /// * `query` - A notmuch query string (e.g., "tag:inbox", "from:alice", "subject:meeting")
    ///
    /// # Returns
    ///
    /// A vector of `SearchItem` results, one for each matching thread.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use whynot::client::NotmuchClient;
    /// # async fn example(client: &dyn NotmuchClient) -> Result<(), Box<dyn std::error::Error>> {
    /// // Search for all unread messages
    /// let unread = client.search("tag:unread").await?;
    ///
    /// // Search with multiple criteria
    /// let important = client.search("tag:important AND date:2024..").await?;
    ///
    /// // Search for messages from a specific sender
    /// let from_alice = client.search("from:alice@example.com").await?;
    /// # Ok(())
    /// # }
    /// ```
    async fn search(&self, query: &str) -> Result<Vec<SearchItem>>;

    /// Search for messages matching a query with pagination support.
    ///
    /// This executes `notmuch search --format=json` with the given query string
    /// and applies offset and limit for pagination.
    ///
    /// # Arguments
    ///
    /// * `query` - A notmuch query string (e.g., "tag:inbox", "from:alice")
    /// * `offset` - Number of results to skip (0-based)
    /// * `limit` - Maximum number of results to return
    ///
    /// # Returns
    ///
    /// A tuple containing:
    /// - Vector of `SearchItem` results for the requested page
    /// - Total count of matching messages (if available)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use whynot::client::NotmuchClient;
    /// # async fn example(client: &dyn NotmuchClient) -> Result<(), Box<dyn std::error::Error>> {
    /// // Get first 20 unread messages
    /// let (page, total) = client.search_paginated("tag:unread", 0, 20).await?;
    ///
    /// // Get next 20 unread messages
    /// let (next_page, _) = client.search_paginated("tag:unread", 20, 20).await?;
    /// # Ok(())
    /// # }
    /// ```
    async fn search_paginated(
        &self,
        query: &str,
        offset: usize,
        limit: usize,
    ) -> Result<(Vec<SearchItem>, Option<usize>)>;

    /// Show messages matching a query in thread format.
    ///
    /// This executes `notmuch show --format=json` with the given query string.
    /// Typically used with a thread ID to retrieve all messages in a thread.
    ///
    /// # Arguments
    ///
    /// * `query` - A notmuch query string, typically "thread:<thread-id>"
    ///
    /// # Returns
    ///
    /// A `Thread` containing all matching messages with their full content.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use whynot::client::NotmuchClient;
    /// # async fn example(client: &dyn NotmuchClient) -> Result<(), Box<dyn std::error::Error>> {
    /// // First search for a thread
    /// let results = client.search("subject:\"Project Update\"").await?;
    /// if let Some(item) = results.first() {
    ///     // Then show all messages in that thread
    ///     let thread_id = item.thread_id();
    ///     let thread = client.show(&format!("thread:{}", thread_id)).await?;
    ///     
    ///     for message in thread.get_messages() {
    ///         println!("From: {}", message.headers.from);
    ///         println!("Subject: {}", message.headers.subject.as_deref().unwrap_or("(no subject)"));
    ///     }
    /// }
    /// # Ok(())
    /// # }
    /// ```
    async fn show(&self, query: &str) -> Result<Thread>;

    /// Add or remove tags from messages matching a query.
    ///
    /// This executes `notmuch tag` with the specified tag operations.
    /// Multiple operations can be performed in a single call.
    ///
    /// # Arguments
    ///
    /// * `query` - A notmuch query string to select messages
    /// * `tags` - A slice of `TagOperation` values (Add or Remove)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use whynot::client::{NotmuchClient, TagOperation};
    /// # async fn example(client: &dyn NotmuchClient) -> Result<(), Box<dyn std::error::Error>> {
    /// // Mark messages as read
    /// client.tag(
    ///     "tag:unread AND from:newsletter@example.com",
    ///     &[TagOperation::Remove("unread".to_string())]
    /// ).await?;
    ///
    /// // Add multiple tags
    /// client.tag(
    ///     "subject:\"Important\" AND tag:inbox",
    ///     &[
    ///         TagOperation::Add("flagged".to_string()),
    ///         TagOperation::Add("todo".to_string()),
    ///     ]
    /// ).await?;
    /// # Ok(())
    /// # }
    /// ```
    async fn tag(&self, query: &str, tags: &[TagOperation]) -> Result<()>;

    /// Scan for new messages in the mail directory.
    ///
    /// This executes `notmuch new` to discover and index new mail files.
    /// Should be called after new mail has been delivered to the mail directory.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use whynot::client::NotmuchClient;
    /// # async fn example(client: &dyn NotmuchClient) -> Result<(), Box<dyn std::error::Error>> {
    /// // Refresh the database to find new messages
    /// client.refresh().await?;
    ///
    /// // Now search will include any newly discovered messages
    /// let new_messages = client.search("tag:new").await?;
    /// # Ok(())
    /// # }
    /// ```
    async fn refresh(&self) -> Result<()>;

    /// Insert a new message into the database.
    ///
    /// This executes `notmuch insert` to add a message and index it immediately.
    /// The message should be in RFC 822 format.
    ///
    /// # Arguments
    ///
    /// * `message` - The complete email message in RFC 822 format
    /// * `folder` - Optional folder name (relative to mail root)
    /// * `tags` - Initial tags to apply to the message
    ///
    /// # Returns
    ///
    /// The message ID of the inserted message.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use whynot::client::NotmuchClient;
    /// # async fn example(client: &dyn NotmuchClient) -> Result<(), Box<dyn std::error::Error>> {
    /// let message = b"From: sender@example.com\r\n\
    ///                 To: recipient@example.com\r\n\
    ///                 Subject: Test Message\r\n\
    ///                 Date: Mon, 01 Jan 2024 12:00:00 +0000\r\n\
    ///                 Message-ID: <unique-id@example.com>\r\n\
    ///                 \r\n\
    ///                 This is the message body.\r\n";
    ///
    /// // Insert into inbox with some tags
    /// let message_id = client.insert(
    ///     message,
    ///     Some("inbox"),
    ///     &["unread", "important"]
    /// ).await?;
    /// # Ok(())
    /// # }
    /// ```
    async fn insert(&self, message: &[u8], folder: Option<&str>, tags: &[&str]) -> Result<String>;

    /// Get a notmuch configuration value.
    ///
    /// This executes `notmuch config get` to retrieve configuration settings.
    ///
    /// # Arguments
    ///
    /// * `key` - The configuration key (e.g., "user.name", "user.primary_email")
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use whynot::client::NotmuchClient;
    /// # async fn example(client: &dyn NotmuchClient) -> Result<(), Box<dyn std::error::Error>> {
    /// // Get user configuration
    /// let user_name = client.config_get("user.name").await?;
    /// let email = client.config_get("user.primary_email").await?;
    ///
    /// println!("Notmuch user: {} <{}>", user_name, email);
    /// # Ok(())
    /// # }
    /// ```
    async fn config_get(&self, key: &str) -> Result<String>;

    /// Set a notmuch configuration value.
    ///
    /// This executes `notmuch config set` to update configuration settings.
    ///
    /// # Arguments
    ///
    /// * `key` - The configuration key
    /// * `value` - The new value
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use whynot::client::NotmuchClient;
    /// # async fn example(client: &dyn NotmuchClient) -> Result<(), Box<dyn std::error::Error>> {
    /// // Update user configuration
    /// client.config_set("user.name", "Alice Smith").await?;
    /// client.config_set("user.primary_email", "alice@example.com").await?;
    /// # Ok(())
    /// # }
    /// ```
    async fn config_set(&self, key: &str, value: &str) -> Result<()>;

    /// List all tags in the notmuch database.
    ///
    /// This executes `notmuch search --output=tags --format=json '*'` to retrieve
    /// all tags that exist in the database.
    ///
    /// # Returns
    ///
    /// A vector of tag names as strings.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use whynot::client::NotmuchClient;
    /// # async fn example(client: &dyn NotmuchClient) -> Result<(), Box<dyn std::error::Error>> {
    /// // Get all available tags
    /// let tags = client.list_tags().await?;
    ///
    /// for tag in tags {
    ///     println!("Tag: {}", tag);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    async fn list_tags(&self) -> Result<Vec<String>>;

    /// Extract the raw content of a specific part from a message.
    ///
    /// This executes `notmuch part --format=raw --part=<part_id>` to retrieve
    /// the raw content of an attachment or message part. This is necessary for
    /// binary attachments as the `show` command doesn't include their content.
    ///
    /// # Arguments
    ///
    /// * `message_id` - The message ID containing the part
    /// * `part_id` - The numeric part ID to extract
    ///
    /// # Returns
    ///
    /// Raw bytes of the part content, typically base64-decoded for attachments.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use whynot::client::NotmuchClient;
    /// # async fn example(client: &dyn NotmuchClient) -> Result<(), Box<dyn std::error::Error>> {
    /// // Extract attachment content from message
    /// let content = client.part("id:message@example.com", 3).await?;
    /// std::fs::write("attachment.pdf", content)?;
    /// # Ok(())
    /// # }
    /// ```
    async fn part(&self, message_id: &str, part_id: u32) -> Result<Vec<u8>>;
}

/// Create a new notmuch client based on the provided configuration.
///
/// This factory function creates either a `LocalClient` or `RemoteClient`
/// based on the configuration variant, returning it as a trait object.
///
/// # Arguments
///
/// * `config` - The client configuration specifying local or remote execution
///
/// # Returns
///
/// A boxed `NotmuchClient` trait object that can be used to execute commands.
///
/// # Examples
///
/// ```no_run
/// # use whynot::client::{ClientConfig, create_client};
/// # use std::path::PathBuf;
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// // Create a local client with default settings
/// let local_config = ClientConfig::Local {
///     notmuch_path: None,
///     database_path: None,
///     mail_root: None,
/// };
/// let local_client = create_client(local_config)?;
///
/// // Create a remote client via SSH
/// let remote_config = ClientConfig::Remote {
///     host: "mail.example.com".to_string(),
///     user: Some("alice".to_string()),
///     port: None,
///     identity_file: Some(PathBuf::from("/home/alice/.ssh/id_rsa")),
///     notmuch_path: None,
/// };
/// let remote_client = create_client(remote_config)?;
/// # Ok(())
/// # }
/// ```
pub fn create_client(config: ClientConfig) -> Result<Box<dyn NotmuchClient>> {
    match &config {
        ClientConfig::Local { .. } => Ok(Box::new(LocalClient::new(config)?)),
        ClientConfig::Remote { .. } => Ok(Box::new(RemoteClient::new(config)?)),
    }
}
