//! Mail sender implementations for sending email via msmtp.
//!
//! This module provides a unified interface for sending email messages
//! using msmtp, supporting both local command execution and remote execution
//! via SSH.
//!
//! # Overview
//!
//! The mail sender module consists of:
//! - `MailSender` trait - The main interface for all mail sender implementations
//! - `LocalMsmtpClient` - Executes msmtp commands locally
//! - `RemoteMsmtpClient` - Executes msmtp commands on a remote host via SSH
//! - `MailSenderConfig` - Configuration for creating mail senders
//! - `ComposableMessage` - Structure for composing email messages
//!
//! # Examples
//!
//! ```no_run
//! # use whynot::mail_sender::{create_mail_sender, MailSenderConfig, MailSender, ComposableMessage};
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! // Create a local mail sender
//! let config = MailSenderConfig::Local {
//!     msmtp_path: None,
//!     config_path: None,
//! };
//! let sender = create_mail_sender(config)?;
//!
//! // Compose and send a message
//! let message = ComposableMessage::builder()
//!     .to("recipient@example.com".to_string())
//!     .subject("Hello".to_string())
//!     .body("This is a test message.".to_string())
//!     .build()?;
//!
//! sender.send(message).await?;
//! # Ok(())
//! # }
//! ```

use async_trait::async_trait;
use crate::error::Result;
use crate::thread::Message;

pub mod config;
pub mod message;
pub mod local;
pub mod remote;

pub use config::MailSenderConfig;
pub use message::{ComposableMessage, MessageBuilder};
pub use local::LocalMsmtpClient;
pub use remote::RemoteMsmtpClient;

/// A client for sending email messages.
///
/// This trait provides a unified interface for sending email
/// either locally or remotely via SSH. All operations are async.
///
/// # Examples
///
/// ```no_run
/// # use whynot::mail_sender::{MailSender, ComposableMessage};
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// # let sender: &dyn MailSender = todo!();
/// // Send a new message
/// let message = ComposableMessage::builder()
///     .to("alice@example.com".to_string())
///     .subject("Meeting Tomorrow".to_string())
///     .body("Let's meet at 2pm.".to_string())
///     .build()?;
/// sender.send(message).await?;
/// # Ok(())
/// # }
/// ```
#[async_trait]
pub trait MailSender: Send + Sync {
    /// Send a new email message.
    ///
    /// This sends the provided message using msmtp. The message should
    /// have all required headers (To, From, Subject) properly set.
    ///
    /// # Arguments
    ///
    /// * `message` - The message to send
    ///
    /// # Returns
    ///
    /// The Message-ID of the sent message on success.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use whynot::mail_sender::{MailSender, ComposableMessage};
    /// # async fn example(sender: &dyn MailSender) -> Result<(), Box<dyn std::error::Error>> {
    /// let message = ComposableMessage::builder()
    ///     .to("recipient@example.com".to_string())
    ///     .from("sender@example.com".to_string())
    ///     .subject("Test Email".to_string())
    ///     .body("This is a test.".to_string())
    ///     .build()?;
    ///
    /// let message_id = sender.send(message).await?;
    /// println!("Sent message with ID: {}", message_id);
    /// # Ok(())
    /// # }
    /// ```
    async fn send(&self, message: ComposableMessage) -> Result<String>;

    /// Reply to an existing email message.
    ///
    /// This creates a reply to the given message with proper headers
    /// (In-Reply-To, References) and sends it. The original message
    /// body is quoted in the reply.
    ///
    /// # Arguments
    ///
    /// * `original` - The message being replied to
    /// * `reply` - The reply message content (without headers)
    /// * `reply_all` - Whether to reply to all recipients
    ///
    /// # Returns
    ///
    /// The Message-ID of the sent reply on success.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use whynot::mail_sender::{MailSender, ComposableMessage};
    /// # use whynot::thread::Message;
    /// # async fn example(sender: &dyn MailSender, original: &Message) -> Result<(), Box<dyn std::error::Error>> {
    /// // Reply to a message
    /// let reply = ComposableMessage::builder()
    ///     .to("dummy@example.com".to_string())  // Will be replaced by reply builder
    ///     .body("Thanks for your message!\n\nI agree with your proposal.".to_string())
    ///     .build()?;
    ///
    /// let message_id = sender.reply(original, reply, false).await?;
    /// # Ok(())
    /// # }
    /// ```
    async fn reply(&self, original: &Message, reply: ComposableMessage, reply_all: bool) -> Result<String>;

