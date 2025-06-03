use std::path::PathBuf;
use tempfile::TempDir;
use tokio::process::Command;

use crate::client::{ClientConfig, LocalClient};
use crate::error::{NotmuchError, Result};

pub struct TestNotmuch {
    _temp_dir: TempDir,
    database_path: PathBuf,
    mail_root: PathBuf,
}

impl TestNotmuch {
    pub async fn new() -> Result<Self> {
        let temp_dir = TempDir::new()?;
        let mail_root = temp_dir.path().join("mail");
        let database_path = temp_dir.path().join("notmuch");

        tokio::fs::create_dir_all(&mail_root).await?;
        tokio::fs::create_dir_all(&database_path).await?;

        let mail_cur = mail_root.join("cur");
        let mail_new = mail_root.join("new");
        let mail_tmp = mail_root.join("tmp");

        tokio::fs::create_dir_all(&mail_cur).await?;
        tokio::fs::create_dir_all(&mail_new).await?;
        tokio::fs::create_dir_all(&mail_tmp).await?;

        // Create inbox folder structure for insert operations
        let inbox_cur = mail_root.join("inbox").join("cur");
        let inbox_new = mail_root.join("inbox").join("new");
        let inbox_tmp = mail_root.join("inbox").join("tmp");

        tokio::fs::create_dir_all(&inbox_cur).await?;
        tokio::fs::create_dir_all(&inbox_new).await?;
        tokio::fs::create_dir_all(&inbox_tmp).await?;

        // Create a minimal notmuch config file
        let notmuch_config = format!(
            "[database]\npath={}\nmail_root={}\n\n[user]\nname=Test User\nprimary_email=test@example.com\n",
            database_path.display(),
            mail_root.display()
        );

        let config_path = database_path.join("config");
        tokio::fs::write(&config_path, notmuch_config).await?;

        // Initialize the notmuch database
        let output = Command::new("notmuch")
            .env("NOTMUCH_DATABASE", &database_path)
            .env("NOTMUCH_CONFIG", &config_path)
            .arg("new")
            .output()
            .await?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(NotmuchError::DatabaseError(format!(
                "Failed to initialize notmuch database: {}",
                stderr
            )));
        }

        Ok(TestNotmuch {
            _temp_dir: temp_dir,
            database_path,
            mail_root,
        })
    }

    pub async fn add_mbox(&self, mbox_content: &[u8]) -> Result<()> {
        let mbox_path = self.mail_root.join("test.mbox");
        tokio::fs::write(&mbox_path, mbox_content).await?;

        let output = Command::new("mb2md")
            .arg("-s")
            .arg(&mbox_path)
            .arg("-d")
            .arg(&self.mail_root)
            .output()
            .await;

        match output {
            Ok(output) if output.status.success() => {
                tokio::fs::remove_file(&mbox_path).await?;
            }
            Ok(output) => {
                let stderr = String::from_utf8_lossy(&output.stderr);
                eprintln!("mb2md failed (trying alternative): {}", stderr);

                let messages = parse_mbox_simple(mbox_content);
                for (i, message) in messages.into_iter().enumerate() {
                    let filename = format!(
                        "{}:2,",
                        chrono::Utc::now().timestamp_nanos_opt().unwrap_or(i as i64)
                    );
                    let message_path = self.mail_root.join("cur").join(filename);
                    tokio::fs::write(message_path, message).await?;
                }

                tokio::fs::remove_file(&mbox_path).await.ok();
            }
            Err(_) => {
                eprintln!("mb2md not available, using simple parser");

                let messages = parse_mbox_simple(mbox_content);
                for (i, message) in messages.into_iter().enumerate() {
                    let filename = format!(
                        "{}:2,",
                        chrono::Utc::now().timestamp_nanos_opt().unwrap_or(i as i64)
                    );
                    let message_path = self.mail_root.join("cur").join(filename);
                    tokio::fs::write(message_path, message).await?;
                }

                tokio::fs::remove_file(&mbox_path).await.ok();
            }
        }

        let config_path = self.database_path.join("config");
        let output = Command::new("notmuch")
            .env("NOTMUCH_DATABASE", &self.database_path)
            .env("NOTMUCH_CONFIG", &config_path)
            .arg("new")
            .output()
            .await?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(NotmuchError::DatabaseError(format!(
                "Failed to index messages: {}",
                stderr
            )));
        }

        // Tag new messages with 'inbox' for test purposes
        let output = Command::new("notmuch")
            .env("NOTMUCH_DATABASE", &self.database_path)
            .env("NOTMUCH_CONFIG", &config_path)
            .arg("tag")
            .arg("+inbox")
            .arg("tag:new")
            .output()
            .await?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            eprintln!("Warning: Failed to tag new messages with inbox: {}", stderr);
        }

        Ok(())
    }

    pub fn client(&self) -> LocalClient {
        let config = ClientConfig::Local {
            notmuch_path: None,
            database_path: Some(self.database_path.clone()),
            mail_root: Some(self.mail_root.clone()),
        };

        LocalClient::new(config).expect("Failed to create client")
    }

    pub fn database_path(&self) -> &PathBuf {
        &self.database_path
    }
}

fn parse_mbox_simple(mbox_content: &[u8]) -> Vec<Vec<u8>> {
    let content = String::from_utf8_lossy(mbox_content);
    let mut messages = Vec::new();
    let mut current_message = Vec::new();
    let mut in_message = false;

    for line in content.lines() {
        if line.starts_with("From ") {
            if in_message && !current_message.is_empty() {
                messages.push(current_message);
                current_message = Vec::new();
            }
            in_message = true;
        } else if in_message {
            current_message.extend_from_slice(line.as_bytes());
            current_message.push(b'\n');
        }
    }

    if !current_message.is_empty() {
        messages.push(current_message);
    }

    messages
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::NotmuchClient;
    use crate::test_utils::mbox::{EmailMessage, MboxBuilder};

    #[tokio::test]
    async fn test_notmuch_setup() {
        let test_notmuch = TestNotmuch::new().await;

        if let Err(e) = &test_notmuch {
            eprintln!("TestNotmuch::new() failed: {:?}", e);
        }

        assert!(test_notmuch.is_ok());

        if let Ok(notmuch) = test_notmuch {
            assert!(notmuch.database_path.exists());
            assert!(notmuch.mail_root.exists());
        }
    }

    #[tokio::test]
    async fn test_add_simple_message() {
        let test_notmuch = TestNotmuch::new().await.unwrap();

        let mbox = MboxBuilder::new()
            .add_message(EmailMessage::new("Test Message"))
            .build();

        let result = test_notmuch.add_mbox(&mbox).await;
        assert!(result.is_ok());

        let client = test_notmuch.client();
        let search_results = client.search("*").await.unwrap();
        assert_eq!(search_results.len(), 1);
        assert_eq!(search_results[0].subject, "Test Message");
    }
}
