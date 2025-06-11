use crate::body::{BodyContent, BodyPart};
use crate::thread::Message;
use ammonia::Builder;
use maplit::hashset;
use regex::Regex;

#[derive(Debug, Clone)]
pub struct RenderedContent {
    pub html: Option<String>,
    pub plain: Option<String>,
    pub content_type: ContentType,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ContentType {
    Html,
    PlainText,
    Mixed,
}

impl RenderedContent {
    pub fn has_html(&self) -> bool {
        self.html.is_some()
    }

    pub fn has_plain(&self) -> bool {
        self.plain.is_some()
    }

    pub fn get_primary_content(&self) -> Option<&str> {
        match self.content_type {
            ContentType::Html => self.html.as_deref().or(self.plain.as_deref()),
            ContentType::PlainText => self.plain.as_deref(),
            ContentType::Mixed => self.html.as_deref().or(self.plain.as_deref()),
        }
    }
}

pub fn render_message_content(message: &Message) -> RenderedContent {
    render_message_content_with_options(message, false, false)
}

pub fn render_message_content_with_url_rewriting(message: &Message) -> RenderedContent {
    render_message_content_with_options(message, true, false)
}

pub fn render_message_content_with_image_control(
    message: &Message,
    show_images: bool,
) -> RenderedContent {
    render_message_content_with_options(message, true, !show_images)
}

fn render_message_content_with_options(
    message: &Message,
    rewrite_urls: bool,
    block_images: bool,
) -> RenderedContent {
    let mut html_content = None;
    let mut plain_content = None;

    tracing::debug!(
        "Rendering message {} with {} body parts",
        message.id,
        message.body.len()
    );

    // Extract content from all body parts
    for body_part in &message.body {
        tracing::debug!(
            "Processing body part id={}, type={}",
            body_part.id,
            body_part.content_type
        );
        extract_content_from_body_part(body_part, &mut html_content, &mut plain_content);
    }

    tracing::debug!(
        "After extraction - has_html: {}, has_plain: {}",
        html_content.is_some(),
        plain_content.is_some()
    );

    // Sanitize HTML content if present
    if let Some(html) = &html_content {
        html_content = Some(sanitize_html_with_image_control(
            html,
            rewrite_urls,
            block_images,
        ));
    }

    // If we have plain text but no HTML, convert plain to HTML for better display
    if html_content.is_none() && plain_content.is_some() {
        if let Some(plain) = &plain_content {
            html_content = Some(plain_text_to_html(plain));
        }
    }

    // Determine content type
    let content_type = match (&html_content, &plain_content) {
        (Some(_), Some(_)) => ContentType::Mixed,
        (Some(_), None) => ContentType::Html,
        (None, Some(_)) => ContentType::PlainText,
        (None, None) => ContentType::PlainText,
    };

    RenderedContent {
        html: html_content,
        plain: plain_content,
        content_type,
    }
}

fn extract_content_from_body_part(
    body_part: &BodyPart,
    html: &mut Option<String>,
    plain: &mut Option<String>,
) {
    tracing::debug!(
        "Extracting from part: id={}, type={}, is_attachment={}",
        body_part.id,
        body_part.content_type,
        body_part.is_attachment()
    );

    // Skip attachments
    if body_part.is_attachment() {
        tracing::debug!("Skipping attachment part {}", body_part.id);
        return;
    }

    match body_part.content_type.as_str() {
        "text/html" => {
            tracing::debug!("Found HTML content in part {}", body_part.id);
            if html.is_none() {
                if let BodyContent::Text(content) = &body_part.content {
                    tracing::debug!("Setting HTML content (length: {})", content.len());
                    *html = Some(content.clone());
                }
            }
        }
        "text/plain" => {
            tracing::debug!("Found plain text content in part {}", body_part.id);
            if plain.is_none() {
                if let BodyContent::Text(content) = &body_part.content {
                    tracing::debug!("Setting plain text content (length: {})", content.len());
                    *plain = Some(content.clone());
                }
            }
        }
        ct if ct.starts_with("multipart/") => {
            tracing::debug!("Processing multipart {} in part {}", ct, body_part.id);
            // Recursively extract from nested parts
            if let BodyContent::Multipart(parts) = &body_part.content {
                tracing::debug!("Found {} nested parts", parts.len());
                for part in parts {
                    extract_content_from_body_part(part, html, plain);
                }
            }
        }
        _ => {
            tracing::debug!(
                "Skipping unsupported content type: {}",
                body_part.content_type
            );
        }
    }
}

fn sanitize_html_with_image_control(html: &str, rewrite_urls: bool, block_images: bool) -> String {
    // Configure ammonia for safe HTML rendering
    let mut builder = Builder::default();

    // Allow comprehensive email formatting tags including responsive design support
    builder
        .tags(hashset![
            "a",
            "abbr",
            "b",
            "blockquote",
            "br",
            "code",
            "dd",
            "del",
            "div",
            "dl",
            "dt",
            "em",
            "h1",
            "h2",
            "h3",
            "h4",
            "h5",
            "h6",
            "hr",
            "i",
            "img",
            "ins",
            "li",
            "ol",
            "p",
            "pre",
            "q",
            "s",
            "small",
            "span",
            "strong",
            "sub",
            "sup",
            "table",
            "tbody",
            "td",
            "tfoot",
            "th",
            "thead",
            "tr",
            "u",
            "ul",
            "center"
        ])
        // Allow generic attributes including style, class, and essential email layout attributes
        .generic_attributes(hashset![
            "style",
            "class",
            "align",
            "valign",
            "bgcolor",
            "width",
            "height",
            "cellpadding",
            "cellspacing",
            "border",
            "target"
        ])
        .link_rel(Some("noopener noreferrer"))
        .url_schemes(hashset!["http", "https", "mailto"])
        .attribute_filter(move |element, attribute, value| {
            match (element, attribute) {
                // Rewrite link URLs through redirect endpoint
                ("a", "href") => {
                    if rewrite_urls
                        && (value.starts_with("http://") || value.starts_with("https://"))
                    {
                        Some(format!("/redirect?url={}", urlencoding::encode(value)).into())
                    } else {
                        Some(value.into())
                    }
                }
                // Ensure all external links open in new window to escape iframe
                ("a", "target") => {
                    if rewrite_urls {
                        Some("_blank".into())
                    } else {
                        Some(value.into())
                    }
                }
                // Add rel attributes for security
                ("a", "rel") => {
                    if rewrite_urls {
                        Some("noopener noreferrer".into())
                    } else {
                        Some(value.into())
                    }
                }
                // Rewrite image URLs through proxy endpoint
                ("img", "src") => {
                    if rewrite_urls
                        && (value.starts_with("http://") || value.starts_with("https://"))
                    {
                        if block_images {
                            Some(
                                format!(
                                    "/image_proxy?url={}&blocked=true",
                                    urlencoding::encode(value)
                                )
                                .into(),
                            )
                        } else {
                            Some(format!("/image_proxy?url={}", urlencoding::encode(value)).into())
                        }
                    } else {
                        Some(value.into())
                    }
                }
                ("img", "alt") | ("img", "width") | ("img", "height") => Some(value.into()),
                // Allow CSS classes for email styling
                (_, "class") => Some(value.into()),
                // Allow essential table layout attributes for email rendering
                ("table" | "td" | "th" | "tr", "align") => Some(value.into()),
                ("table" | "td" | "th" | "tr", "valign") => Some(value.into()),
                ("table" | "td" | "th", "bgcolor") => Some(value.into()),
                ("table" | "td" | "th", "width") => Some(value.into()),
                ("table" | "td" | "th", "height") => Some(value.into()),
                ("table", "cellpadding") => Some(value.into()),
                ("table", "cellspacing") => Some(value.into()),
                ("table", "border") => Some(value.into()),
                // Allow safe style attributes while blocking dangerous ones
                (_, "style") => {
                    let value_lower = value.to_lowercase();

                    // Block dangerous properties that could break layout or enable attacks
                    // More permissive for email fidelity while maintaining security
                    let dangerous_patterns = [
                        "position: fixed",
                        "position: absolute",
                        "position: sticky",
                        "z-index:",
                        "javascript:",
                        "expression(",
                        "url(javascript",
                        "url(data:text/html",
                        "@import",
                        "behavior:",
                        "binding:",
                        "-moz-binding",
                    ];

                    // Allow extensive styling properties for enhanced email fidelity and responsive design
                    let safe_properties = [
                        // Core styling
                        "color:",
                        "background-color:",
                        "background:",
                        "background-image:",
                        "background-size:",
                        "background-position:",
                        "font-weight:",
                        "font-style:",
                        "font-size:",
                        "font-family:",
                        "text-decoration:",
                        "text-align:",
                        // Layout and spacing
                        "padding:",
                        "margin:",
                        "border:",
                        "border-color:",
                        "border-width:",
                        "border-style:",
                        "border-radius:",
                        "width:",
                        "height:",
                        "max-width:",
                        "max-height:",
                        "min-width:",
                        "min-height:",
                        "margin-left:",
                        "margin-right:",
                        "margin-top:",
                        "margin-bottom:",
                        "padding-left:",
                        "padding-right:",
                        "padding-top:",
                        "padding-bottom:",
                        // Flexbox and layout
                        "display:",
                        "flex:",
                        "flex-direction:",
                        "align-items:",
                        "justify-content:",
                        "flex-wrap:",
                        "align-content:",
                        "gap:",
                        "row-gap:",
                        "column-gap:",
                        // Table layout
                        "table-layout:",
                        "border-collapse:",
                        "border-spacing:",
                        "vertical-align:",
                        // Typography
                        "line-height:",
                        "letter-spacing:",
                        "word-spacing:",
                        "white-space:",
                        "text-shadow:",
                        // Visual effects (safe subset)
                        "opacity:",
                        "box-shadow:",
                        "transform:",
                        "transition:",
                        // Responsive design and images
                        "object-fit:",
                        "object-position:",
                        "max-width: 100%",
                        "height: auto",
                        // CSS Grid (safe subset)
                        "grid-template-columns:",
                        "grid-template-rows:",
                        "grid-gap:",
                        "grid-column:",
                        "grid-row:",
                        // Safe positioning for email layouts
                        "position: relative",
                        "top:",
                        "right:",
                        "bottom:",
                        "left:",
                        // Overflow control for email content
                        "overflow:",
                        "overflow-x:",
                        "overflow-y:",
                        "text-overflow:",
                        // Email-specific properties
                        "mso-",
                        "-webkit-",
                        "-moz-",
                        "-ms-",
                        // Media queries support
                        "@media",
                        "screen",
                        "print",
                        "min-width:",
                        "max-width:",
                        "min-height:",
                        "max-height:",
                    ];

                    // Reject if contains dangerous patterns
                    if dangerous_patterns
                        .iter()
                        .any(|pattern| value_lower.contains(pattern))
                    {
                        None
                    }
                    // Accept if contains safe properties
                    else if safe_properties
                        .iter()
                        .any(|prop| value_lower.contains(prop))
                    {
                        Some(value.into())
                    }
                    // Default to rejecting unknown properties
                    else {
                        None
                    }
                }
                _ => None,
            }
        });

    let cleaned_html = builder.clean(html).to_string();

    // Post-process to ensure all rewritten links have target="_blank" if they don't already
    if rewrite_urls {
        post_process_links_for_new_window(&cleaned_html)
    } else {
        cleaned_html
    }
}

fn post_process_links_for_new_window(html: &str) -> String {
    // Use regex to find links that go through /redirect and ensure they have target="_blank"
    let re = Regex::new(r#"<a([^>]*href="/redirect\?[^"]*"[^>]*)>"#).unwrap();

    re.replace_all(html, |caps: &regex::Captures| {
        let link_attrs = &caps[1];

        // Check if target="_blank" is already present
        if link_attrs.contains("target=") {
            // Replace existing target with target="_blank"
            let target_re = Regex::new(r#"\s*target="[^"]*""#).unwrap();
            let attrs_without_target = target_re.replace_all(link_attrs, "");
            format!(
                "<a{} target=\"_blank\" rel=\"noopener noreferrer\">",
                attrs_without_target
            )
        } else {
            // Add target="_blank" and rel="noopener noreferrer"
            format!(
                "<a{} target=\"_blank\" rel=\"noopener noreferrer\">",
                link_attrs
            )
        }
    })
    .to_string()
}

pub fn plain_text_to_html(text: &str) -> String {
    // Convert plain text to simple HTML
    html_escape::encode_text(text)
        .lines()
        .map(|line| {
            if line.is_empty() {
                "<br>".to_string()
            } else {
                format!("{}<br>", line)
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::body::BodyPart;
    use crate::common::Headers;
    use crate::thread::Message;
    use std::collections::HashMap;

    #[test]
    fn test_sanitize_html() {
        let dangerous_html = r#"
            <p>Hello <script>alert('xss')</script></p>
            <a href="javascript:alert('xss')">Click me</a>
            <img src="http://example.com/image.jpg" onerror="alert('xss')">
        "#;

        let safe_html = sanitize_html_with_image_control(dangerous_html, false, false);

        assert!(!safe_html.contains("<script>"));
        assert!(!safe_html.contains("javascript:"));
        assert!(!safe_html.contains("onerror="));
        assert!(safe_html.contains("<p>Hello"));
        assert!(safe_html.contains("<img"));
    }

    #[test]
    fn test_sanitize_html_with_styles() {
        let html_with_styles = r#"
            <div style="max-width: 600px; margin: 0 auto; font-family: Arial;">
                <h1 style="color: navy; text-align: center;">Test Header</h1>
                <p style="color: red; font-weight: bold;">Important text</p>
                <div style="background: yellow; padding: 10px; border: 1px solid orange;">
                    Warning box
                </div>
            </div>
        "#;

        let safe_html = sanitize_html_with_image_control(html_with_styles, false, false);

        // Check that legitimate styles are preserved
        assert!(safe_html.contains("color: red") || safe_html.contains("color:red"));
        assert!(
            safe_html.contains("background: yellow") || safe_html.contains("background:yellow")
        );
        assert!(safe_html.contains("border:") || safe_html.contains("border "));
    }

    #[test]
    fn test_sanitize_malformed_html() {
        let malformed_html = r#"<html><body>
<p>This email has malformed HTML:</p>
<div style="font-size: 14px; color: #333;
<p>Unclosed div and unclosed style attribute</p>
<table><tr><td>Unclosed table
<strong>Unclosed strong
<em>Multiple unclosed <span>elements
<img src="nonexistent.jpg" alt="Broken image">
<script>alert('This should be removed');</script>
</body>
"#;

        let safe_html = sanitize_html_with_image_control(malformed_html, false, false);

        // Note: ammonia doesn't fix malformed HTML, it sanitizes what it can parse
        // The malformed style attribute may still be present if it's partially valid
        // Should not contain script tags
        assert!(!safe_html.contains("<script>"));
        // Should contain some basic content
        assert!(safe_html.contains("This email has malformed HTML"));
    }

    #[test]
    fn test_plain_text_to_html() {
        let text = "Hello\nWorld\n\nThis is a test";
        let html = plain_text_to_html(text);

        println!("HTML output: {}", html);

        assert!(html.contains("Hello<br>"));
        assert!(html.contains("World<br>"));
        assert!(html.contains("<br>")); // Empty lines will be single <br>
        assert!(html.contains("This is a test<br>"));
    }

    #[test]
    fn test_render_message_with_plain_text() {
        let message = Message {
            id: "test123".to_string(),
            is_match: true,
            excluded: false,
            filename: vec![],
            timestamp: 0,
            date_relative: "now".to_string(),
            tags: vec![],
            duplicate: None,
            body: vec![BodyPart {
                id: 1,
                content_type: "text/plain".to_string(),
                content: BodyContent::Text("Hello, this is plain text".to_string()),
                content_disposition: None,
                content_id: None,
                filename: None,
                content_transfer_encoding: None,
                content_length: None,
            }],
            crypto: crate::common::CryptoInfo::default(),
            headers: Headers {
                subject: "Test".to_string(),
                from: "test@example.com".to_string(),
                to: "user@example.com".to_string(),
                date: "2024-01-01".to_string(),
                reply_to: None,
                additional: HashMap::new(),
            },
        };

        let rendered = render_message_content(&message);
        assert!(rendered.has_plain());
        assert!(rendered.has_html()); // Plain text converted to HTML
        assert_eq!(
            rendered.plain.as_ref().unwrap(),
            "Hello, this is plain text"
        );
        assert!(
            rendered
                .html
                .as_ref()
                .unwrap()
                .contains("Hello, this is plain text<br>")
        );
    }

    #[test]
    fn test_render_message_with_html() {
        let message = Message {
            id: "test123".to_string(),
            is_match: true,
            excluded: false,
            filename: vec![],
            timestamp: 0,
            date_relative: "now".to_string(),
            tags: vec![],
            duplicate: None,
            body: vec![BodyPart {
                id: 1,
                content_type: "text/html".to_string(),
                content: BodyContent::Text("<p>Hello <b>world</b></p>".to_string()),
                content_disposition: None,
                content_id: None,
                filename: None,
                content_transfer_encoding: None,
                content_length: None,
            }],
            crypto: crate::common::CryptoInfo::default(),
            headers: Headers {
                subject: "Test".to_string(),
                from: "test@example.com".to_string(),
                to: "user@example.com".to_string(),
                date: "2024-01-01".to_string(),
                reply_to: None,
                additional: HashMap::new(),
            },
        };

        let rendered = render_message_content(&message);
        assert!(rendered.has_html());
        assert!(!rendered.has_plain());
        assert!(
            rendered
                .html
                .as_ref()
                .unwrap()
                .contains("<p>Hello <b>world</b></p>")
        );
    }

    #[test]
    fn test_render_multipart_alternative() {
        let message = Message {
            id: "test123".to_string(),
            is_match: true,
            excluded: false,
            filename: vec![],
            timestamp: 0,
            date_relative: "now".to_string(),
            tags: vec![],
            duplicate: None,
            body: vec![BodyPart {
                id: 1,
                content_type: "multipart/alternative".to_string(),
                content: BodyContent::Multipart(vec![
                    BodyPart {
                        id: 2,
                        content_type: "text/plain".to_string(),
                        content: BodyContent::Text("Plain text version".to_string()),
                        content_disposition: None,
                        content_id: None,
                        filename: None,
                        content_transfer_encoding: None,
                        content_length: None,
                    },
                    BodyPart {
                        id: 3,
                        content_type: "text/html".to_string(),
                        content: BodyContent::Text("<p>HTML version</p>".to_string()),
                        content_disposition: None,
                        content_id: None,
                        filename: None,
                        content_transfer_encoding: None,
                        content_length: None,
                    },
                ]),
                content_disposition: None,
                content_id: None,
                filename: None,
                content_transfer_encoding: None,
                content_length: None,
            }],
            crypto: crate::common::CryptoInfo::default(),
            headers: Headers {
                subject: "Test".to_string(),
                from: "test@example.com".to_string(),
                to: "user@example.com".to_string(),
                date: "2024-01-01".to_string(),
                reply_to: None,
                additional: HashMap::new(),
            },
        };

        let rendered = render_message_content(&message);
        assert!(rendered.has_html());
        assert!(rendered.has_plain());
        assert_eq!(rendered.plain.as_ref().unwrap(), "Plain text version");
        assert!(
            rendered
                .html
                .as_ref()
                .unwrap()
                .contains("<p>HTML version</p>")
        );
    }
}
