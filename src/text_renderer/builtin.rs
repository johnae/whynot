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

        // Decode HTML entities
        let decoded = self.decode_html_entities(&text);

        // Clean up whitespace more aggressively
        let cleaned = self.clean_whitespace_improved(&decoded);

        // Wrap text to configured width
        let wrapped = self.wrap_text(&cleaned);

        Ok(wrapped)
    }

    /// Decode HTML entities to their corresponding characters
    fn decode_html_entities(&self, text: &str) -> String {
        let mut result = String::new();
        let mut chars = text.chars().peekable();

        while let Some(ch) = chars.next() {
            if ch == '&' {
                // Collect the entity
                let mut entity = String::new();
                let mut found_semicolon = false;

                // Look ahead up to 10 characters for the entity
                for _ in 0..10 {
                    if let Some(&next_ch) = chars.peek() {
                        if next_ch == ';' {
                            chars.next(); // Consume the semicolon
                            found_semicolon = true;
                            break;
                        } else if next_ch.is_alphanumeric() || next_ch == '#' {
                            entity.push(chars.next().unwrap());
                        } else {
                            // Invalid entity, break
                            break;
                        }
                    } else {
                        break;
                    }
                }

                if found_semicolon {
                    // Try to decode the entity
                    if let Some(decoded_char) = self.decode_entity(&entity) {
                        result.push_str(&decoded_char);
                    } else {
                        // Unknown entity, keep as-is
                        result.push('&');
                        result.push_str(&entity);
                        result.push(';');
                    }
                } else {
                    // No semicolon found, treat as literal &
                    result.push('&');
                    result.push_str(&entity);
                }
            } else {
                result.push(ch);
            }
        }

        result
    }

    /// Decode a specific HTML entity (without & and ;)
    fn decode_entity(&self, entity: &str) -> Option<String> {
        match entity {
            // Common named entities
            "nbsp" => Some(" ".to_string()),
            "amp" => Some("&".to_string()),
            "lt" => Some("<".to_string()),
            "gt" => Some(">".to_string()),
            "quot" => Some("\"".to_string()),
            "apos" | "#39" => Some("'".to_string()),
            "copy" => Some("©".to_string()),
            "reg" => Some("®".to_string()),
            "trade" => Some("™".to_string()),
            "mdash" => Some("—".to_string()),
            "ndash" => Some("–".to_string()),
            "hellip" => Some("…".to_string()),
            "laquo" => Some("«".to_string()),
            "raquo" => Some("»".to_string()),
            "ldquo" => Some("\u{201C}".to_string()), // Left double quotation mark
            "rdquo" => Some("\u{201D}".to_string()), // Right double quotation mark
            "lsquo" => Some("\u{2018}".to_string()), // Left single quotation mark
            "rsquo" => Some("\u{2019}".to_string()), // Right single quotation mark

            // Zero-width and formatting characters
            "shy" => Some("\u{00AD}".to_string()),     // Soft hyphen
            "zwnj" => Some("\u{200C}".to_string()),    // Zero width non-joiner
            "zwj" => Some("\u{200D}".to_string()),     // Zero width joiner

            // Spaces
            "thinsp" => Some("\u{2009}".to_string()),  // Thin space
            "ensp" => Some("\u{2002}".to_string()),    // En space
            "emsp" => Some("\u{2003}".to_string()),    // Em space

            // Mathematical symbols
            "times" => Some("×".to_string()),          // Multiplication sign
            "divide" => Some("÷".to_string()),         // Division sign
            "plusmn" => Some("±".to_string()),         // Plus minus
            "deg" => Some("°".to_string()),            // Degree symbol

            // Currency symbols
            "euro" => Some("€".to_string()),           // Euro symbol
            "pound" => Some("£".to_string()),          // Pound symbol
            "yen" => Some("¥".to_string()),            // Yen symbol
            "cent" => Some("¢".to_string()),           // Cent symbol

            // Fractions
            "frac12" => Some("½".to_string()),         // One half
            "frac14" => Some("¼".to_string()),         // One quarter
            "frac34" => Some("¾".to_string()),         // Three quarters

            // Additional common entities
            "sup1" => Some("¹".to_string()),           // Superscript 1
            "sup2" => Some("²".to_string()),           // Superscript 2
            "sup3" => Some("³".to_string()),           // Superscript 3
            "ordm" => Some("º".to_string()),           // Masculine ordinal
            "ordf" => Some("ª".to_string()),           // Feminine ordinal
            "sect" => Some("§".to_string()),           // Section sign
            "para" => Some("¶".to_string()),           // Pilcrow sign
            "micro" => Some("µ".to_string()),          // Micro sign
            "middot" => Some("·".to_string()),         // Middle dot
            "cedil" => Some("¸".to_string()),          // Cedilla
            "acute" => Some("´".to_string()),          // Acute accent
            "uml" => Some("¨".to_string()),            // Diaeresis
            "macr" => Some("¯".to_string()),           // Macron
            "not" => Some("¬".to_string()),            // Not sign
            "brvbar" => Some("¦".to_string()),         // Broken bar
            "curren" => Some("¤".to_string()),         // Currency sign

            // Nordic/Scandinavian characters (Swedish, Norwegian, Danish, Finnish)
            "Aring" => Some("Å".to_string()),          // Latin Capital Letter A with Ring Above
            "aring" => Some("å".to_string()),          // Latin Small Letter A with Ring Above
            "Auml" => Some("Ä".to_string()),           // Latin Capital Letter A with Diaeresis
            "auml" => Some("ä".to_string()),           // Latin Small Letter A with Diaeresis
            "Ouml" => Some("Ö".to_string()),           // Latin Capital Letter O with Diaeresis
            "ouml" => Some("ö".to_string()),           // Latin Small Letter O with Diaeresis
            "AElig" => Some("Æ".to_string()),          // Latin Capital Letter Æ
            "aelig" => Some("æ".to_string()),          // Latin Small Letter æ
            "Oslash" => Some("Ø".to_string()),         // Latin Capital Letter O with Stroke
            "oslash" => Some("ø".to_string()),         // Latin Small Letter O with Stroke

            // German characters
            "Uuml" => Some("Ü".to_string()),           // Latin Capital Letter U with Diaeresis
            "uuml" => Some("ü".to_string()),           // Latin Small Letter U with Diaeresis
            "szlig" => Some("ß".to_string()),          // Latin Small Letter Sharp S

            // French characters
            "Agrave" => Some("À".to_string()),         // Latin Capital Letter A with Grave
            "agrave" => Some("à".to_string()),         // Latin Small Letter A with Grave
            "Aacute" => Some("Á".to_string()),         // Latin Capital Letter A with Acute
            "aacute" => Some("á".to_string()),         // Latin Small Letter A with Acute
            "Acirc" => Some("Â".to_string()),          // Latin Capital Letter A with Circumflex
            "acirc" => Some("â".to_string()),          // Latin Small Letter A with Circumflex
            "Atilde" => Some("Ã".to_string()),         // Latin Capital Letter A with Tilde
            "atilde" => Some("ã".to_string()),         // Latin Small Letter A with Tilde
            "Ccedil" => Some("Ç".to_string()),         // Latin Capital Letter C with Cedilla
            "ccedil" => Some("ç".to_string()),         // Latin Small Letter C with Cedilla
            "Egrave" => Some("È".to_string()),         // Latin Capital Letter E with Grave
            "egrave" => Some("è".to_string()),         // Latin Small Letter E with Grave
            "Eacute" => Some("É".to_string()),         // Latin Capital Letter E with Acute
            "eacute" => Some("é".to_string()),         // Latin Small Letter E with Acute
            "Ecirc" => Some("Ê".to_string()),          // Latin Capital Letter E with Circumflex
            "ecirc" => Some("ê".to_string()),          // Latin Small Letter E with Circumflex
            "Euml" => Some("Ë".to_string()),           // Latin Capital Letter E with Diaeresis
            "euml" => Some("ë".to_string()),           // Latin Small Letter E with Diaeresis
            "Igrave" => Some("Ì".to_string()),         // Latin Capital Letter I with Grave
            "igrave" => Some("ì".to_string()),         // Latin Small Letter I with Grave
            "Iacute" => Some("Í".to_string()),         // Latin Capital Letter I with Acute
            "iacute" => Some("í".to_string()),         // Latin Small Letter I with Acute
            "Icirc" => Some("Î".to_string()),          // Latin Capital Letter I with Circumflex
            "icirc" => Some("î".to_string()),          // Latin Small Letter I with Circumflex
            "Iuml" => Some("Ï".to_string()),           // Latin Capital Letter I with Diaeresis
            "iuml" => Some("ï".to_string()),           // Latin Small Letter I with Diaeresis
            "Ograve" => Some("Ò".to_string()),         // Latin Capital Letter O with Grave
            "ograve" => Some("ò".to_string()),         // Latin Small Letter O with Grave
            "Oacute" => Some("Ó".to_string()),         // Latin Capital Letter O with Acute
            "oacute" => Some("ó".to_string()),         // Latin Small Letter O with Acute
            "Ocirc" => Some("Ô".to_string()),          // Latin Capital Letter O with Circumflex
            "ocirc" => Some("ô".to_string()),          // Latin Small Letter O with Circumflex
            "Otilde" => Some("Õ".to_string()),         // Latin Capital Letter O with Tilde
            "otilde" => Some("õ".to_string()),         // Latin Small Letter O with Tilde
            "Ugrave" => Some("Ù".to_string()),         // Latin Capital Letter U with Grave
            "ugrave" => Some("ù".to_string()),         // Latin Small Letter U with Grave
            "Uacute" => Some("Ú".to_string()),         // Latin Capital Letter U with Acute
            "uacute" => Some("ú".to_string()),         // Latin Small Letter U with Acute
            "Ucirc" => Some("Û".to_string()),          // Latin Capital Letter U with Circumflex
            "ucirc" => Some("û".to_string()),          // Latin Small Letter U with Circumflex
            "Yacute" => Some("Ý".to_string()),         // Latin Capital Letter Y with Acute
            "yacute" => Some("ý".to_string()),         // Latin Small Letter Y with Acute
            "yuml" => Some("ÿ".to_string()),           // Latin Small Letter Y with Diaeresis

            // Spanish characters
            "Ntilde" => Some("Ñ".to_string()),         // Latin Capital Letter N with Tilde
            "ntilde" => Some("ñ".to_string()),         // Latin Small Letter N with Tilde
            "iquest" => Some("¿".to_string()),         // Inverted Question Mark
            "iexcl" => Some("¡".to_string()),          // Inverted Exclamation Mark

            // Icelandic characters
            "THORN" => Some("Þ".to_string()),          // Latin Capital Letter Thorn
            "thorn" => Some("þ".to_string()),          // Latin Small Letter Thorn
            "ETH" => Some("Ð".to_string()),            // Latin Capital Letter Eth
            "eth" => Some("ð".to_string()),            // Latin Small Letter Eth

            // Numeric entities
            _ if entity.starts_with('#') => self.decode_numeric_entity(&entity[1..]),

            // Unknown entity
            _ => None,
        }
    }

    /// Decode numeric HTML entities (decimal and hexadecimal)
    fn decode_numeric_entity(&self, num_str: &str) -> Option<String> {
        if num_str.is_empty() {
            return None;
        }

        let code_point = if num_str.starts_with('x') || num_str.starts_with('X') {
            // Hexadecimal
            u32::from_str_radix(&num_str[1..], 16).ok()?
        } else {
            // Decimal
            num_str.parse::<u32>().ok()?
        };

        // Convert to character
        char::from_u32(code_point).map(|c| c.to_string())
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
                        for skip_ch in chars.by_ref() {
                            if skip_ch == '>' {
                                break;
                            }
                        }
                        continue;
                    } else if lookahead_lower.starts_with("/script") {
                        in_script_tag = false;
                        // Skip until we find the closing >
                        for skip_ch in chars.by_ref() {
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
                        tag if tag.starts_with("h1")
                            || tag.starts_with("h2")
                            || tag.starts_with("h3")
                            || tag.starts_with("h4")
                            || tag.starts_with("h5")
                            || tag.starts_with("h6") =>
                        {
                            result.push_str("\n\n");
                        }
                        tag if tag.starts_with("/h1")
                            || tag.starts_with("/h2")
                            || tag.starts_with("/h3")
                            || tag.starts_with("/h4")
                            || tag.starts_with("/h5")
                            || tag.starts_with("/h6") =>
                        {
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

    #[tokio::test]
    async fn test_html_entities_conversion() {
        let converter = create_test_converter();
        let html = r#"<p>Hello&nbsp;world&amp;more&lt;test&gt;"quotes"</p>"#;
        let result = converter.convert(html).await.unwrap();

        // Should not contain raw HTML entities
        assert!(!result.contains("&nbsp;"));
        assert!(!result.contains("&amp;"));
        assert!(!result.contains("&lt;"));
        assert!(!result.contains("&gt;"));
        assert!(!result.contains("&quot;"));

        // Should contain the proper characters
        assert!(result.contains("Hello world"));
        assert!(result.contains("&more"));
        assert!(result.contains("<test>"));
    }

    #[tokio::test]
    async fn test_apcoa_email_structure() {
        let converter = create_test_converter();
        let html = r#"
        <table>
            <tr>
                <td><h1>Ditt kvitto</h1></td>
            </tr>
            <tr>
                <td>Tack för att du betalade med APCOA FLOW, <span>Test User</span>.</td>
            </tr>
            <tr>
                <td>&nbsp;</td>
            </tr>
            <tr>
                <td>
                    <table>
                        <tr>
                            <td><b>Namn :</b> <span>Test User Name</span></td>
                        </tr>
                    </table>
                </td>
            </tr>
        </table>
        "#;
        let result = converter.convert(html).await.unwrap();

        println!("APCOA conversion result: {}", result);

        // Should contain the key text content
        assert!(result.contains("Ditt kvitto"));
        assert!(result.contains("Tack för att du betalade"));
        assert!(result.contains("Test User"));
        assert!(result.contains("Namn :"));
        assert!(result.contains("Test User Name"));

        // Should not contain raw HTML entities
        assert!(!result.contains("&nbsp;"));

        // Should not be empty or just whitespace
        assert!(!result.trim().is_empty());
    }

    #[tokio::test]
    async fn test_numeric_html_entities() {
        let converter = create_test_converter();
        let html = r#"<p>&#8203;&#32;&#160;&#39;Quote&#8217;</p>"#;
        let result = converter.convert(html).await.unwrap();

        println!("Numeric entities result: '{}'", result);

        // Should not contain raw numeric entities
        assert!(!result.contains("&#8203;"));
        assert!(!result.contains("&#32;"));
        assert!(!result.contains("&#160;"));
        assert!(!result.contains("&#39;"));
        assert!(!result.contains("&#8217;"));

        // Should contain proper characters (spaces and quotes)
        assert!(result.contains("Quote"));
    }

    #[tokio::test]
    async fn test_real_apcoa_email_content() {
        let converter = create_test_converter();

        // Simplified version of the real APCOA email HTML structure
        let apcoa_html = r#"
        <!DOCTYPE html>
        <html>
        <head>
            <title>Payment Success SE</title>
        </head>
        <body style="margin: 0; padding: 0;">
            <table role="presentation" border="0" cellpadding="0" cellspacing="0" width="100%">
                <tbody>
                    <tr>
                        <td style="padding: 30px 0">
                            <table width="600" align="center" border="0" cellpadding="0" cellspacing="0">
                                <tbody>
                                    <tr>
                                        <td align="center">
                                            <img width="200" src="https://example.com/logo.png" alt="Flow Logo" />
                                        </td>
                                    </tr>
                                    <tr>
                                        <td align="center" style="font-family: Arial, sans-serif; font-size: 12px; color: #06395b;">
                                            <h1>Ditt kvitto</h1>
                                        </td>
                                    </tr>
                                    <tr>
                                        <td style="font-family: Arial, sans-serif; font-size: 12px; color: #06395b;">
                                            Tack för att du betalade med APCOA FLOW,
                                            <span>Test User</span>.
                                        </td>
                                    </tr>
                                    <tr>
                                        <td><hr style="border-color: #97979735;" /></td>
                                    </tr>
                                    <tr>
                                        <td>
                                            <table border="0" cellpadding="0" cellspacing="0" width="100%">
                                                <tbody>
                                                    <tr>
                                                        <td>
                                                            <table border="0" cellpadding="0" cellspacing="0" width="100%">
                                                                <tbody>
                                                                    <tr>
                                                                        <td style="font-family: Arial, sans-serif; font-size: 12px; color: #06395b;">
                                                                            <b>Namn :</b>
                                                                            <span>Test User Name</span>
                                                                        </td>
                                                                    </tr>
                                                                </tbody>
                                                            </table>
                                                        </td>
                                                    </tr>
                                                </tbody>
                                            </table>
                                        </td>
                                    </tr>
                                    <tr>
                                        <td>
                                            <table border="0" cellpadding="0" cellspacing="0" width="100%">
                                                <tbody>
                                                    <tr>
                                                        <td style="font-family: Arial, sans-serif; font-size: 12px; color: #06395b;">
                                                            <b>Information om din parkering : </b>
                                                        </td>
                                                    </tr>
                                                    <tr>
                                                        <td style="font-family: Arial, sans-serif; font-size: 12px; color: #06395b;">
                                                            <span>Hospital Visitor Parking</span>
                                                            <span style="background-color: #f1f1f1; padding: 4px;">
                                                                Zonkod : <span>12345</span>
                                                            </span>
                                                        </td>
                                                    </tr>
                                                </tbody>
                                            </table>
                                        </td>
                                    </tr>
                                    <tr>
                                        <td style="border-top: 1px solid #97979735; margin: 8px 0;">
                                            &nbsp;
                                        </td>
                                    </tr>
                                    <tr>
                                        <td>
                                            <table border="0" cellpadding="0" cellspacing="0" width="100%">
                                                <tbody>
                                                    <tr>
                                                        <td style="font-family: Arial, sans-serif; font-size: 12px; color: #06395b; padding-bottom: 15px;">
                                                            <b>Din parkering :</b>
                                                            <span>#12345678</span>
                                                        </td>
                                                    </tr>
                                                </tbody>
                                            </table>
                                        </td>
                                    </tr>
                                </tbody>
                            </table>
                        </td>
                    </tr>
                </tbody>
            </table>
        </body>
        </html>
        "#;

        let result = converter.convert(apcoa_html).await.unwrap();

        println!("=== REAL APCOA EMAIL CONVERSION RESULT ===");
        println!("{}", result);
        println!("=== END RESULT (length: {}) ===", result.len());

        // Should contain the main Swedish content
        assert!(result.contains("Ditt kvitto"));
        assert!(result.contains("Tack för att du betalade"));
        assert!(result.contains("Test User"));
        assert!(result.contains("Namn :"));
        assert!(result.contains("Test User Name"));
        assert!(result.contains("Information om din parkering"));
        assert!(result.contains("Hospital Visitor Parking"));
        assert!(result.contains("Zonkod"));
        assert!(result.contains("12345"));
        assert!(result.contains("Din parkering"));
        assert!(result.contains("#12345678"));

        // Should not contain raw HTML entities
        assert!(!result.contains("&nbsp;"));

        // Should not be empty
        assert!(!result.trim().is_empty());
        assert!(result.len() > 100); // Should have substantial content
    }

    #[tokio::test]
    async fn test_medium_article_with_entities() {
        let converter = create_test_converter();

        // Simplified version of the Medium article with HTML entities
        let medium_html = r#"
        <html lang="en">
        <head>
            <title>Why we love puzzles</title>
        </head>
        <body>
            <div>
                <h2>Why we love puzzles</h2>
                <h4>Emotion in the age of AI + avoiding cognitive bias while booking summer travel (Issue #345)</h4>
                <div>
                    <p>Puzzles aren't just a way to pass time for <a href="https://writeplatform.example/u/b61faf3f1de8">Heidi Erwin</a>; they're her livelihood. As a senior game designer at <em>The New York Times</em>, <a href="http://redirect.medium.systems/r-xyYALjyI6d">she creates the weekly Brain Tickler</a>: a visual riddle or word game that invites lateral thinking.</p>
                    <p>Inspiration might come from something small or random. One puzzle began when Erwin started noticing bike racks around Queens. Each had a distinct geometric shape, which she transformed into a visual riddle: What item might be seen with all five? The answer — "a bicycle" — only lands if the clues are clear and the connections click.</p>
                    <p>Whether analog or digital, puzzles require patience and focus. They ask you to notice more, make new connections, and tolerate uncertainty longer than most tasks allow. The pleasure comes from staying with a problem until something clicks.</p>
                </div>
                <h3>🤿 Jumping-off quotes</h3>
                <ul>
                    <li>"The reduction of emotion to something inconvenient, unproductive, or excessive is not just a cultural shift; it's a design principle. We're living in a world being increasingly built by, for, and through machines that don't feel. And in the process, we're unlearning how to." — <a href="https://writeplatform.example/u/84eb4492fc9d">ashwini asokan</a></li>
                    <li>"There's a difference between letting feedback shape you and letting it name you. One is formation. The other is fusion. And when we fuse our identity with what others say — good or bad — we become unsteady, living at the mercy of whatever opinions blow through the room."</li>
                </ul>
                <p><em>Questions, feedback, or story suggestions? Email us: </em><a href="mailto:user@example.com">user@example.com</a></p>
            </div>
        </body>
        </html>
        "#;

        let result = converter.convert(medium_html).await.unwrap();

        println!("=== MEDIUM ARTICLE CONVERSION RESULT ===");
        println!("{}", result);
        println!("=== END RESULT (length: {}) ===", result.len());

        // Should contain the main content
        assert!(result.contains("Why we love puzzles"));
        assert!(result.contains("Emotion in the age of AI"));
        assert!(result.contains("Heidi Erwin"));
        assert!(result.contains("The New York Times"));
        assert!(result.contains("Brain") && result.contains("Tickler"));
        assert!(result.contains("bike racks around Queens"));
        assert!(result.contains("Jumping-off quotes"));
        assert!(result.contains("ashwini") && result.contains("asokan"));
        assert!(result.contains("user@example.com"));

        // Should handle quotes properly
        assert!(result.contains("\"a bicycle\""));

        // Should not be empty
        assert!(!result.trim().is_empty());
        assert!(result.len() > 200); // Should have substantial content
    }

    #[tokio::test]
    async fn test_comprehensive_html_entities() {
        let converter = create_test_converter();

        // Test HTML with a comprehensive set of entities
        let html_with_entities = r#"
        <html>
        <body>
            <p>Zero-width: &shy;&zwnj;&#8203;&#173;&#8204;&zwj;</p>
            <p>Spaces: word&thinsp;thin&ensp;en&emsp;em word</p>
            <p>Math: 5&times;3=15, 10&divide;2=5, &plusmn;1, 90&deg;</p>
            <p>Currency: &euro;10, &pound;5, &yen;100, &cent;50</p>
            <p>Fractions: &frac12;, &frac14;, &frac34;</p>
            <p>Quotes: &ldquo;hello&rdquo; and &lsquo;world&rsquo;</p>
            <p>Other: &copy;2025 &reg; &trade; &hellip;</p>
        </body>
        </html>
        "#;

        let result = converter.convert(html_with_entities).await.unwrap();

        println!("=== COMPREHENSIVE ENTITIES TEST RESULT ===");
        println!("{}", result);
        println!("=== END RESULT ===");

        // Verify that raw entities are not present
        assert!(!result.contains("&shy;"));
        assert!(!result.contains("&zwnj;"));
        assert!(!result.contains("&zwj;"));
        assert!(!result.contains("&thinsp;"));
        assert!(!result.contains("&ensp;"));
        assert!(!result.contains("&emsp;"));
        assert!(!result.contains("&times;"));
        assert!(!result.contains("&divide;"));
        assert!(!result.contains("&euro;"));
        assert!(!result.contains("&pound;"));
        assert!(!result.contains("&yen;"));
        assert!(!result.contains("&cent;"));
        assert!(!result.contains("&deg;"));
        assert!(!result.contains("&plusmn;"));
        assert!(!result.contains("&frac12;"));
        assert!(!result.contains("&frac14;"));
        assert!(!result.contains("&frac34;"));
        assert!(!result.contains("&ldquo;"));
        assert!(!result.contains("&rdquo;"));
        assert!(!result.contains("&lsquo;"));
        assert!(!result.contains("&rsquo;"));

        // Verify that symbols are properly converted
        assert!(result.contains("×"));
        assert!(result.contains("÷"));
        assert!(result.contains("±"));
        assert!(result.contains("°"));
        assert!(result.contains("€"));
        assert!(result.contains("£"));
        assert!(result.contains("¥"));
        assert!(result.contains("¢"));
        assert!(result.contains("½"));
        assert!(result.contains("¼"));
        assert!(result.contains("¾"));
        assert!(result.contains("\u{201C}")); // Left double quote
        assert!(result.contains("\u{201D}")); // Right double quote
        assert!(result.contains("\u{2018}")); // Left single quote
        assert!(result.contains("\u{2019}")); // Right single quote
        assert!(result.contains("©"));
        assert!(result.contains("®"));
        assert!(result.contains("™"));
        assert!(result.contains("…"));
    }

    #[tokio::test]
    async fn test_bilprovningen_zwnj_entities() {
        let converter = create_test_converter();

        // Test the specific pattern found in bilprovningen email
        let html_with_zwnj = r#"
        <html>
        <body>
            <div style="display:none;max-height:0px;overflow:hidden;">&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;</div>
            <p>This is the main content that should be visible.</p>
        </body>
        </html>
        "#;

        let result = converter.convert(html_with_zwnj).await.unwrap();

        println!("=== BILPROVNINGEN ZWNJ TEST RESULT ===");
        println!("'{}'", result);
        println!("=== END RESULT ===");

        // Should not contain raw ZWNJ entities
        assert!(!result.contains("&zwnj;"));
        assert!(!result.contains("&nbsp;"));
        
        // Should contain the main content
        assert!(result.contains("This is the main content"));
        assert!(result.contains("should be visible"));
    }

    #[tokio::test]
    async fn test_language_specific_entities() {
        let converter = create_test_converter();

        // Test HTML with language-specific entities including Swedish example
        let html_with_language_entities = r#"
        <html>
        <body>
            <p>Swedish: &Ouml;ppna i din webbl&auml;sare</p>
            <p>Nordic: &Aring;rhus, Malm&ouml;, K&oslash;benhavn, &AElig;beltoft</p>
            <p>German: M&uuml;nchen, Wei&szlig;bier, &Uuml;berraschung</p>
            <p>French: Caf&eacute;, na&iuml;ve, cr&egrave;me br&ucirc;l&eacute;e, &ccedil;a va</p>
            <p>Spanish: Ma&ntilde;ana, &iquest;C&oacute;mo est&aacute;s?</p>
            <p>Icelandic: &THORN;&oacute;r, Gu&eth;r&uacute;n</p>
        </body>
        </html>
        "#;

        let result = converter.convert(html_with_language_entities).await.unwrap();

        println!("=== LANGUAGE ENTITIES TEST RESULT ===");
        println!("{}", result);
        println!("=== END RESULT ===");

        // Should not contain raw entities
        assert!(!result.contains("&Ouml;"));
        assert!(!result.contains("&auml;"));
        assert!(!result.contains("&Aring;"));
        assert!(!result.contains("&oslash;"));
        assert!(!result.contains("&AElig;"));
        assert!(!result.contains("&uuml;"));
        assert!(!result.contains("&szlig;"));
        assert!(!result.contains("&eacute;"));
        assert!(!result.contains("&iuml;"));
        assert!(!result.contains("&egrave;"));
        assert!(!result.contains("&ucirc;"));
        assert!(!result.contains("&ccedil;"));
        assert!(!result.contains("&ntilde;"));
        assert!(!result.contains("&iquest;"));
        assert!(!result.contains("&oacute;"));
        assert!(!result.contains("&aacute;"));
        assert!(!result.contains("&THORN;"));
        assert!(!result.contains("&eth;"));
        assert!(!result.contains("&uacute;"));

        // Should contain properly converted characters
        assert!(result.contains("Öppna i din webbläsare"));  // Swedish example
        assert!(result.contains("Århus"));                   // Danish Å
        assert!(result.contains("Malmö"));                   // Swedish ö
        assert!(result.contains("København"));               // Danish ø
        assert!(result.contains("Æbeltoft"));                // Danish Æ
        assert!(result.contains("München"));                 // German ü
        assert!(result.contains("Weißbier"));                // German ß
        assert!(result.contains("Überraschung"));            // German Ü
        assert!(result.contains("Café"));                    // French é
        assert!(result.contains("naïve"));                   // French ï
        assert!(result.contains("crème brûlée"));            // French è, û, é
        assert!(result.contains("ça va"));                   // French ç
        assert!(result.contains("Mañana"));                  // Spanish ñ
        assert!(result.contains("¿Cómo estás?"));            // Spanish ¿, ó, á, ?
        assert!(result.contains("Þór"));                     // Icelandic Þ, ó
        assert!(result.contains("Guðrún"));                  // Icelandic ð, ú
    }
}
