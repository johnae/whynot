//! External tool-based HTML to text converter
//!
//! This module provides HTML to text conversion by delegating to external
//! command-line tools like lynx, w3m, html2text, or pandoc.

use super::{HtmlToTextConverter, TextRendererConfig, TextRendererError, TextRendererResult};
use async_trait::async_trait;
use std::process::Stdio;
use tokio::io::AsyncWriteExt;
use tokio::process::Command;

/// External tool-based HTML to text converter
pub struct ExternalToolConverter {
    command: String,
    config: TextRendererConfig,
}

impl ExternalToolConverter {
    /// Create a new external tool converter with the given command
    pub fn new(command: String, config: TextRendererConfig) -> Self {
        Self { command, config }
    }

    /// Parse the command string into command and arguments
    fn parse_command(&self) -> (String, Vec<String>) {
        let parts: Vec<String> = self
            .command
            .split_whitespace()
            .map(|s| s.to_string())
            .collect();

        if parts.is_empty() {
            ("".to_string(), vec![])
        } else {
            let cmd = parts[0].clone();
            let args = parts[1..].to_vec();
            (cmd, args)
        }
    }

    /// Check if the external tool command is available
    async fn check_tool_availability(&self) -> bool {
        let (cmd, _) = self.parse_command();

        if cmd.is_empty() {
            return false;
        }

        // Try to run the command with --version to see if it exists
        let result = Command::new(&cmd)
            .arg("--version")
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .await;

        match result {
            Ok(status) => status.success(),
            Err(_) => {
                // If --version fails, try --help
                let help_result = Command::new(&cmd)
                    .arg("--help")
                    .stdout(Stdio::null())
                    .stderr(Stdio::null())
                    .status()
                    .await;

                match help_result {
                    Ok(status) => status.success(),
                    Err(_) => false,
                }
            }
        }
    }

    /// Convert HTML to text using the external tool
    async fn convert_with_tool(&self, html: &str) -> TextRendererResult<String> {
        let (cmd, args) = self.parse_command();

        if cmd.is_empty() {
            return Err(TextRendererError::ConfigError(
                "Empty command specified".to_string(),
            ));
        }

        // Start the external process
        let mut child = Command::new(&cmd)
            .args(&args)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|e| {
                TextRendererError::ExternalToolError(format!(
                    "Failed to start command '{}': {}",
                    cmd, e
                ))
            })?;

        // Write HTML to stdin
        if let Some(stdin) = child.stdin.as_mut() {
            stdin.write_all(html.as_bytes()).await.map_err(|e| {
                TextRendererError::ExternalToolError(format!("Failed to write to stdin: {}", e))
            })?;

            // Close stdin to signal end of input
            drop(child.stdin.take());
        }

