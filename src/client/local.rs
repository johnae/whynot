use async_trait::async_trait;
use std::path::PathBuf;
use std::process::Stdio;
use tokio::process::Command;

use crate::client::{ClientConfig, NotmuchClient, TagOperation};
use crate::error::{NotmuchError, Result};
use crate::search::{SearchItem, SearchResult};
use crate::thread::Thread;

/// A notmuch client that executes commands locally.
///
/// `LocalClient` runs notmuch commands directly on the local system using
/// `tokio::process::Command`. It supports custom database paths via the
/// `NOTMUCH_DATABASE` environment variable.
///
/// # Examples
///
/// ```no_run
/// # use whynot::client::{LocalClient, ClientConfig};
/// # use std::path::PathBuf;
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// // Create a client with default notmuch location
/// let config = ClientConfig::Local {
///     notmuch_path: None,
///     database_path: None,
///     mail_root: None,
/// };
/// let client = LocalClient::new(config)?;
///
/// // Create a client with custom paths
/// let config = ClientConfig::Local {
///     notmuch_path: Some(PathBuf::from("/usr/local/bin/notmuch")),
///     database_path: Some(PathBuf::from("/home/user/.mail/.notmuch")),
///     mail_root: Some(PathBuf::from("/home/user/.mail")),
/// };
/// let client = LocalClient::new(config)?;
/// # Ok(())
/// # }
/// ```
pub struct LocalClient {
    notmuch_path: PathBuf,
    database_path: Option<PathBuf>,
    #[allow(dead_code)]
    mail_root: Option<PathBuf>,
}

impl LocalClient {
    pub fn new(config: ClientConfig) -> Result<Self> {
        match config {
            ClientConfig::Local {
                notmuch_path,
                database_path,
                mail_root,
            } => Ok(LocalClient {
                notmuch_path: notmuch_path.unwrap_or_else(|| PathBuf::from("notmuch")),
                database_path,
                mail_root,
            }),
            _ => Err(NotmuchError::ConfigError(
                "Invalid config type for LocalClient".to_string(),
            )),
        }
    }

    async fn execute_command(&self, args: &[&str]) -> Result<String> {
        let mut cmd = Command::new(&self.notmuch_path);

        if let Some(db_path) = &self.database_path {
            cmd.env("NOTMUCH_DATABASE", db_path);
            let config_path = db_path.join("config");
            if config_path.exists() {
                cmd.env("NOTMUCH_CONFIG", config_path);
            }
        }

        let output = cmd
            .args(args)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .await?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(NotmuchError::CommandFailed(format!(
                "notmuch {} failed: {}",
                args.join(" "),
                stderr
            )));
        }

        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }

    async fn execute_command_bytes(&self, args: &[&str]) -> Result<Vec<u8>> {
        let mut cmd = Command::new(&self.notmuch_path);

        if let Some(db_path) = &self.database_path {
            cmd.env("NOTMUCH_DATABASE", db_path);
            let config_path = db_path.join("config");
            if config_path.exists() {
                cmd.env("NOTMUCH_CONFIG", config_path);
            }
        }

        let output = cmd
            .args(args)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .await?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(NotmuchError::CommandFailed(format!(
                "notmuch {} failed: {}",
                args.join(" "),
                stderr
            )));
        }

        Ok(output.stdout)
    }
}

#[async_trait]
impl NotmuchClient for LocalClient {
    async fn search(&self, query: &str) -> Result<Vec<SearchItem>> {
        let output = self
            .execute_command(&["search", "--format=json", query])
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
            .execute_command(&[
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
        let total_count = match self.execute_command(&["count", query]).await {
            Ok(count_output) => count_output.trim().parse::<usize>().ok(),
            Err(_) => None, // If count fails, continue without total count
        };

        Ok((result.0, total_count))
    }

    async fn show(&self, query: &str) -> Result<Thread> {
        let output = self
            .execute_command(&[
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

        self.execute_command(&args).await?;
        Ok(())
    }

    async fn refresh(&self) -> Result<()> {
        self.execute_command(&["new"]).await?;
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

        let mut cmd = Command::new(&self.notmuch_path);

        if let Some(db_path) = &self.database_path {
            cmd.env("NOTMUCH_DATABASE", db_path);
            let config_path = db_path.join("config");
            if config_path.exists() {
                cmd.env("NOTMUCH_CONFIG", config_path);
            }
        }

        let mut child = cmd
            .args(args)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;

        if let Some(mut stdin) = child.stdin.take() {
            use tokio::io::AsyncWriteExt;
            stdin.write_all(message).await?;
        }

        let output = child.wait_with_output().await?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(NotmuchError::CommandFailed(format!(
                "notmuch insert failed: {}",
                stderr
            )));
        }

        Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
    }

    async fn config_get(&self, key: &str) -> Result<String> {
        let output = self.execute_command(&["config", "get", key]).await?;
        Ok(output.trim().to_string())
    }

    async fn config_set(&self, key: &str, value: &str) -> Result<()> {
        self.execute_command(&["config", "set", key, value]).await?;
        Ok(())
    }

    async fn list_tags(&self) -> Result<Vec<String>> {
        let output = self
            .execute_command(&["search", "--output=tags", "--format=json", "*"])
            .await?;
        // Trim whitespace which might include newlines from notmuch output
        let tags: Vec<String> = serde_json::from_str(output.trim())?;
        Ok(tags)
    }

    async fn part(&self, message_id: &str, part_id: u32) -> Result<Vec<u8>> {
        let part_arg = format!("--part={}", part_id);
        self.execute_command_bytes(&["show", "--format=raw", &part_arg, message_id])
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_tag_operation_formatting() {
        let add_tag = TagOperation::Add("inbox".to_string());
        assert_eq!(add_tag.to_string(), "+inbox");

        let remove_tag = TagOperation::Remove("unread".to_string());
        assert_eq!(remove_tag.to_string(), "-unread");
    }

    #[test]
    fn test_tags_json_parsing() {
        let json_output = r#"["inbox","unread","important","spam","draft"]"#;
        let parsed: std::result::Result<Vec<String>, _> = serde_json::from_str(json_output);
        assert!(parsed.is_ok());

        let tags = parsed.unwrap();
        assert_eq!(tags.len(), 5);
        assert_eq!(tags[0], "inbox");
        assert_eq!(tags[1], "unread");
        assert_eq!(tags[2], "important");
        assert_eq!(tags[3], "spam");
        assert_eq!(tags[4], "draft");
    }

    #[test]
    fn test_empty_tags_json_parsing() {
        let json_output = r#"[]"#;
        let parsed: std::result::Result<Vec<String>, _> = serde_json::from_str(json_output);
        assert!(parsed.is_ok());

        let tags = parsed.unwrap();
        assert_eq!(tags.len(), 0);
    }
}
