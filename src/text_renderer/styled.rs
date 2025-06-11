//! HTML to styled text conversion for rich terminal display
//!
//! This module converts HTML content into styled ratatui::text::Text
//! with proper formatting (bold, italic, colors, etc.)

use crate::text_renderer::{HtmlToTextConverter, TextRendererConfig, TextRendererResult};
use async_trait::async_trait;
use ratatui::{
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
};

/// A converter that produces styled ratatui Text from HTML
pub struct StyledTextConverter {
    _config: TextRendererConfig,
}

impl StyledTextConverter {
    pub fn new(config: TextRendererConfig) -> Self {
        Self { _config: config }
    }

    /// Convert HTML to styled ratatui Text
    pub fn convert_to_styled_text(&self, html: &str) -> TextRendererResult<Text<'static>> {
        let mut parser = HtmlParser::new();
        parser.parse(html)
    }
}

#[async_trait]
impl HtmlToTextConverter for StyledTextConverter {
    async fn convert(&self, html: &str) -> TextRendererResult<String> {
        // For compatibility with the trait, convert styled text back to plain string
        let styled_text = self.convert_to_styled_text(html)?;
        Ok(styled_text.to_string())
    }

    async fn is_available(&self) -> bool {
        true // Always available since it's built-in
    }

    fn name(&self) -> &'static str {
        "styled"
    }
}

/// Simple HTML parser that tracks styling state
struct HtmlParser {
    style_stack: Vec<Style>,
    current_style: Style,
    spans: Vec<Span<'static>>,
    lines: Vec<Line<'static>>,
    current_text: String,
}

impl HtmlParser {
    fn new() -> Self {
        Self {
            style_stack: Vec::new(),
            current_style: Style::default(),
            spans: Vec::new(),
            lines: Vec::new(),
            current_text: String::new(),
        }
    }

    fn parse(&mut self, html: &str) -> TextRendererResult<Text<'static>> {
        // Remove CSS and scripts first (reuse logic from builtin converter)
        let cleaned_html = self.remove_css_and_scripts(html);
        
        // Simple state machine to parse HTML
        let mut chars = cleaned_html.chars().peekable();
        
        while let Some(ch) = chars.next() {
            if ch == '<' {
                // Save current text if any
                self.flush_current_text();
                
                // Parse HTML tag
                let mut tag = String::new();
                while let Some(&next_ch) = chars.peek() {
                    if next_ch == '>' {
                        chars.next(); // consume '>'
                        break;
                    }
                    tag.push(chars.next().unwrap());
                }
                
                self.handle_tag(&tag);
            } else if ch == '&' {
                // Handle HTML entities
                let mut entity = String::new();
                let mut found_semicolon = false;
                
                for _ in 0..10 {
                    if let Some(&next_ch) = chars.peek() {
                        if next_ch == ';' {
                            chars.next();
                            found_semicolon = true;
                            break;
                        } else if next_ch.is_alphanumeric() || next_ch == '#' {
                            entity.push(chars.next().unwrap());
                        } else {
                            break;
                        }
                    } else {
                        break;
                    }
                }
                
                if found_semicolon {
                    if let Some(decoded) = self.decode_entity(&entity) {
                        self.current_text.push_str(&decoded);
                    } else {
                        self.current_text.push('&');
                        self.current_text.push_str(&entity);
                        self.current_text.push(';');
                    }
                } else {
                    self.current_text.push('&');
                    self.current_text.push_str(&entity);
                }
            } else {
                self.current_text.push(ch);
            }
        }
        
        // Flush any remaining text
        self.flush_current_text();
        
        // Create final line if we have spans
        if !self.spans.is_empty() {
            self.lines.push(Line::from(std::mem::take(&mut self.spans)));
        }
        
        // If no lines, create empty line
        if self.lines.is_empty() {
            self.lines.push(Line::from(""));
        }
        
