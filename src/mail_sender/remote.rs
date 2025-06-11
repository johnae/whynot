//! Remote msmtp client implementation for sending email via msmtp over SSH.

use async_trait::async_trait;
use std::path::PathBuf;
use std::process::Stdio;
use tokio::process::Command;

use crate::error::{NotmuchError, Result};
use crate::mail_sender::{ComposableMessage, MailSender, MailSenderConfig};
use crate::thread::Message;

/// A mail sender that executes msmtp commands on a remote host via SSH.
///
/// `RemoteMsmtpClient` runs msmtp commands on a remote system using SSH.
/// It supports key-based authentication and custom SSH options for
/// reliable connections, following the same pattern as RemoteClient.
///
/// # SSH Options
///
/// The client automatically configures the following SSH options:
/// - `BatchMode=yes` - Prevents interactive prompts
/// - `ConnectTimeout=30` - 30 second connection timeout
/// - `ServerAliveInterval=60` - Keepalive every 60 seconds
/// - `ServerAliveCountMax=3` - Disconnect after 3 missed keepalives
///
/// # Examples
///
/// ```no_run
/// # use whynot::mail_sender::{RemoteMsmtpClient, MailSenderConfig};
/// # use std::path::PathBuf;
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// // Connect with default SSH settings
/// let config = MailSenderConfig::Remote {
///     host: "mail.example.com".to_string(),
///     user: None,
///     port: None,
///     identity_file: None,
///     msmtp_path: None,
///     config_path: None,
/// };
/// let client = RemoteMsmtpClient::new(config)?;
///
/// // Connect with custom SSH options
/// let config = MailSenderConfig::Remote {
///     host: "mail.example.com".to_string(),
///     user: Some("alice".to_string()),
///     port: Some(2222),
///     identity_file: Some(PathBuf::from("/home/alice/.ssh/mail_key")),
///     msmtp_path: Some(PathBuf::from("/usr/local/bin/msmtp")),
///     config_path: Some(PathBuf::from("/home/alice/.msmtprc")),
/// };
/// let client = RemoteMsmtpClient::new(config)?;
/// # Ok(())
/// # }
/// ```
pub struct RemoteMsmtpClient {
    host: String,
    user: Option<String>,
    port: Option<u16>,
    identity_file: Option<PathBuf>,
    msmtp_path: PathBuf,
    config_path: Option<PathBuf>,
}

impl RemoteMsmtpClient {
    pub fn new(config: MailSenderConfig) -> Result<Self> {
        match config {
            MailSenderConfig::Remote {
                host,
                user,
                port,
                identity_file,
                msmtp_path,
                config_path,
            } => Ok(RemoteMsmtpClient {
                host,
                user,
                port,
                identity_file,
                msmtp_path: msmtp_path.unwrap_or_else(|| PathBuf::from("msmtp")),
                config_path,
            }),
            _ => Err(NotmuchError::ConfigError(
                "Invalid config type for RemoteMsmtpClient".to_string(),
            )),
        }
    }

    async fn execute_ssh_msmtp_command(
        &self,
        msmtp_args: &[&str],
        message_data: &[u8],
    ) -> Result<String> {
        let mut ssh_args = vec![];

        // Add SSH options
        if let Some(port) = self.port {
            ssh_args.push("-p".to_string());
            ssh_args.push(port.to_string());
        }

        if let Some(identity_file) = &self.identity_file {
            ssh_args.push("-i".to_string());
            ssh_args.push(identity_file.to_string_lossy().to_string());
        }

        // Add SSH connection options for better reliability
        ssh_args.extend([
            "-o".to_string(),
            "BatchMode=yes".to_string(),
            "-o".to_string(),
            "ConnectTimeout=30".to_string(),
            "-o".to_string(),
            "ServerAliveInterval=60".to_string(),
            "-o".to_string(),
            "ServerAliveCountMax=3".to_string(),
        ]);

        // Add user@host
        let connection = if let Some(user) = &self.user {
            format!("{}@{}", user, self.host)
        } else {
            self.host.clone()
        };
        ssh_args.push(connection.clone());

        // Build the msmtp command
        let mut msmtp_cmd_parts = vec![self.msmtp_path.to_string_lossy().to_string()];

        // Add config file if specified
        if let Some(config_path) = &self.config_path {
            msmtp_cmd_parts.push("--file".to_string());
            msmtp_cmd_parts.push(config_path.to_string_lossy().to_string());
        }

        // Add msmtp arguments
        msmtp_cmd_parts.extend(msmtp_args.iter().map(|s| s.to_string()));

        let msmtp_cmd = msmtp_cmd_parts.join(" ");
        ssh_args.push(msmtp_cmd.clone());

        // Log the command being executed
        tracing::debug!(
            "Executing SSH msmtp command to {}: {}",
            connection,
            msmtp_cmd
        );

        let mut child = Command::new("ssh")
            .args(&ssh_args)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|e| NotmuchError::SshError(format!("Failed to spawn SSH: {}", e)))?;

        // Send message data to msmtp via SSH stdin
        if let Some(mut stdin) = child.stdin.take() {
            use tokio::io::AsyncWriteExt;
            stdin.write_all(message_data).await.map_err(|e| {
                NotmuchError::SshError(format!("Failed to write to SSH stdin: {}", e))
            })?;
        }

