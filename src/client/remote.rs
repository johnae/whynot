use async_trait::async_trait;
use std::path::PathBuf;
use std::process::Stdio;
use tokio::process::Command;

use crate::client::{ClientConfig, NotmuchClient, TagOperation};
use crate::error::{NotmuchError, Result};
use crate::search::{SearchItem, SearchResult};
use crate::thread::Thread;

/// A notmuch client that executes commands on a remote host via SSH.
///
/// `RemoteClient` runs notmuch commands on a remote system using SSH.
/// It supports key-based authentication and custom SSH options for
/// reliable connections.
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
/// # use whynot::client::{RemoteClient, ClientConfig};
/// # use std::path::PathBuf;
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// // Connect with default SSH settings
/// let config = ClientConfig::Remote {
///     host: "mail.example.com".to_string(),
///     user: None,
///     port: None,
///     identity_file: None,
///     notmuch_path: None,
/// };
/// let client = RemoteClient::new(config)?;
///
/// // Connect with custom SSH options
/// let config = ClientConfig::Remote {
///     host: "mail.example.com".to_string(),
///     user: Some("alice".to_string()),
///     port: Some(2222),
///     identity_file: Some(PathBuf::from("/home/alice/.ssh/mail_key")),
///     notmuch_path: Some(PathBuf::from("/usr/local/bin/notmuch")),
/// };
/// let client = RemoteClient::new(config)?;
/// # Ok(())
/// # }
/// ```
pub struct RemoteClient {
    host: String,
    user: Option<String>,
    port: Option<u16>,
    identity_file: Option<PathBuf>,
    notmuch_path: PathBuf,
}

impl RemoteClient {
    pub fn new(config: ClientConfig) -> Result<Self> {
        match config {
            ClientConfig::Remote {
                host,
                user,
                port,
                identity_file,
                notmuch_path,
            } => Ok(RemoteClient {
                host,
                user,
                port,
                identity_file,
                notmuch_path: notmuch_path.unwrap_or_else(|| PathBuf::from("notmuch")),
            }),
            _ => Err(NotmuchError::ConfigError(
                "Invalid config type for RemoteClient".to_string(),
            )),
        }
    }

    async fn execute_ssh_command(&self, notmuch_args: &[&str]) -> Result<String> {
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

        // Add the notmuch command
        let notmuch_cmd = format!(
            "{} {}",
            self.notmuch_path.to_string_lossy(),
            notmuch_args.join(" ")
        );
        ssh_args.push(notmuch_cmd.clone());

        // Log the command being executed
        tracing::debug!("Executing SSH command to {}: {}", connection, notmuch_cmd);

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
            tracing::error!("SSH command failed with status: {:?}", output.status);
            tracing::error!("SSH stderr: {}", stderr);
            tracing::error!("SSH stdout: {}", stdout);
            return Err(NotmuchError::SshError(format!(
                "SSH command failed: {}",
                stderr
            )));
        }

        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        tracing::debug!(
            "SSH command output (first 500 chars): {}",
            stdout.chars().take(500).collect::<String>()
        );

        Ok(stdout)
    }

    async fn execute_ssh_command_bytes(&self, notmuch_args: &[&str]) -> Result<Vec<u8>> {
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

        // Add the notmuch command
        let notmuch_cmd = format!(
            "{} {}",
            self.notmuch_path.to_string_lossy(),
            notmuch_args.join(" ")
        );
        ssh_args.push(notmuch_cmd.clone());

        // Log the command being executed
        tracing::debug!(
            "Executing SSH command (binary) to {}: {}",
            connection,
            notmuch_cmd
        );

        let output = Command::new("ssh")
            .args(&ssh_args)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .await?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(NotmuchError::CommandFailed(format!(
                "ssh notmuch {} failed: {}",
                notmuch_args.join(" "),
                stderr
            )));
        }

        Ok(output.stdout)
    }

    async fn execute_ssh_command_with_input(
        &self,
        notmuch_args: &[&str],
        input: &[u8],
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

        // Add SSH connection options
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
        ssh_args.push(connection);

        // Add the notmuch command
        let notmuch_cmd = format!(
            "{} {}",
            self.notmuch_path.to_string_lossy(),
            notmuch_args.join(" ")
        );
        ssh_args.push(notmuch_cmd);

        let mut child = Command::new("ssh")
            .args(&ssh_args)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|e| NotmuchError::SshError(format!("Failed to spawn SSH: {}", e)))?;

        // Write input to stdin
        if let Some(mut stdin) = child.stdin.take() {
            use tokio::io::AsyncWriteExt;
            stdin.write_all(input).await.map_err(|e| {
                NotmuchError::SshError(format!("Failed to write to SSH stdin: {}", e))
            })?;
        }

        let output = child
            .wait_with_output()
            .await
            .map_err(|e| NotmuchError::SshError(format!("SSH command failed: {}", e)))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(NotmuchError::SshError(format!(
                "SSH command failed: {}",
                stderr
            )));
        }

        Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
    }
}

#[async_trait]
impl NotmuchClient for RemoteClient {
    async fn search(&self, query: &str) -> Result<Vec<SearchItem>> {
        let output = self
            .execute_ssh_command(&["search", "--format=json", query])
            .await?;
        let result: SearchResult = serde_json::from_str(&output)?;
        Ok(result.0)
    }