        Ok(Text::from(self.lines.clone()))
    }

    fn flush_current_text(&mut self) {
        if !self.current_text.is_empty() {
            let text = std::mem::take(&mut self.current_text);
            let span = Span::styled(text, self.current_style);
            self.spans.push(span);
        }
    }

    fn handle_tag(&mut self, tag: &str) {
        let tag = tag.trim();
        
        if tag.starts_with('/') {
            // Closing tag
            self.handle_closing_tag(&tag[1..]);
        } else {
            // Opening tag (might have attributes)
            let tag_name = tag.split_whitespace().next().unwrap_or(tag);
            self.handle_opening_tag(tag_name, tag);
        }
    }

    fn handle_opening_tag(&mut self, tag_name: &str, _full_tag: &str) {
        // Save current style to stack
        self.style_stack.push(self.current_style);
        
        match tag_name.to_lowercase().as_str() {
            "b" | "strong" => {
                self.current_style = self.current_style.add_modifier(Modifier::BOLD);
            }
            "i" | "em" => {
                self.current_style = self.current_style.add_modifier(Modifier::ITALIC);
            }
            "u" => {
                self.current_style = self.current_style.add_modifier(Modifier::UNDERLINED);
            }
            "a" => {
                self.current_style = self.current_style
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::UNDERLINED);
            }
            "h1" | "h2" | "h3" | "h4" | "h5" | "h6" => {
                self.current_style = self.current_style.add_modifier(Modifier::BOLD);
            }
            "code" | "pre" => {
                self.current_style = self.current_style.bg(Color::DarkGray);
            }
            "blockquote" => {
                self.current_style = self.current_style
                    .fg(Color::Gray)
                    .add_modifier(Modifier::ITALIC);
            }
            "p" => {
                // Add paragraph break
                self.add_line_break();
            }
            "br" => {
                // Line break
                self.add_line_break();
                // br is self-closing, so restore style
                if let Some(previous_style) = self.style_stack.pop() {
                    self.current_style = previous_style;
                }
            }
            "li" => {
                // List item - add bullet
                self.add_line_break();
                self.current_text.push_str("• ");
            }
            _ => {
                // Unknown tag, ignore but keep style stack consistent
            }
        }
    }

    fn handle_closing_tag(&mut self, tag_name: &str) {
        match tag_name.to_lowercase().as_str() {
            "p" | "div" | "h1" | "h2" | "h3" | "h4" | "h5" | "h6" | "blockquote" => {
                // Block elements add line breaks
                self.add_line_break();
            }
            "li" => {
                // End of list item
                self.add_line_break();
            }
            _ => {
                // Inline elements
            }
        }
        
        // Restore previous style from stack
        if let Some(previous_style) = self.style_stack.pop() {
            self.current_style = previous_style;
        }
    }

    fn add_line_break(&mut self) {
        self.flush_current_text();
        if !self.spans.is_empty() {
            self.lines.push(Line::from(std::mem::take(&mut self.spans)));
        }
    }

    fn remove_css_and_scripts(&self, html: &str) -> String {
        // Simple CSS/script removal (reuse logic from builtin converter)
        let mut result = String::new();
        let mut chars = html.chars().peekable();
        let mut in_style = false;
        let mut in_script = false;
        
        while let Some(ch) = chars.next() {
            if ch == '<' {
                let mut tag = String::new();
                while let Some(&next_ch) = chars.peek() {
                    if next_ch == '>' {
                        chars.next();
                        break;
                    }
                    tag.push(chars.next().unwrap());
                }
                
                let tag_lower = tag.to_lowercase();
                if tag_lower.starts_with("style") {
                    in_style = true;
                } else if tag_lower == "/style" {
                    in_style = false;
                } else if tag_lower.starts_with("script") {
                    in_script = true;
                } else if tag_lower == "/script" {
                    in_script = false;
                } else if !in_style && !in_script {
                    result.push('<');
                    result.push_str(&tag);
                    result.push('>');
                }
            } else if !in_style && !in_script {
                result.push(ch);
            }
        }
        
        result
    }

    fn decode_entity(&self, entity: &str) -> Option<String> {
        match entity {
            "nbsp" => Some(" ".to_string()),
            "amp" => Some("&".to_string()),
            "lt" => Some("<".to_string()),
            "gt" => Some(">".to_string()),
            "quot" => Some("\"".to_string()),
            "apos" | "#39" => Some("'".to_string()),
            // Numeric entities
            entity if entity.starts_with('#') => {
                if let Some(num_str) = entity.strip_prefix('#') {
                    if let Some(hex_str) = num_str.strip_prefix('x') {
                        // Hexadecimal
                        if let Ok(code) = u32::from_str_radix(hex_str, 16) {
                            if let Some(ch) = char::from_u32(code) {
                                return Some(ch.to_string());
                            }
                        }
                    } else {
                        // Decimal
                        if let Ok(code) = num_str.parse::<u32>() {
                            if let Some(ch) = char::from_u32(code) {
                                return Some(ch.to_string());
                            }
                        }
                    }
                }
                None
            }
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn default_config() -> TextRendererConfig {
        TextRendererConfig::default()
    }

    #[test]
    fn test_bold_text_conversion() {
        let converter = StyledTextConverter::new(default_config());
        let html = "<b>Bold text</b>";
        let result = converter.convert_to_styled_text(html).unwrap();
        
        assert_eq!(result.lines.len(), 1);
        let line = &result.lines[0];
        assert_eq!(line.spans.len(), 1);
        let span = &line.spans[0];
        assert_eq!(span.content, "Bold text");
        assert!(span.style.add_modifier.contains(Modifier::BOLD));
    }

    #[test]
    fn test_italic_text_conversion() {
        let converter = StyledTextConverter::new(default_config());
        let html = "<i>Italic text</i>";
        let result = converter.convert_to_styled_text(html).unwrap();
        
        assert_eq!(result.lines.len(), 1);
        let line = &result.lines[0];
        assert_eq!(line.spans.len(), 1);
        let span = &line.spans[0];
        assert_eq!(span.content, "Italic text");
        assert!(span.style.add_modifier.contains(Modifier::ITALIC));
    }

    #[test]
    fn test_underlined_text_conversion() {
        let converter = StyledTextConverter::new(default_config());
        let html = "<u>Underlined text</u>";
        let result = converter.convert_to_styled_text(html).unwrap();
        
        assert_eq!(result.lines.len(), 1);
        let line = &result.lines[0];
        assert_eq!(line.spans.len(), 1);
        let span = &line.spans[0];
        assert_eq!(span.content, "Underlined text");
        assert!(span.style.add_modifier.contains(Modifier::UNDERLINED));
    }

    #[test]
    fn test_link_conversion() {
        let converter = StyledTextConverter::new(default_config());
        let html = r#"<a href="https://example.com">Link text</a>"#;
        let result = converter.convert_to_styled_text(html).unwrap();
        
        assert_eq!(result.lines.len(), 1);
        let line = &result.lines[0];
        assert_eq!(line.spans.len(), 1);
        let span = &line.spans[0];
        assert_eq!(span.content, "Link text");
        assert_eq!(span.style.fg, Some(Color::Cyan));
        assert!(span.style.add_modifier.contains(Modifier::UNDERLINED));
    }

    #[test]
    fn test_nested_styles() {
        let converter = StyledTextConverter::new(default_config());
        let html = "<b><i>Bold and italic</i></b>";
        let result = converter.convert_to_styled_text(html).unwrap();
        
        assert_eq!(result.lines.len(), 1);
        let line = &result.lines[0];
        assert_eq!(line.spans.len(), 1);
        let span = &line.spans[0];
        assert_eq!(span.content, "Bold and italic");
        assert!(span.style.add_modifier.contains(Modifier::BOLD));
        assert!(span.style.add_modifier.contains(Modifier::ITALIC));
    }

    #[test]
    fn test_mixed_content() {
        let converter = StyledTextConverter::new(default_config());
        let html = "Normal <b>bold</b> and <i>italic</i> text";
        let result = converter.convert_to_styled_text(html).unwrap();
        
        assert_eq!(result.lines.len(), 1);
        let line = &result.lines[0];
        assert_eq!(line.spans.len(), 5);
        
        assert_eq!(line.spans[0].content, "Normal ");
        assert_eq!(line.spans[0].style, Style::default());
        
        assert_eq!(line.spans[1].content, "bold");
        assert!(line.spans[1].style.add_modifier.contains(Modifier::BOLD));
        
        assert_eq!(line.spans[2].content, " and ");
        assert_eq!(line.spans[2].style, Style::default());
        
        assert_eq!(line.spans[3].content, "italic");
        assert!(line.spans[3].style.add_modifier.contains(Modifier::ITALIC));
        
        assert_eq!(line.spans[4].content, " text");
        assert_eq!(line.spans[4].style, Style::default());
    }

    #[test]
    fn test_headers() {
        let converter = StyledTextConverter::new(default_config());
        
        // H1 should be bold and maybe a different color
        let html = "<h1>Header 1</h1>";
        let result = converter.convert_to_styled_text(html).unwrap();
        assert_eq!(result.lines.len(), 1);
        let span = &result.lines[0].spans[0];
        assert_eq!(span.content, "Header 1");
        assert!(span.style.add_modifier.contains(Modifier::BOLD));
        
        // H2 should also be styled
        let html = "<h2>Header 2</h2>";
        let result = converter.convert_to_styled_text(html).unwrap();
        let span = &result.lines[0].spans[0];
        assert_eq!(span.content, "Header 2");
        assert!(span.style.add_modifier.contains(Modifier::BOLD));
    }

    #[test]
    fn test_code_blocks() {
        let converter = StyledTextConverter::new(default_config());
        let html = "<code>inline code</code>";
        let result = converter.convert_to_styled_text(html).unwrap();
        
        assert_eq!(result.lines.len(), 1);
        let span = &result.lines[0].spans[0];
        assert_eq!(span.content, "inline code");
        // Code should have a different style (e.g., different background or color)
        assert_ne!(span.style, Style::default());
    }

    #[test]
    fn test_blockquote() {
        let converter = StyledTextConverter::new(default_config());
        let html = "<blockquote>This is a quote</blockquote>";
        let result = converter.convert_to_styled_text(html).unwrap();
        
        assert_eq!(result.lines.len(), 1);
        let span = &result.lines[0].spans[0];
        assert!(span.content.contains("This is a quote"));
        // Blockquotes might be dimmed or italic
        assert_ne!(span.style, Style::default());
    }

    #[test]
    fn test_html_entities() {
        let converter = StyledTextConverter::new(default_config());
        let html = "<b>&amp; &lt; &gt; &nbsp;</b>";
        let result = converter.convert_to_styled_text(html).unwrap();
        
        let span = &result.lines[0].spans[0];
        assert_eq!(span.content, "& < >  "); // nbsp becomes space
        assert!(span.style.add_modifier.contains(Modifier::BOLD));
    }

    #[test]
    fn test_paragraph_separation() {
        let converter = StyledTextConverter::new(default_config());
        let html = "<p>First paragraph</p><p>Second paragraph</p>";
        let result = converter.convert_to_styled_text(html).unwrap();
        
        // Should have multiple lines with paragraph separation
        assert!(result.lines.len() >= 2);
        assert!(result.lines.iter().any(|line| 
            line.spans.iter().any(|span| span.content.contains("First paragraph"))
        ));
        assert!(result.lines.iter().any(|line| 
            line.spans.iter().any(|span| span.content.contains("Second paragraph"))
        ));
    }

    #[test]
    fn test_list_formatting() {
        let converter = StyledTextConverter::new(default_config());
        let html = "<ul><li>Item 1</li><li>Item 2</li></ul>";
        let result = converter.convert_to_styled_text(html).unwrap();
        
        // Lists should be formatted with bullets or numbers
        let text_content = result.to_string();
        assert!(text_content.contains("Item 1"));
        assert!(text_content.contains("Item 2"));
    }

    #[test]
    fn test_strong_and_em_tags() {
        let converter = StyledTextConverter::new(default_config());
        
        // <strong> should behave like <b>
        let html = "<strong>Strong text</strong>";
        let result = converter.convert_to_styled_text(html).unwrap();
        let span = &result.lines[0].spans[0];
        assert_eq!(span.content, "Strong text");
        assert!(span.style.add_modifier.contains(Modifier::BOLD));
        
        // <em> should behave like <i>
        let html = "<em>Emphasized text</em>";
        let result = converter.convert_to_styled_text(html).unwrap();
        let span = &result.lines[0].spans[0];
        assert_eq!(span.content, "Emphasized text");
        assert!(span.style.add_modifier.contains(Modifier::ITALIC));
    }

    #[test]
    fn test_complex_nested_html() {
        let converter = StyledTextConverter::new(default_config());
        let html = r#"
            <div>
                <h1>Title</h1>
                <p>This is a <b>paragraph</b> with <i>mixed</i> <u>styles</u>.</p>
                <blockquote>
                    A <em>quoted</em> text
                </blockquote>
            </div>
        "#;
        let result = converter.convert_to_styled_text(html).unwrap();
        
        // Should handle the complex structure
        let text_content = result.to_string();
        assert!(text_content.contains("Title"));
        assert!(text_content.contains("paragraph"));
        assert!(text_content.contains("mixed"));
        assert!(text_content.contains("styles"));
        assert!(text_content.contains("quoted"));
    }

    #[tokio::test]
    async fn test_html_to_text_converter_trait() {
        let converter = StyledTextConverter::new(default_config());
        
        // Test the trait implementation
        assert_eq!(converter.name(), "styled");
        assert!(converter.is_available().await);
        
        // Test that convert returns a plain string version
        let html = "<b>Bold</b> and <i>italic</i>";
        let result = converter.convert(html).await.unwrap();
        assert!(result.contains("Bold"));
        assert!(result.contains("italic"));
    }
}