    /// Forward an existing email message.
    ///
    /// This forwards the given message to new recipients, preserving
    /// the original message content and attachments.
    ///
    /// # Arguments
    ///
    /// * `original` - The message to forward
    /// * `forward` - The forward message with new recipients and optional comment
    ///
    /// # Returns
    ///
    /// The Message-ID of the sent forward on success.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use whynot::mail_sender::{MailSender, ComposableMessage};
    /// # use whynot::thread::Message;
    /// # async fn example(sender: &dyn MailSender, original: &Message) -> Result<(), Box<dyn std::error::Error>> {
    /// // Forward a message
    /// let forward = ComposableMessage::builder()
    ///     .to("colleague@example.com".to_string())
    ///     .body("FYI - see below".to_string())
    ///     .build()?;
    ///
    /// let message_id = sender.forward(original, forward).await?;
    /// # Ok(())
    /// # }
    /// ```
    async fn forward(&self, original: &Message, forward: ComposableMessage) -> Result<String>;

    /// Test the mail sender configuration.
    ///
    /// This verifies that msmtp is properly configured and can connect
    /// to the SMTP server without actually sending a message.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use whynot::mail_sender::MailSender;
    /// # async fn example(sender: &dyn MailSender) -> Result<(), Box<dyn std::error::Error>> {
    /// // Test configuration before sending
    /// sender.test_connection().await?;
    /// println!("Mail sender is properly configured!");
    /// # Ok(())
    /// # }
    /// ```
    async fn test_connection(&self) -> Result<()>;

    /// Get the configured sender email address.
    ///
    /// Returns the email address that will be used as the From address
    /// when sending messages, if not explicitly overridden.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use whynot::mail_sender::MailSender;
    /// # async fn example(sender: &dyn MailSender) -> Result<(), Box<dyn std::error::Error>> {
    /// let from_address = sender.get_from_address().await?;
    /// println!("Sending from: {}", from_address);
    /// # Ok(())
    /// # }
    /// ```
    async fn get_from_address(&self) -> Result<String>;
}

/// Create a new mail sender based on the provided configuration.
///
/// This factory function creates either a `LocalMsmtpClient` or `RemoteMsmtpClient`
/// based on the configuration variant, returning it as a trait object.
///
/// # Arguments
///
/// * `config` - The mail sender configuration specifying local or remote execution
///
/// # Returns
///
/// A boxed `MailSender` trait object that can be used to send messages.
///
/// # Examples
///
/// ```no_run
/// # use whynot::mail_sender::{MailSenderConfig, create_mail_sender};
/// # use std::path::PathBuf;
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// // Create a local mail sender with default settings
/// let local_config = MailSenderConfig::Local {
///     msmtp_path: None,
///     config_path: None,
/// };
/// let local_sender = create_mail_sender(local_config)?;
///
/// // Create a remote mail sender via SSH
/// let remote_config = MailSenderConfig::Remote {
///     host: "mail.example.com".to_string(),
///     user: Some("alice".to_string()),
///     port: None,
///     identity_file: Some(PathBuf::from("/home/alice/.ssh/id_rsa")),
///     msmtp_path: None,
///     config_path: None,
/// };
/// let remote_sender = create_mail_sender(remote_config)?;
/// # Ok(())
/// # }
/// ```
pub fn create_mail_sender(config: MailSenderConfig) -> Result<Box<dyn MailSender>> {
    match config {
        MailSenderConfig::Local { .. } => {
            let client = LocalMsmtpClient::new(config)?;
            Ok(Box::new(client))
        }
        MailSenderConfig::Remote { .. } => {
            let client = RemoteMsmtpClient::new(config)?;
            Ok(Box::new(client))
        }
    }
}