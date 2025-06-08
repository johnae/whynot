//! Local msmtp client implementation for sending email via msmtp locally.

use async_trait::async_trait;
use std::path::PathBuf;
use std::process::Stdio;
use tokio::process::Command;

use crate::error::{NotmuchError, Result};
use crate::mail_sender::{MailSender, MailSenderConfig, ComposableMessage};
use crate::thread::Message;

/// A mail sender that executes msmtp commands locally.
///
/// `LocalMsmtpClient` runs msmtp commands directly on the local system using
/// `tokio::process::Command`. It supports custom msmtp paths and configuration
/// file locations.
///
/// # Examples
///
/// ```no_run
/// # use whynot::mail_sender::{LocalMsmtpClient, MailSenderConfig};
/// # use std::path::PathBuf;
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// // Create a client with default msmtp location
/// let config = MailSenderConfig::Local {
///     msmtp_path: None,
///     config_path: None,
/// };
/// let client = LocalMsmtpClient::new(config)?;
///
/// // Create a client with custom paths
/// let config = MailSenderConfig::Local {
///     msmtp_path: Some(PathBuf::from("/usr/local/bin/msmtp")),
///     config_path: Some(PathBuf::from("/home/user/.msmtprc")),
/// };
/// let client = LocalMsmtpClient::new(config)?;
/// # Ok(())
/// # }
/// ```
pub struct LocalMsmtpClient {
    msmtp_path: PathBuf,
    config_path: Option<PathBuf>,
}

impl LocalMsmtpClient {
    pub fn new(config: MailSenderConfig) -> Result<Self> {
        match config {
            MailSenderConfig::Local {
                msmtp_path,
                config_path,
            } => Ok(LocalMsmtpClient {
                msmtp_path: msmtp_path.unwrap_or_else(|| PathBuf::from("msmtp")),
                config_path,
            }),
            _ => Err(NotmuchError::ConfigError(
                "Invalid config type for LocalMsmtpClient".to_string(),
            )),
        }
    }

    async fn execute_msmtp_command(&self, args: &[&str], message_data: &[u8]) -> Result<String> {
        let mut cmd = Command::new(&self.msmtp_path);

        // Add config file if specified
        if let Some(config_path) = &self.config_path {
            cmd.arg("--file").arg(config_path);
        }

        let mut child = cmd
            .args(args)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;

        // Send message data to msmtp via stdin
        if let Some(mut stdin) = child.stdin.take() {
            use tokio::io::AsyncWriteExt;
            stdin.write_all(message_data).await?;
        }

        let output = child.wait_with_output().await?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(NotmuchError::CommandFailed(format!(
                "msmtp {} failed: {}",
                args.join(" "),
                stderr
            )));
        }

        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }
}

#[async_trait]
impl MailSender for LocalMsmtpClient {
    async fn send(&self, message: ComposableMessage) -> Result<String> {
        let rfc822_data = message.to_rfc822()?;
        
        // Build recipient list for msmtp
        let mut recipients = message.to.clone();
        recipients.extend(message.cc.clone());
        recipients.extend(message.bcc.clone());
        
        let mut args = vec![];
        for recipient in &recipients {
            args.push(recipient.as_str());
        }
        
        self.execute_msmtp_command(&args, &rfc822_data).await?;
        Ok(message.message_id)
    }

    async fn reply(&self, original: &Message, reply: ComposableMessage, reply_all: bool) -> Result<String> {
        let mut full_reply = ComposableMessage::reply_builder(original, reply_all)
            .body(reply.body.clone())
            .build()?;
        
        // Merge in any additional fields from the reply
        if let Some(from) = reply.from {
            full_reply.from = Some(from);
        }
        
        self.send(full_reply).await
    }

    async fn forward(&self, original: &Message, forward: ComposableMessage) -> Result<String> {
        let mut full_forward = ComposableMessage::forward_builder(original)
            .body(forward.body.clone())
            .build()?;
        
        // Merge in recipients from forward
        full_forward.to = forward.to;
        full_forward.cc = forward.cc;
        full_forward.bcc = forward.bcc;
        
        if let Some(from) = forward.from {
            full_forward.from = Some(from);
        }
        
        self.send(full_forward).await
    }

    async fn test_connection(&self) -> Result<()> {
        // Test msmtp configuration with --serverinfo flag
        let mut cmd = Command::new(&self.msmtp_path);
        
        if let Some(config_path) = &self.config_path {
            cmd.arg("--file").arg(config_path);
        }
        
        let output = cmd
            .arg("--serverinfo")
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .await?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(NotmuchError::CommandFailed(format!(
                "msmtp test connection failed: {}",
                stderr
            )));
        }

        Ok(())
    }

    async fn get_from_address(&self) -> Result<String> {
        // Get the default from address from msmtp config
        let mut cmd = Command::new(&self.msmtp_path);
        
        if let Some(config_path) = &self.config_path {
            cmd.arg("--file").arg(config_path);
        }
        
        let output = cmd
            .arg("--print-config")
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .await?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(NotmuchError::CommandFailed(format!(
                "msmtp get config failed: {}",
                stderr
            )));
        }

        let config_output = String::from_utf8_lossy(&output.stdout);
        
        // Parse the config output to find the from address
        for line in config_output.lines() {
            if line.starts_with("from") {
                if let Some(from_addr) = line.split_whitespace().nth(1) {
                    return Ok(from_addr.to_string());
                }
            }
        }

        Err(NotmuchError::ConfigError(
            "Could not determine from address from msmtp config".to_string(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_local_msmtp_client_config() {
        let config = MailSenderConfig::Local {
            msmtp_path: Some(PathBuf::from("/usr/bin/msmtp")),
            config_path: Some(PathBuf::from("/home/user/.msmtprc")),
        };

        let client = LocalMsmtpClient::new(config);
        assert!(client.is_ok());

        let client = client.unwrap();
        assert_eq!(client.msmtp_path, PathBuf::from("/usr/bin/msmtp"));
        assert_eq!(client.config_path, Some(PathBuf::from("/home/user/.msmtprc")));
    }

    #[test]
    fn test_local_msmtp_client_default_config() {
        let config = MailSenderConfig::Local {
            msmtp_path: None,
            config_path: None,
        };

        let client = LocalMsmtpClient::new(config);
        assert!(client.is_ok());

        let client = client.unwrap();
        assert_eq!(client.msmtp_path, PathBuf::from("msmtp"));
        assert_eq!(client.config_path, None);
    }

    #[test]
    fn test_local_msmtp_client_invalid_config() {
        let config = MailSenderConfig::Remote {
            host: "example.com".to_string(),
            user: None,
            port: None,
            identity_file: None,
            msmtp_path: None,
            config_path: None,
        };

        let result = LocalMsmtpClient::new(config);
        assert!(result.is_err());
    }
}