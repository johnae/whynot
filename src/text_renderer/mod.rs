//! HTML to text conversion for terminal display
//!
//! This module provides functionality to convert HTML email content into readable
//! plain text suitable for display in a terminal interface. It supports both
//! built-in Rust-based conversion and external command-line tools.

use async_trait::async_trait;
use std::process::Stdio;
use thiserror::Error;
// use tokio::io::{AsyncReadExt, AsyncWriteExt}; // Will be used in external module
use tokio::process::Command;

pub mod builtin;
pub mod external;

/// Configuration for HTML to text conversion
#[derive(Debug, Clone)]
pub struct TextRendererConfig {
    /// The conversion strategy to use
    pub converter_type: ConverterType,
    /// External tool command (used when converter_type is External)
    pub external_tool_command: Option<String>,
    /// Target text width for wrapping
    pub text_width: usize,
    /// Whether to preserve link URLs inline or as footnotes
    pub preserve_links: bool,
}

impl Default for TextRendererConfig {
    fn default() -> Self {
        Self {
            converter_type: ConverterType::Builtin,
            external_tool_command: None,
            text_width: 80,
            preserve_links: true,
        }
    }
}

/// The type of HTML to text converter to use
#[derive(Debug, Clone, PartialEq)]
pub enum ConverterType {
    /// Use the built-in Rust-based converter
    Builtin,
    /// Use an external command-line tool
    External,
    /// Auto-detect: try external tool first, fallback to built-in
    Auto,
}

/// Errors that can occur during HTML to text conversion
#[derive(Error, Debug)]
pub enum TextRendererError {
    #[error("HTML parsing failed: {0}")]
    ParseError(String),

    #[error("External tool execution failed: {0}")]
    ExternalToolError(String),

    #[error("External tool not found: {0}")]
    ExternalToolNotFound(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Configuration error: {0}")]
    ConfigError(String),
}

/// Result type for text renderer operations
pub type TextRendererResult<T> = Result<T, TextRendererError>;

/// Trait for converting HTML content to readable plain text
#[async_trait]
pub trait HtmlToTextConverter: Send + Sync {
    /// Convert HTML content to plain text
    async fn convert(&self, html: &str) -> TextRendererResult<String>;

    /// Check if this converter is available on the system
    async fn is_available(&self) -> bool;

    /// Get a human-readable name for this converter
    fn name(&self) -> &'static str;
}

/// Factory for creating HTML to text converters
pub struct TextRendererFactory;

impl TextRendererFactory {
    /// Create a converter based on the provided configuration
    pub async fn create_converter(
        config: &TextRendererConfig,
    ) -> TextRendererResult<Box<dyn HtmlToTextConverter>> {
        match config.converter_type {
            ConverterType::Builtin => Ok(Box::new(builtin::BuiltinConverter::new(config.clone()))),
            ConverterType::External => {
                let command = config.external_tool_command.as_ref().ok_or_else(|| {
                    TextRendererError::ConfigError(
                        "External tool command not specified".to_string(),
                    )
                })?;

                let converter =
                    external::ExternalToolConverter::new(command.clone(), config.clone());

                if !converter.is_available().await {
                    return Err(TextRendererError::ExternalToolNotFound(command.clone()));
                }

                Ok(Box::new(converter))
            }
            ConverterType::Auto => {
                // Try external tool first if configured, fallback to built-in
                if let Some(command) = &config.external_tool_command {
                    let external_converter =
                        external::ExternalToolConverter::new(command.clone(), config.clone());
                    if external_converter.is_available().await {
                        return Ok(Box::new(external_converter));
                    }
                }

                // Fallback to built-in
                Ok(Box::new(builtin::BuiltinConverter::new(config.clone())))
            }
        }
    }

    /// Get a list of popular external tools with their commands
    pub fn popular_external_tools() -> Vec<(&'static str, &'static str)> {
        vec![
            ("lynx", "lynx -dump -stdin"),
            ("w3m", "w3m -dump -T text/html"),
            ("html2text", "html2text"),
            ("pandoc", "pandoc -f html -t plain"),
        ]
    }

    /// Detect which external tools are available on the system
    pub async fn detect_available_tools() -> Vec<String> {
        let mut available = Vec::new();

        for (name, command) in Self::popular_external_tools() {
            let tool_name = command.split_whitespace().next().unwrap_or("");

            // Try to run the tool with --version or --help to see if it exists
            if let Ok(output) = Command::new(tool_name)
                .arg("--version")
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .status()
                .await
            {
                if output.success() {
                    available.push(name.to_string());
                }
            }
        }

        available
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_factory_create_builtin_converter() {
        let config = TextRendererConfig::default();
        let converter = TextRendererFactory::create_converter(&config)
            .await
            .unwrap();
        assert_eq!(converter.name(), "builtin");
        assert!(converter.is_available().await);
    }

    #[tokio::test]
    async fn test_factory_external_tool_not_configured() {
        let config = TextRendererConfig {
            converter_type: ConverterType::External,
            external_tool_command: None,
            ..Default::default()
        };

        let result = TextRendererFactory::create_converter(&config).await;
        assert!(matches!(result, Err(TextRendererError::ConfigError(_))));
    }

    #[tokio::test]
    async fn test_detect_available_tools() {
        // This test might find tools on the system, or return empty list
        let _tools = TextRendererFactory::detect_available_tools().await;
        // Just ensure it doesn't panic and returns a vector
        // Note: This assertion is trivial since len() is always >= 0, but kept for clarity
    }

    #[tokio::test]
    async fn test_popular_external_tools_list() {
        let tools = TextRendererFactory::popular_external_tools();
        assert!(!tools.is_empty());
        assert!(tools.iter().any(|(name, _)| *name == "lynx"));
        assert!(tools.iter().any(|(name, _)| *name == "w3m"));
    }
}