        // Wait for the process to complete and collect output
        let output = child.wait_with_output().await.map_err(|e| {
            TextRendererError::ExternalToolError(format!("Failed to wait for command: {}", e))
        })?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(TextRendererError::ExternalToolError(format!(
                "Command failed with status {}: {}",
                output.status, stderr
            )));
        }

        let text = String::from_utf8(output.stdout).map_err(|e| {
            TextRendererError::ExternalToolError(format!("Invalid UTF-8 output: {}", e))
        })?;

        // Apply any post-processing based on configuration
        Ok(self.post_process_text(&text))
    }

    /// Apply post-processing to the converted text
    fn post_process_text(&self, text: &str) -> String {
        let mut result = text.to_string();

        // If text width is configured and different from default, re-wrap
        if self.config.text_width != 80 {
            result = self.rewrap_text(&result, self.config.text_width);
        }

        // Clean up excessive blank lines
        result = self.clean_excessive_newlines(&result);

        result.trim().to_string()
    }

    /// Re-wrap text to the specified width
    fn rewrap_text(&self, text: &str, width: usize) -> String {
        let mut result = String::new();

        for paragraph in text.split("\n\n") {
            if paragraph.trim().is_empty() {
                result.push_str("\n\n");
                continue;
            }

            // Join lines within paragraph and re-wrap
            let joined = paragraph
                .lines()
                .map(|line| line.trim())
                .filter(|line| !line.is_empty())
                .collect::<Vec<_>>()
                .join(" ");

            let wrapped = self.wrap_paragraph(&joined, width);
            result.push_str(&wrapped);
            result.push_str("\n\n");
        }

        result
    }

    /// Wrap a paragraph to the specified width
    fn wrap_paragraph(&self, paragraph: &str, width: usize) -> String {
        if paragraph.len() <= width {
            return paragraph.to_string();
        }

        let mut result = String::new();
        let mut current_line = String::new();

        for word in paragraph.split_whitespace() {
            if current_line.is_empty() {
                current_line = word.to_string();
            } else if current_line.len() + 1 + word.len() <= width {
                current_line.push(' ');
                current_line.push_str(word);
            } else {
                result.push_str(&current_line);
                result.push('\n');
                current_line = word.to_string();
            }
        }

        if !current_line.is_empty() {
            result.push_str(&current_line);
        }

        result
    }

    /// Clean up excessive newlines
    fn clean_excessive_newlines(&self, text: &str) -> String {
        // Replace more than 2 consecutive newlines with exactly 2
        let mut result = String::new();
        let mut newline_count = 0;

        for ch in text.chars() {
            if ch == '\n' {
                newline_count += 1;
                if newline_count <= 2 {
                    result.push(ch);
                }
            } else {
                newline_count = 0;
                result.push(ch);
            }
        }

        result
    }
}

#[async_trait]
impl HtmlToTextConverter for ExternalToolConverter {
    async fn convert(&self, html: &str) -> TextRendererResult<String> {
        self.convert_with_tool(html).await
    }

    async fn is_available(&self) -> bool {
        self.check_tool_availability().await
    }

    fn name(&self) -> &'static str {
        "external"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_converter(command: &str) -> ExternalToolConverter {
        ExternalToolConverter::new(command.to_string(), TextRendererConfig::default())
    }

    #[test]
    fn test_parse_command() {
        let converter = create_test_converter("lynx -dump -stdin");
        let (cmd, args) = converter.parse_command();
        assert_eq!(cmd, "lynx");
        assert_eq!(args, vec!["-dump", "-stdin"]);
    }

    #[test]
    fn test_parse_empty_command() {
        let converter = create_test_converter("");
        let (cmd, args) = converter.parse_command();
        assert_eq!(cmd, "");
        assert!(args.is_empty());
    }

    #[tokio::test]
    async fn test_nonexistent_tool_not_available() {
        let converter = create_test_converter("definitely-not-a-real-command");
        assert!(!converter.is_available().await);
    }

    #[tokio::test]
    async fn test_empty_command_not_available() {
        let converter = create_test_converter("");
        assert!(!converter.is_available().await);
    }

    #[tokio::test]
    async fn test_convert_with_invalid_command() {
        let converter = create_test_converter("definitely-not-a-real-command");
        let result = converter.convert("<p>Test</p>").await;
        assert!(matches!(
            result,
            Err(TextRendererError::ExternalToolError(_))
        ));
    }

    #[test]
    fn test_rewrap_text() {
        let converter = create_test_converter("echo");
        let text = "This is a very long line that should be wrapped at a specific width for better readability";
        let wrapped = converter.rewrap_text(text, 20);

        for line in wrapped.lines() {
            if !line.trim().is_empty() {
                assert!(line.len() <= 20, "Line too long: '{}'", line);
            }
        }
    }

    #[test]
    fn test_clean_excessive_newlines() {
        let converter = create_test_converter("echo");
        let text = "Line 1\n\n\n\n\nLine 2\n\n\nLine 3";
        let cleaned = converter.clean_excessive_newlines(text);
        assert!(!cleaned.contains("\n\n\n"));
        assert!(cleaned.contains("\n\n"));
    }

    #[test]
    fn test_wrap_paragraph() {
        let converter = create_test_converter("echo");
        let paragraph = "This is a test paragraph that should be wrapped";
        let wrapped = converter.wrap_paragraph(paragraph, 10);

        for line in wrapped.lines() {
            assert!(line.len() <= 10);
        }
    }
}
