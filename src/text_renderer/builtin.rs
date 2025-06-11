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
    
    /// Convert HTML to plain text using improved text extraction
    /// This implementation handles CSS removal, better formatting, and structure preservation
    fn convert_html_to_text(&self, html: &str) -> TextRendererResult<String> {
        // Remove CSS styles and script content first
        let cleaned_html = self.remove_css_and_scripts(html);
        
        // Strip HTML tags with better structure handling
        let text = self.strip_html_tags_improved(&cleaned_html);
        
        // Clean up whitespace more aggressively
        let cleaned = self.clean_whitespace_improved(&text);
        
        // Wrap text to configured width
        let wrapped = self.wrap_text(&cleaned);
        
        Ok(wrapped)
    }
    
    /// Remove CSS styles, script tags, and other non-content elements
    fn remove_css_and_scripts(&self, html: &str) -> String {
        let mut result = String::new();
        let mut chars = html.chars().peekable();
        let mut in_style_tag = false;
        let mut in_script_tag = false;
        let mut in_tag = false;
        let mut current_tag = String::new();
        
        while let Some(ch) = chars.next() {
            match ch {
                '<' => {
                    in_tag = true;
                    current_tag.clear();
                    
                    // Look ahead to see if this is a style or script tag
                    let mut lookahead = String::new();
                    let mut temp_chars = chars.clone();
                    for _ in 0..10 {
                        if let Some(next_ch) = temp_chars.next() {
                            lookahead.push(next_ch);
                            if next_ch == '>' {
                                break;
                            }
                        }
                    }
                    
                    let lookahead_lower = lookahead.to_lowercase();
                    if lookahead_lower.starts_with("style") {
                        in_style_tag = true;
                    } else if lookahead_lower.starts_with("script") {
                        in_script_tag = true;
                    } else if lookahead_lower.starts_with("/style") {
                        in_style_tag = false;
                        // Skip until we find the closing >
                        while let Some(skip_ch) = chars.next() {
                            if skip_ch == '>' {
                                break;
                            }
                        }
                        continue;
                    } else if lookahead_lower.starts_with("/script") {
                        in_script_tag = false;
                        // Skip until we find the closing >
                        while let Some(skip_ch) = chars.next() {
                            if skip_ch == '>' {
                                break;
                            }
                        }
                        continue;
                    }
                    
                    if !in_style_tag && !in_script_tag {
                        result.push(ch);
                    }
                }
                '>' => {
                    in_tag = false;
                    if !in_style_tag && !in_script_tag {
                        result.push(ch);
                    }
                }
                _ => {
                    if in_tag {
                        current_tag.push(ch);
                    }
                    
                    if !in_style_tag && !in_script_tag {
                        result.push(ch);
                    }
                }
            }
        }
        
        // Remove inline style attributes
        self.remove_inline_styles(&result)
    }
    
    /// Remove inline style attributes from HTML
    fn remove_inline_styles(&self, html: &str) -> String {
        let mut result = String::new();
        let mut chars = html.chars().peekable();
        let mut in_tag = false;
        let mut in_style_attr = false;
        let mut quote_char: Option<char> = None;
        
        while let Some(ch) = chars.next() {
            match ch {
                '<' => {
                    in_tag = true;
                    result.push(ch);
                }
                '>' => {
                    in_tag = false;
                    in_style_attr = false;
                    quote_char = None;
                    result.push(ch);
                }
                _ if in_tag => {
                    // Check if we're starting a style attribute
                    if !in_style_attr && ch == 's' {
                        let mut lookahead = String::new();
                        let mut temp_chars = chars.clone();
                        lookahead.push(ch);
                        for _ in 0..6 {
                            if let Some(next_ch) = temp_chars.next() {
                                lookahead.push(next_ch);
                                if lookahead.len() >= 6 {
                                    break;
                                }
                            }
                        }
                        
                        if lookahead.to_lowercase().starts_with("style=") {
                            in_style_attr = true;
                            // Skip the "style=" part
                            for _ in 0..5 {
                                chars.next();
                            }
                            // Determine quote character
                            if let Some(&next_ch) = chars.peek() {
                                if next_ch == '"' || next_ch == '\'' {
                                    quote_char = Some(next_ch);
                                    chars.next(); // Skip the opening quote
                                }
                            }
                            continue;
                        }
                    }
                    
                    if in_style_attr {
                        // Skip everything until we close the style attribute
                        if let Some(q) = quote_char {
                            if ch == q {
                                in_style_attr = false;
                                quote_char = None;
                            }
                        } else if ch.is_whitespace() {
                            in_style_attr = false;
                        }
                    } else {
                        result.push(ch);
                    }
                }
                _ => {
                    result.push(ch);
                }
            }
        }
        
        result
    }
    
    /// Improved HTML tag stripping with better structure handling
    fn strip_html_tags_improved(&self, html: &str) -> String {
        let mut result = String::new();
        let mut in_tag = false;
        let mut chars = html.chars().peekable();
        
        while let Some(ch) = chars.next() {
            match ch {
                '<' => {
                    in_tag = true;
                    
                    // Look ahead to handle specific tags for better formatting
                    let mut lookahead = String::new();
                    let mut temp_chars = chars.clone();
                    for _ in 0..10 {
                        if let Some(next_ch) = temp_chars.next() {
                            lookahead.push(next_ch.to_ascii_lowercase());
                            if next_ch == '>' || next_ch.is_whitespace() {
                                break;
                            }
                        }
                    }
                    
                    // Add appropriate spacing for different tags
                    match lookahead.as_str() {
                        tag if tag.starts_with("br") || tag.starts_with("br/") => {
                            result.push('\n');
                        }
                        tag if tag.starts_with("p") || tag.starts_with("/p") => {
                            result.push_str("\n\n");
                        }
                        tag if tag.starts_with("div") || tag.starts_with("/div") => {
                            result.push('\n');
                        }
                        tag if tag.starts_with("h1") || tag.starts_with("h2") || 
                               tag.starts_with("h3") || tag.starts_with("h4") ||
                               tag.starts_with("h5") || tag.starts_with("h6") => {
                            result.push_str("\n\n");
                        }
                        tag if tag.starts_with("/h1") || tag.starts_with("/h2") || 
                               tag.starts_with("/h3") || tag.starts_with("/h4") ||
                               tag.starts_with("/h5") || tag.starts_with("/h6") => {
                            result.push_str("\n\n");
                        }
                        tag if tag.starts_with("li") => {
                            result.push_str("\n• ");
                        }
                        tag if tag.starts_with("tr") || tag.starts_with("/tr") => {
                            result.push('\n');
                        }
                        tag if tag.starts_with("td") || tag.starts_with("th") => {
                            result.push('\t');
                        }
                        tag if tag.starts_with("blockquote") => {
                            result.push_str("\n> ");
                        }
                        _ => {}
                    }
                }
                '>' => {
                    in_tag = false;
                }
                _ if !in_tag => {
                    result.push(ch);
                }
                _ => {} // Skip characters inside tags
            }
        }
        
        result
    }
    
    /// Improved whitespace cleanup
    fn clean_whitespace_improved(&self, text: &str) -> String {
        let mut result = String::new();
        let mut prev_was_space = false;
        let mut consecutive_newlines = 0;
        
        for ch in text.chars() {
            match ch {
                '\n' => {
                    consecutive_newlines += 1;
                    if consecutive_newlines <= 2 {
                        result.push('\n');
                        prev_was_space = false;
                    }
                }
                ' ' | '\t' | '\r' => {
                    if !prev_was_space && consecutive_newlines == 0 {
                        result.push(' ');
                        prev_was_space = true;
                    }
                }
                _ => {
                    result.push(ch);
                    prev_was_space = false;
                    consecutive_newlines = 0;
                }
            }
        }
        
        // Remove leading/trailing whitespace and normalize line endings
        result.trim().replace("\n ", "\n").replace(" \n", "\n")
    }
    
    
    /// Wrap text to the configured width with improved handling
    fn wrap_text(&self, text: &str) -> String {
        let mut result = String::new();
        
        for line in text.lines() {
            let trimmed = line.trim();
            if trimmed.is_empty() {
                result.push('\n');
                continue;
            }
            
            // Handle special formatting prefixes
            if trimmed.starts_with('•') || trimmed.starts_with('>') {
                let wrapped_line = self.wrap_line_with_prefix(trimmed, self.config.text_width);
                result.push_str(&wrapped_line);
            } else {
                let wrapped_line = self.wrap_line(trimmed, self.config.text_width);
                result.push_str(&wrapped_line);
            }
            result.push('\n');
        }
        
        result
    }
    
    /// Wrap a line that has a special prefix (like bullet points or quotes)
    fn wrap_line_with_prefix(&self, line: &str, width: usize) -> String {
        if line.len() <= width {
            return line.to_string();
        }
        
        let prefix = if line.starts_with('•') {
            "• "
        } else if line.starts_with('>') {
            "> "
        } else {
            ""
        };
        
        let content = if !prefix.is_empty() {
            &line[prefix.len()..]
        } else {
            line
        };
        
        let indent = " ".repeat(prefix.len());
        let effective_width = width.saturating_sub(prefix.len());
        
        let mut result = String::new();
        let mut first_line = true;
        
        for wrapped_part in self.wrap_content(content, effective_width) {
            if first_line {
                result.push_str(prefix);
                first_line = false;
            } else {
                result.push('\n');
                result.push_str(&indent);
            }
            result.push_str(&wrapped_part);
        }
        
        result
    }
    
    /// Split content into chunks that fit within the given width
    fn wrap_content(&self, content: &str, width: usize) -> Vec<String> {
        let mut lines = Vec::new();
        let mut current_line = String::new();
        
        for word in content.split_whitespace() {
            if current_line.is_empty() {
                current_line = word.to_string();
            } else if current_line.len() + 1 + word.len() <= width {
                current_line.push(' ');
                current_line.push_str(word);
            } else {
                lines.push(current_line);
                current_line = word.to_string();
            }
        }
        
        if !current_line.is_empty() {
            lines.push(current_line);
        }
        
        lines
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
    async fn test_css_removal() {
        let converter = create_test_converter();
        let html = r#"<div style="color: red; font-size: 14px;">Hello <span style="font-weight: bold;">world</span>!</div>"#;
        let result = converter.convert(html).await.unwrap();
        assert!(result.contains("Hello world!"));
        assert!(!result.contains("color: red"));
        assert!(!result.contains("font-size"));
        assert!(!result.contains("style="));
    }
    
    #[tokio::test]
    async fn test_script_and_style_tag_removal() {
        let converter = create_test_converter();
        let html = r#"
            <html>
                <head>
                    <style>
                        body { font-family: Arial; }
                        .header { color: blue; }
                    </style>
                    <script>
                        alert('Hello world');
                        console.log('test');
                    </script>
                </head>
                <body>
                    <p>Visible content</p>
                </body>
            </html>
        "#;
        let result = converter.convert(html).await.unwrap();
        assert!(result.contains("Visible content"));
        assert!(!result.contains("font-family"));
        assert!(!result.contains("alert"));
        assert!(!result.contains("console.log"));
    }
    
    #[tokio::test]
    async fn test_list_formatting() {
        let converter = create_test_converter();
        let html = "<ul><li>First item</li><li>Second item</li></ul>";
        let result = converter.convert(html).await.unwrap();
        assert!(result.contains("• First item"));
        assert!(result.contains("• Second item"));
    }
    
    #[tokio::test]
    async fn test_heading_formatting() {
        let converter = create_test_converter();
        let html = "<h1>Main Title</h1><p>Some content</p><h2>Subtitle</h2>";
        let result = converter.convert(html).await.unwrap();
        assert!(result.contains("Main Title"));
        assert!(result.contains("Subtitle"));
        // Check that headings have proper spacing
        let lines: Vec<&str> = result.lines().collect();
        assert!(lines.len() > 3); // Should have spacing between elements
    }
    
    #[tokio::test]
    async fn test_blockquote_formatting() {
        let converter = create_test_converter();
        let html = "<blockquote>This is a quoted text</blockquote>";
        let result = converter.convert(html).await.unwrap();
        assert!(result.contains("> This is a quoted text"));
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
    fn test_clean_whitespace_improved() {
        let converter = create_test_converter();
        let messy_text = "Hello    world\n\n\nwith   spaces";
        let cleaned = converter.clean_whitespace_improved(messy_text);
        assert!(!cleaned.contains("    "));
        assert!(!cleaned.contains("\n\n\n"));
    }
}