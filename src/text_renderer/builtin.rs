//! Built-in HTML to text converter implementation
//! 
//! This module provides a native Rust implementation for converting HTML to
//! readable plain text suitable for terminal display.

use super::{HtmlToTextConverter, TextRendererConfig, TextRendererResult};
use async_trait::async_trait;

/// Built-in HTML to text converter using native Rust HTML parsing
pub struct BuiltinConverter {
    config: TextRendererConfig,
}

impl BuiltinConverter {
    /// Create a new built-in converter with the given configuration
    pub fn new(config: TextRendererConfig) -> Self {
        Self { config }
    }
    
    /// Convert HTML to plain text using simple text extraction
    /// This is a basic implementation that strips HTML tags and formats the content
    fn convert_html_to_text(&self, html: &str) -> TextRendererResult<String> {
        // For now, implement a simple version that strips HTML tags
        // TODO: In a future iteration, we'll use a proper HTML parser like scraper
        let text = self.strip_html_tags(html);
        let wrapped = self.wrap_text(&text);
        Ok(wrapped)
    }
    
    /// Simple HTML tag stripping (temporary implementation)
    fn strip_html_tags(&self, html: &str) -> String {
        let mut result = String::new();
        let mut in_tag = false;
        let mut chars = html.chars().peekable();
        
        while let Some(ch) = chars.next() {
            match ch {
                '<' => {
                    in_tag = true;
                    // Handle some common cases for better formatting
                    if self.starts_with_tag(&mut chars, "br") || 
                       self.starts_with_tag(&mut chars, "p") ||
                       self.starts_with_tag(&mut chars, "/p") {
                        result.push('\n');
                    } else if self.starts_with_tag(&mut chars, "h1") ||
                              self.starts_with_tag(&mut chars, "h2") ||
                              self.starts_with_tag(&mut chars, "h3") {
                        result.push_str("\n\n");
                    }
                },
                '>' => {
                    in_tag = false;
                },
                _ if !in_tag => {
                    result.push(ch);
                },
                _ => {} // Skip characters inside tags
            }
        }
        
        // Clean up excessive whitespace
        self.clean_whitespace(&result)
    }
    
    /// Check if the character sequence starts with a specific HTML tag
    fn starts_with_tag(&self, chars: &mut std::iter::Peekable<std::str::Chars>, tag: &str) -> bool {
        let mut temp_chars = Vec::new();
        
        for expected in tag.chars() {
            if let Some(&ch) = chars.peek() {
                temp_chars.push(ch);
                if ch.to_ascii_lowercase() == expected {
                    chars.next();
                } else {
                    // Restore characters we peeked at
                    for _ in temp_chars {
                        // Note: We can't actually restore chars to the iterator
                        // This is a limitation of this simple approach
                    }
                    return false;
                }
            } else {
                return false;
            }
        }
        
        true
    }
    
    /// Clean up excessive whitespace while preserving intentional line breaks
    fn clean_whitespace(&self, text: &str) -> String {
        let mut result = String::new();
        let mut prev_was_space = false;
        let mut prev_was_newline = false;
        
        for ch in text.chars() {
            match ch {
                '\n' => {
                    if !prev_was_newline {
                        result.push('\n');
                        prev_was_newline = true;
                        prev_was_space = false;
                    }
                },
                ' ' | '\t' | '\r' => {
                    if !prev_was_space && !prev_was_newline {
                        result.push(' ');
                        prev_was_space = true;
                    }
                },
                _ => {
                    result.push(ch);
                    prev_was_space = false;
                    prev_was_newline = false;
                }
            }
        }
        
        result.trim().to_string()
    }
    
    /// Wrap text to the configured width
    fn wrap_text(&self, text: &str) -> String {
        let mut result = String::new();
        
        for line in text.lines() {
            if line.trim().is_empty() {
                result.push('\n');
                continue;
            }
            
            let wrapped_line = self.wrap_line(line, self.config.text_width);
            result.push_str(&wrapped_line);
            result.push('\n');
        }
        
        result
    }
    
    /// Wrap a single line to the specified width
    fn wrap_line(&self, line: &str, width: usize) -> String {
        if line.len() <= width {
            return line.to_string();
        }
        
        let mut result = String::new();
        let mut current_line = String::new();
        
        for word in line.split_whitespace() {
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
}

#[async_trait]
impl HtmlToTextConverter for BuiltinConverter {
    async fn convert(&self, html: &str) -> TextRendererResult<String> {
        self.convert_html_to_text(html)
    }
    
    async fn is_available(&self) -> bool {
        // Built-in converter is always available
        true
    }
    
    fn name(&self) -> &'static str {
        "builtin"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    fn create_test_converter() -> BuiltinConverter {
        BuiltinConverter::new(TextRendererConfig::default())
    }
    
    #[tokio::test]
    async fn test_builtin_converter_is_available() {
        let converter = create_test_converter();
        assert!(converter.is_available().await);
        assert_eq!(converter.name(), "builtin");
    }
    
    #[tokio::test]
    async fn test_simple_html_conversion() {
        let converter = create_test_converter();
        let html = "<p>Hello <strong>world</strong>!</p>";
        let result = converter.convert(html).await.unwrap();
        assert!(result.contains("Hello world!"));
    }
    
    #[tokio::test]
    async fn test_html_with_line_breaks() {
        let converter = create_test_converter();
        let html = "<p>First paragraph</p><br><p>Second paragraph</p>";
        let result = converter.convert(html).await.unwrap();
        assert!(result.contains("First paragraph"));
        assert!(result.contains("Second paragraph"));
    }
    
    #[tokio::test]
    async fn test_empty_html() {
        let converter = create_test_converter();
        let result = converter.convert("").await.unwrap();
        assert_eq!(result.trim(), "");
    }
    
    #[tokio::test]
    async fn test_plain_text() {
        let converter = create_test_converter();
        let text = "This is plain text";
        let result = converter.convert(text).await.unwrap();
        assert_eq!(result.trim(), text);
    }
    
    #[test]
    fn test_wrap_line() {
        let converter = create_test_converter();
        let long_line = "This is a very long line that should be wrapped at the specified width";
        let wrapped = converter.wrap_line(long_line, 20);
        
        for line in wrapped.lines() {
            assert!(line.len() <= 20);
        }
    }
    
    #[test]
    fn test_clean_whitespace() {
        let converter = create_test_converter();
        let messy_text = "Hello    world\n\n\nwith   spaces";
        let cleaned = converter.clean_whitespace(messy_text);
        assert!(!cleaned.contains("    "));
        assert!(!cleaned.contains("\n\n\n"));
    }
}