    async fn search_paginated(
        &self,
        query: &str,
        offset: usize,
        limit: usize,
    ) -> Result<(Vec<SearchItem>, Option<usize>)> {
        // For pagination, we need to use notmuch's --offset and --limit flags
        let offset_str = offset.to_string();
        let limit_str = limit.to_string();

        let output = self
            .execute_ssh_command(&[
                "search",
                "--format=json",
                "--offset",
                &offset_str,
                "--limit",
                &limit_str,
                query,
            ])
            .await?;
        let result: SearchResult = serde_json::from_str(&output)?;

        // Get total count by running a count command (separate query)
        // This is optional since it requires an extra query
        let total_count = match self.execute_ssh_command(&["count", query]).await {
            Ok(count_output) => count_output.trim().parse::<usize>().ok(),
            Err(_) => None, // If count fails, continue without total count
        };

        Ok((result.0, total_count))
    }

    async fn show(&self, query: &str) -> Result<Thread> {
        let output = self
            .execute_ssh_command(&[
                "show",
                "--format=json",
                "--include-html",
                "--entire-thread",
                query,
            ])
            .await?;
        let thread: Thread = serde_json::from_str(&output)?;
        Ok(thread)
    }

    async fn tag(&self, query: &str, tags: &[TagOperation]) -> Result<()> {
        if tags.is_empty() {
            return Ok(());
        }

        let tag_args: Vec<String> = tags.iter().map(|t| t.to_string()).collect();
        let mut args = vec!["tag"];
        for tag_arg in &tag_args {
            args.push(tag_arg);
        }
        args.push("--");
        args.push(query);

        self.execute_ssh_command(&args).await?;
        Ok(())
    }

    async fn refresh(&self) -> Result<()> {
        self.execute_ssh_command(&["new"]).await?;
        Ok(())
    }

    async fn insert(&self, message: &[u8], folder: Option<&str>, tags: &[&str]) -> Result<String> {
        let mut args = vec!["insert".to_string()];

        if let Some(folder) = folder {
            args.push("--folder".to_string());
            args.push(folder.to_string());
        }

        for tag in tags {
            args.push(format!("+{}", tag));
        }

        let args_refs: Vec<&str> = args.iter().map(|s| s.as_str()).collect();
        self.execute_ssh_command_with_input(&args_refs, message)
            .await
    }

    async fn config_get(&self, key: &str) -> Result<String> {
        let output = self.execute_ssh_command(&["config", "get", key]).await?;
        Ok(output.trim().to_string())
    }

    async fn config_set(&self, key: &str, value: &str) -> Result<()> {
        self.execute_ssh_command(&["config", "set", key, value])
            .await?;
        Ok(())
    }

    async fn list_tags(&self) -> Result<Vec<String>> {
        tracing::debug!("RemoteClient::list_tags() called");
        let output = self
            .execute_ssh_command(&["search", "--output=tags", "--format=json", "'*'"])
            .await?;
        tracing::debug!("Raw tags output length: {} chars", output.len());
        tracing::debug!("Raw tags output: {}", output);

        // Trim whitespace which might include newlines from notmuch output
        let trimmed_output = output.trim();

        // Handle empty output
        if trimmed_output.is_empty() {
            tracing::warn!("Empty output from notmuch search --output=tags");
            return Ok(Vec::new());
        }

        let tags: Vec<String> = match serde_json::from_str(trimmed_output) {
            Ok(tags) => tags,
            Err(e) => {
                tracing::error!(
                    "Failed to parse tags JSON: {}, raw output: {}",
                    e,
                    trimmed_output
                );
                // Check if it might be an error message instead of JSON
                if !trimmed_output.starts_with('[') {
                    tracing::error!("Output doesn't look like JSON, might be an error message");
                }
                return Err(e.into());
            }
        };

        tracing::info!("Successfully parsed {} tags from remote", tags.len());
        Ok(tags)
    }

    async fn part(&self, message_id: &str, part_id: u32) -> Result<Vec<u8>> {
        let part_arg = format!("--part={}", part_id);
        self.execute_ssh_command_bytes(&["show", "--format=raw", &part_arg, message_id])
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remote_client_config() {
        let config = ClientConfig::Remote {
            host: "example.com".to_string(),
            user: Some("testuser".to_string()),
            port: Some(2222),
            identity_file: Some(PathBuf::from("/home/user/.ssh/id_rsa")),
            notmuch_path: None,
        };

        let client = RemoteClient::new(config);
        assert!(client.is_ok());

        let client = client.unwrap();
        assert_eq!(client.host, "example.com");
        assert_eq!(client.user, Some("testuser".to_string()));
        assert_eq!(client.port, Some(2222));
        assert_eq!(client.notmuch_path, PathBuf::from("notmuch"));
    }

    #[test]
    fn test_remote_client_invalid_config() {
        let config = ClientConfig::Local {
            notmuch_path: None,
            database_path: None,
            mail_root: None,
        };

        let result = RemoteClient::new(config);
        assert!(result.is_err());
    }
}