        let output = child
            .wait_with_output()
            .await
            .map_err(|e| NotmuchError::SshError(format!("SSH msmtp command failed: {}", e)))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            let stdout = String::from_utf8_lossy(&output.stdout);
            tracing::error!("SSH msmtp command failed with status: {:?}", output.status);
            tracing::error!("SSH stderr: {}", stderr);
            tracing::error!("SSH stdout: {}", stdout);
            return Err(NotmuchError::SshError(format!(
                "SSH msmtp command failed: {}",
                stderr
            )));
        }

        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        tracing::debug!(
            "SSH msmtp command output: {}",
            stdout.chars().take(100).collect::<String>()
        );

        Ok(stdout)
    }

    async fn execute_ssh_msmtp_info_command(&self, msmtp_args: &[&str]) -> Result<String> {
        let mut ssh_args = vec![];

        // Add SSH options
        if let Some(port) = self.port {
            ssh_args.push("-p".to_string());
            ssh_args.push(port.to_string());
        }

        if let Some(identity_file) = &self.identity_file {
            ssh_args.push("-i".to_string());
            ssh_args.push(identity_file.to_string_lossy().to_string());
        }

        // Add SSH connection options for better reliability
        ssh_args.extend([
            "-o".to_string(),
            "BatchMode=yes".to_string(),
            "-o".to_string(),
            "ConnectTimeout=30".to_string(),
            "-o".to_string(),
            "ServerAliveInterval=60".to_string(),
            "-o".to_string(),
            "ServerAliveCountMax=3".to_string(),
        ]);

        // Add user@host
        let connection = if let Some(user) = &self.user {
            format!("{}@{}", user, self.host)
        } else {
            self.host.clone()
        };
        ssh_args.push(connection.clone());

        // Build the msmtp command
        let mut msmtp_cmd_parts = vec![self.msmtp_path.to_string_lossy().to_string()];

        // Add config file if specified
        if let Some(config_path) = &self.config_path {
            msmtp_cmd_parts.push("--file".to_string());
            msmtp_cmd_parts.push(config_path.to_string_lossy().to_string());
        }

        // Add msmtp arguments
        msmtp_cmd_parts.extend(msmtp_args.iter().map(|s| s.to_string()));

        let msmtp_cmd = msmtp_cmd_parts.join(" ");
        ssh_args.push(msmtp_cmd.clone());

        // Log the command being executed
        tracing::debug!(
            "Executing SSH msmtp info command to {}: {}",
            connection,
            msmtp_cmd
        );

        let output = Command::new("ssh")
            .args(&ssh_args)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .await
            .map_err(|e| NotmuchError::SshError(format!("SSH command failed: {}", e)))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            let stdout = String::from_utf8_lossy(&output.stdout);
            tracing::error!(
                "SSH msmtp info command failed with status: {:?}",
                output.status
            );
            tracing::error!("SSH stderr: {}", stderr);
            tracing::error!("SSH stdout: {}", stdout);
            return Err(NotmuchError::SshError(format!(
                "SSH msmtp command failed: {}",
                stderr
            )));
        }

        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        tracing::debug!(
            "SSH msmtp info command output: {}",
            stdout.chars().take(500).collect::<String>()
        );

        Ok(stdout)
    }
}

#[async_trait]
impl MailSender for RemoteMsmtpClient {
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

        self.execute_ssh_msmtp_command(&args, &rfc822_data).await?;
        Ok(message.message_id)
    }

    async fn reply(
        &self,
        original: &Message,
        reply: ComposableMessage,
        reply_all: bool,
    ) -> Result<String> {
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
        self.execute_ssh_msmtp_info_command(&["--serverinfo"])
            .await?;
        Ok(())
    }

    async fn get_from_address(&self) -> Result<String> {
        // Get the default from address from msmtp config
        let config_output = self
            .execute_ssh_msmtp_info_command(&["--print-config"])
            .await?;

        // Parse the config output to find the from address
        for line in config_output.lines() {
            if line.starts_with("from") {
                if let Some(from_addr) = line.split_whitespace().nth(1) {
                    return Ok(from_addr.to_string());
                }
            }
        }

        Err(NotmuchError::ConfigError(
            "Could not determine from address from remote msmtp config".to_string(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remote_msmtp_client_config() {
        let config = MailSenderConfig::Remote {
            host: "example.com".to_string(),
            user: Some("testuser".to_string()),
            port: Some(2222),
            identity_file: Some(PathBuf::from("/home/user/.ssh/id_rsa")),
            msmtp_path: Some(PathBuf::from("/usr/local/bin/msmtp")),
            config_path: Some(PathBuf::from("/home/user/.msmtprc")),
        };

        let client = RemoteMsmtpClient::new(config);
        assert!(client.is_ok());

        let client = client.unwrap();
        assert_eq!(client.host, "example.com");
        assert_eq!(client.user, Some("testuser".to_string()));
        assert_eq!(client.port, Some(2222));
        assert_eq!(client.msmtp_path, PathBuf::from("/usr/local/bin/msmtp"));
        assert_eq!(
            client.config_path,
            Some(PathBuf::from("/home/user/.msmtprc"))
        );
    }

    #[test]
    fn test_remote_msmtp_client_default_config() {
        let config = MailSenderConfig::Remote {
            host: "mail.example.com".to_string(),
            user: None,
            port: None,
            identity_file: None,
            msmtp_path: None,
            config_path: None,
        };

        let client = RemoteMsmtpClient::new(config);
        assert!(client.is_ok());

        let client = client.unwrap();
        assert_eq!(client.host, "mail.example.com");
        assert_eq!(client.user, None);
        assert_eq!(client.port, None);
        assert_eq!(client.msmtp_path, PathBuf::from("msmtp"));
        assert_eq!(client.config_path, None);
    }

    #[test]
    fn test_remote_msmtp_client_invalid_config() {
        let config = MailSenderConfig::Local {
            msmtp_path: None,
            config_path: None,
        };

        let result = RemoteMsmtpClient::new(config);
        assert!(result.is_err());
    }
}
