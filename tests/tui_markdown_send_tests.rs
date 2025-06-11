//! Tests for TUI markdown email sending functionality

use whynot::tui::markdown::markdown_to_html;

#[test]
fn test_markdown_conversion_for_email_body() {
    let markdown_body = r#"# Hello

This is a **bold** message with:

- Point 1
- Point 2

[Link](https://example.com)

```rust
fn hello() {
    println!("world");
}
```

Best regards!"#;

    let html_body = markdown_to_html(markdown_body);
    
    // Verify HTML contains expected elements
    assert!(html_body.contains("<h1>Hello</h1>"));
    assert!(html_body.contains("<strong>bold</strong>"));
    assert!(html_body.contains("<ul>"));
    assert!(html_body.contains("<li>Point 1</li>"));
    assert!(html_body.contains("<li>Point 2</li>"));
    assert!(html_body.contains("<a href=\"https://example.com\">Link</a>"));
    assert!(html_body.contains("<pre><code"));
    assert!(html_body.contains("fn hello()"));
    assert!(html_body.contains("Best regards"));
}

#[test]
fn test_plain_text_remains_unchanged() {
    let plain_text = "Just plain text\nwith line breaks\nand nothing special.";
    let html_body = markdown_to_html(plain_text);
    
    // Should be wrapped in paragraphs but otherwise unchanged
    assert!(html_body.contains("<p>Just plain text"));
    assert!(html_body.contains("with line breaks"));
    assert!(html_body.contains("and nothing special.</p>"));
}

#[test]
fn test_email_signature_markdown() {
    let markdown_with_signature = r#"Hi John,

Thanks for your **quick response**. The proposal looks good.

Best regards,
*Jane Smith*  
Senior Developer"#;

    let html_body = markdown_to_html(markdown_with_signature);
    
    // Check formatting is preserved
    assert!(html_body.contains("<strong>quick response</strong>"));
    assert!(html_body.contains("<em>Jane Smith</em>"));
    assert!(html_body.contains("Senior Developer"));
}

#[test]
fn test_markdown_email_with_code_and_links() {
    let technical_email = r#"## Status Update

The `main` function has been updated:

```rust
fn main() {
    println!("Hello, world!");
}
```

Please review the changes at: https://github.com/example/repo

Thanks!"#;

    let html_body = markdown_to_html(technical_email);
    
    // Verify technical content is properly formatted
    assert!(html_body.contains("<h2>Status Update</h2>"));
    assert!(html_body.contains("<code>main</code>"));
    assert!(html_body.contains("<pre><code"));
    assert!(html_body.contains("println!"));
    assert!(html_body.contains("https://github.com/example/repo"));
}