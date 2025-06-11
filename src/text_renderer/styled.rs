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
                self.current_style = self
                    .current_style
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
                self.current_style = self
                    .current_style
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

            // List and bullet symbols
            "bull" => Some("•".to_string()),        // Bullet
            "squf" => Some("▪".to_string()),        // Black small square
            "blacksquare" => Some("■".to_string()), // Black large square
            "whitesquare" => Some("□".to_string()), // White large square
            "diamond" => Some("◊".to_string()),     // Diamond

            // Arrow symbols
            "larr" => Some("←".to_string()),  // Left arrow
            "uarr" => Some("↑".to_string()),  // Up arrow
            "rarr" => Some("→".to_string()),  // Right arrow
            "darr" => Some("↓".to_string()),  // Down arrow
            "harr" => Some("↔".to_string()),  // Left right arrow
            "crarr" => Some("↵".to_string()), // Down left arrow (carriage return)

            // Single angle quotes
            "lsaquo" => Some("‹".to_string()), // Single left angle quote
            "rsaquo" => Some("›".to_string()), // Single right angle quote

            // Card suits
            "spades" => Some("♠".to_string()), // Spades
            "clubs" => Some("♣".to_string()),  // Clubs
            "hearts" => Some("♥".to_string()), // Hearts
            "diams" => Some("♦".to_string()),  // Diamonds

            // Additional common entities (synced from builtin)
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
            "shy" => Some("\u{00AD}".to_string()), // Soft hyphen
            "zwnj" => Some("\u{200C}".to_string()), // Zero width non-joiner
            "zwj" => Some("\u{200D}".to_string()), // Zero width joiner

            // Spaces
            "thinsp" => Some("\u{2009}".to_string()), // Thin space
            "ensp" => Some("\u{2002}".to_string()),   // En space
            "emsp" => Some("\u{2003}".to_string()),   // Em space

            // Mathematical symbols
            "times" => Some("×".to_string()), // Multiplication sign
            "divide" => Some("÷".to_string()), // Division sign
            "plusmn" => Some("±".to_string()), // Plus minus
            "deg" => Some("°".to_string()),   // Degree symbol

            // Currency symbols
            "euro" => Some("€".to_string()),  // Euro symbol
            "pound" => Some("£".to_string()), // Pound symbol
            "yen" => Some("¥".to_string()),   // Yen symbol
            "cent" => Some("¢".to_string()),  // Cent symbol

            // Fractions
            "frac12" => Some("½".to_string()), // One half
            "frac14" => Some("¼".to_string()), // One quarter
            "frac34" => Some("¾".to_string()), // Three quarters

            // Additional common entities
            "sup1" => Some("¹".to_string()),   // Superscript 1
            "sup2" => Some("²".to_string()),   // Superscript 2
            "sup3" => Some("³".to_string()),   // Superscript 3
            "ordm" => Some("º".to_string()),   // Masculine ordinal
            "ordf" => Some("ª".to_string()),   // Feminine ordinal
            "sect" => Some("§".to_string()),   // Section sign
            "para" => Some("¶".to_string()),   // Pilcrow sign
            "micro" => Some("µ".to_string()),  // Micro sign
            "middot" => Some("·".to_string()), // Middle dot
            "cedil" => Some("¸".to_string()),  // Cedilla
            "acute" => Some("´".to_string()),  // Acute accent
            "uml" => Some("¨".to_string()),    // Diaeresis
            "macr" => Some("¯".to_string()),   // Macron
            "not" => Some("¬".to_string()),    // Not sign
            "brvbar" => Some("¦".to_string()), // Broken bar
            "curren" => Some("¤".to_string()), // Currency sign

            // Nordic/Scandinavian characters (Swedish, Norwegian, Danish, Finnish)
            "Aring" => Some("Å".to_string()), // Latin Capital Letter A with Ring Above
            "aring" => Some("å".to_string()), // Latin Small Letter A with Ring Above
            "Auml" => Some("Ä".to_string()),  // Latin Capital Letter A with Diaeresis
            "auml" => Some("ä".to_string()),  // Latin Small Letter A with Diaeresis
            "Ouml" => Some("Ö".to_string()),  // Latin Capital Letter O with Diaeresis
            "ouml" => Some("ö".to_string()),  // Latin Small Letter O with Diaeresis
            "AElig" => Some("Æ".to_string()), // Latin Capital Letter Æ
            "aelig" => Some("æ".to_string()), // Latin Small Letter æ
            "Oslash" => Some("Ø".to_string()), // Latin Capital Letter O with Stroke
            "oslash" => Some("ø".to_string()), // Latin Small Letter O with Stroke

            // German characters
            "Uuml" => Some("Ü".to_string()), // Latin Capital Letter U with Diaeresis
            "uuml" => Some("ü".to_string()), // Latin Small Letter U with Diaeresis
            "szlig" => Some("ß".to_string()), // Latin Small Letter Sharp S

            // French characters
            "Agrave" => Some("À".to_string()), // Latin Capital Letter A with Grave
            "agrave" => Some("à".to_string()), // Latin Small Letter A with Grave
            "Aacute" => Some("Á".to_string()), // Latin Capital Letter A with Acute
            "aacute" => Some("á".to_string()), // Latin Small Letter A with Acute
            "Acirc" => Some("Â".to_string()),  // Latin Capital Letter A with Circumflex
            "acirc" => Some("â".to_string()),  // Latin Small Letter A with Circumflex
            "Atilde" => Some("Ã".to_string()), // Latin Capital Letter A with Tilde
            "atilde" => Some("ã".to_string()), // Latin Small Letter A with Tilde
            "Ccedil" => Some("Ç".to_string()), // Latin Capital Letter C with Cedilla
            "ccedil" => Some("ç".to_string()), // Latin Small Letter C with Cedilla
            "Egrave" => Some("È".to_string()), // Latin Capital Letter E with Grave
            "egrave" => Some("è".to_string()), // Latin Small Letter E with Grave
            "Eacute" => Some("É".to_string()), // Latin Capital Letter E with Acute
            "eacute" => Some("é".to_string()), // Latin Small Letter E with Acute
            "Ecirc" => Some("Ê".to_string()),  // Latin Capital Letter E with Circumflex
            "ecirc" => Some("ê".to_string()),  // Latin Small Letter E with Circumflex
            "Euml" => Some("Ë".to_string()),   // Latin Capital Letter E with Diaeresis
            "euml" => Some("ë".to_string()),   // Latin Small Letter E with Diaeresis
            "Igrave" => Some("Ì".to_string()), // Latin Capital Letter I with Grave
            "igrave" => Some("ì".to_string()), // Latin Small Letter I with Grave
            "Iacute" => Some("Í".to_string()), // Latin Capital Letter I with Acute
            "iacute" => Some("í".to_string()), // Latin Small Letter I with Acute
            "Icirc" => Some("Î".to_string()),  // Latin Capital Letter I with Circumflex
            "icirc" => Some("î".to_string()),  // Latin Small Letter I with Circumflex
            "Iuml" => Some("Ï".to_string()),   // Latin Capital Letter I with Diaeresis
            "iuml" => Some("ï".to_string()),   // Latin Small Letter I with Diaeresis
            "Ograve" => Some("Ò".to_string()), // Latin Capital Letter O with Grave
            "ograve" => Some("ò".to_string()), // Latin Small Letter O with Grave
            "Oacute" => Some("Ó".to_string()), // Latin Capital Letter O with Acute
            "oacute" => Some("ó".to_string()), // Latin Small Letter O with Acute
            "Ocirc" => Some("Ô".to_string()),  // Latin Capital Letter O with Circumflex
            "ocirc" => Some("ô".to_string()),  // Latin Small Letter O with Circumflex
            "Otilde" => Some("Õ".to_string()), // Latin Capital Letter O with Tilde
            "otilde" => Some("õ".to_string()), // Latin Small Letter O with Tilde
            "Ugrave" => Some("Ù".to_string()), // Latin Capital Letter U with Grave
            "ugrave" => Some("ù".to_string()), // Latin Small Letter U with Grave
            "Uacute" => Some("Ú".to_string()), // Latin Capital Letter U with Acute
            "uacute" => Some("ú".to_string()), // Latin Small Letter U with Acute
            "Ucirc" => Some("Û".to_string()),  // Latin Capital Letter U with Circumflex
            "ucirc" => Some("û".to_string()),  // Latin Small Letter U with Circumflex
            "Yacute" => Some("Ý".to_string()), // Latin Capital Letter Y with Acute
            "yacute" => Some("ý".to_string()), // Latin Small Letter Y with Acute
            "yuml" => Some("ÿ".to_string()),   // Latin Small Letter Y with Diaeresis

            // Spanish characters
            "Ntilde" => Some("Ñ".to_string()), // Latin Capital Letter N with Tilde
            "ntilde" => Some("ñ".to_string()), // Latin Small Letter N with Tilde
            "iquest" => Some("¿".to_string()), // Inverted Question Mark
            "iexcl" => Some("¡".to_string()),  // Inverted Exclamation Mark

            // Icelandic characters
            "THORN" => Some("Þ".to_string()), // Latin Capital Letter Thorn
            "thorn" => Some("þ".to_string()), // Latin Small Letter Thorn
            "ETH" => Some("Ð".to_string()),   // Latin Capital Letter Eth
            "eth" => Some("ð".to_string()),   // Latin Small Letter Eth

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
        assert!(result.lines.iter().any(|line| {
            line.spans
                .iter()
                .any(|span| span.content.contains("First paragraph"))
        }));
        assert!(result.lines.iter().any(|line| {
            line.spans
                .iter()
                .any(|span| span.content.contains("Second paragraph"))
        }));
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

    #[test]
    fn test_bullet_and_list_entities() {
        let converter = StyledTextConverter::new(default_config());
        let html = r#"<p>&bull; Bullet point &squf; Square &diamond; Diamond</p>"#;
        let result = converter.convert_to_styled_text(html).unwrap();

        let text_content = result
            .lines
            .iter()
            .flat_map(|line| line.spans.iter())
            .map(|span| span.content.as_ref())
            .collect::<String>();

        // Should not contain raw HTML entities
        assert!(!text_content.contains("&bull;"));
        assert!(!text_content.contains("&squf;"));
        assert!(!text_content.contains("&diamond;"));

        // Should contain the proper Unicode characters
        assert!(text_content.contains("• Bullet point"));
        assert!(text_content.contains("▪ Square"));
        assert!(text_content.contains("◊ Diamond"));
    }

    #[test]
    fn test_arrow_entities() {
        let converter = StyledTextConverter::new(default_config());
        let html =
            r#"<p>&larr; Left &uarr; Up &rarr; Right &darr; Down &harr; Both &crarr; Return</p>"#;
        let result = converter.convert_to_styled_text(html).unwrap();

        let text_content = result
            .lines
            .iter()
            .flat_map(|line| line.spans.iter())
            .map(|span| span.content.as_ref())
            .collect::<String>();

        // Should not contain raw HTML entities
        assert!(!text_content.contains("&larr;"));
        assert!(!text_content.contains("&uarr;"));
        assert!(!text_content.contains("&rarr;"));
        assert!(!text_content.contains("&darr;"));
        assert!(!text_content.contains("&harr;"));
        assert!(!text_content.contains("&crarr;"));

        // Should contain the proper Unicode characters
        assert!(text_content.contains("← Left"));
        assert!(text_content.contains("↑ Up"));
        assert!(text_content.contains("→ Right"));
        assert!(text_content.contains("↓ Down"));
        assert!(text_content.contains("↔ Both"));
        assert!(text_content.contains("↵ Return"));
    }

    #[test]
    fn test_angle_quote_entities() {
        let converter = StyledTextConverter::new(default_config());
        let html = r#"<p>&lsaquo;single quotes&rsaquo; and &laquo;double quotes&raquo;</p>"#;
        let result = converter.convert_to_styled_text(html).unwrap();

        let text_content = result
            .lines
            .iter()
            .flat_map(|line| line.spans.iter())
            .map(|span| span.content.as_ref())
            .collect::<String>();

        // Should not contain raw HTML entities
        assert!(!text_content.contains("&lsaquo;"));
        assert!(!text_content.contains("&rsaquo;"));

        // Should contain the proper Unicode characters
        assert!(text_content.contains("‹single quotes›"));
        assert!(text_content.contains("«double quotes»"));
    }

    #[test]
    fn test_card_suit_entities() {
        let converter = StyledTextConverter::new(default_config());
        let html = r#"<p>&spades; &clubs; &hearts; &diams;</p>"#;
        let result = converter.convert_to_styled_text(html).unwrap();

        let text_content = result
            .lines
            .iter()
            .flat_map(|line| line.spans.iter())
            .map(|span| span.content.as_ref())
            .collect::<String>();

        // Should not contain raw HTML entities
        assert!(!text_content.contains("&spades;"));
        assert!(!text_content.contains("&clubs;"));
        assert!(!text_content.contains("&hearts;"));
        assert!(!text_content.contains("&diams;"));

        // Should contain the proper Unicode characters
        assert!(text_content.contains("♠"));
        assert!(text_content.contains("♣"));
        assert!(text_content.contains("♥"));
        assert!(text_content.contains("♦"));
    }
}
