//! Markdown to HTML conversion for TUI email composition.

use pulldown_cmark::{html, Options, Parser};

/// Converts markdown text to HTML for email composition.
///
/// This function takes markdown text and converts it to HTML suitable for
/// sending as the text/html part of a multipart email message. The conversion
/// includes proper handling of common markdown features like headers, lists,
/// links, code blocks, and basic formatting.
pub fn markdown_to_html(markdown: &str) -> String {
    // Enable GitHub-flavored markdown extensions
    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_FOOTNOTES);
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TASKLISTS);

    let parser = Parser::new_ext(markdown, options);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);

    html_output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_markdown_to_html_basic_formatting() {
        let markdown = "**bold** and *italic* text";
        let html = markdown_to_html(markdown);
        assert!(html.contains("<strong>bold</strong>"));
        assert!(html.contains("<em>italic</em>"));
    }

    #[test]
    fn test_markdown_to_html_headers() {
        let markdown = "# Header 1\n## Header 2\n### Header 3";
        let html = markdown_to_html(markdown);
        assert!(html.contains("<h1>Header 1</h1>"));
        assert!(html.contains("<h2>Header 2</h2>"));
        assert!(html.contains("<h3>Header 3</h3>"));
    }

    #[test]
    fn test_markdown_to_html_lists() {
        let markdown = "- Item 1\n- Item 2\n- Item 3";
        let html = markdown_to_html(markdown);
        assert!(html.contains("<ul>"));
        assert!(html.contains("<li>Item 1</li>"));
        assert!(html.contains("<li>Item 2</li>"));
        assert!(html.contains("<li>Item 3</li>"));
        assert!(html.contains("</ul>"));
    }

    #[test]
    fn test_markdown_to_html_ordered_lists() {
        let markdown = "1. First\n2. Second\n3. Third";
        let html = markdown_to_html(markdown);
        assert!(html.contains("<ol>"));
        assert!(html.contains("<li>First</li>"));
        assert!(html.contains("<li>Second</li>"));
        assert!(html.contains("<li>Third</li>"));
        assert!(html.contains("</ol>"));
    }

    #[test]
    fn test_markdown_to_html_links() {
        let markdown = "[Link text](https://example.com)";
        let html = markdown_to_html(markdown);
        assert!(html.contains("<a href=\"https://example.com\">Link text</a>"));
    }

    #[test]
    fn test_markdown_to_html_code_inline() {
        let markdown = "Here is `inline code` example";
        let html = markdown_to_html(markdown);
        assert!(html.contains("<code>inline code</code>"));
    }

    #[test]
    fn test_markdown_to_html_code_block() {
        let markdown = "```rust\nfn main() {\n    println!(\"Hello\");\n}\n```";
        let html = markdown_to_html(markdown);
        assert!(html.contains("<pre><code"));
        assert!(html.contains("fn main()"));
        assert!(html.contains("println!"));
    }

    #[test]
    fn test_markdown_to_html_blockquotes() {
        let markdown = "> This is a blockquote\n> with multiple lines";
        let html = markdown_to_html(markdown);
        assert!(html.contains("<blockquote>"));
        assert!(html.contains("This is a blockquote"));
        assert!(html.contains("with multiple lines"));
        assert!(html.contains("</blockquote>"));
    }

    #[test]
    fn test_markdown_to_html_tables() {
        let markdown = "| Header 1 | Header 2 |\n|----------|----------|\n| Cell 1   | Cell 2   |";
        let html = markdown_to_html(markdown);
        assert!(html.contains("<table>"));
        assert!(html.contains("<th>Header 1</th>"));
        assert!(html.contains("<th>Header 2</th>"));
        assert!(html.contains("<td>Cell 1</td>"));
        assert!(html.contains("<td>Cell 2</td>"));
        assert!(html.contains("</table>"));
    }

    #[test]
    fn test_markdown_to_html_strikethrough() {
        let markdown = "~~strikethrough text~~";
        let html = markdown_to_html(markdown);
        assert!(html.contains("<del>strikethrough text</del>"));
    }

    #[test]
    fn test_markdown_to_html_paragraphs() {
        let markdown = "First paragraph.\n\nSecond paragraph.";
        let html = markdown_to_html(markdown);
        assert!(html.contains("<p>First paragraph.</p>"));
        assert!(html.contains("<p>Second paragraph.</p>"));
    }

    #[test]
    fn test_markdown_to_html_line_breaks() {
        let markdown = "Line 1  \nLine 2";
        let html = markdown_to_html(markdown);
        assert!(html.contains("Line 1<br />"));
        assert!(html.contains("Line 2"));
    }

    #[test]
    fn test_markdown_to_html_empty_input() {
        let html = markdown_to_html("");
        assert_eq!(html, "");
    }

    #[test]
    fn test_markdown_to_html_plain_text() {
        let markdown = "Just plain text with no markdown";
        let html = markdown_to_html(markdown);
        assert!(html.contains("<p>Just plain text with no markdown</p>"));
    }

    #[test]
    fn test_markdown_to_html_complex_email() {
        let markdown = r#"# Email Subject

Hello **John**,

I hope this email finds you well. Here are the key points:

- First point with *emphasis*
- Second point with [a link](https://example.com)
- Third point with `inline code`

## Code Example

```rust
fn greet(name: &str) {
    println!("Hello, {}!", name);
}
```

> This is an important note that should be highlighted.

Best regards,  
Jane

---

P.S. Check out this table:

| Feature | Status |
|---------|--------|
| Markdown | ✅ |
| HTML | ✅ |
"#;

        let html = markdown_to_html(markdown);
        
        // Check major components are present
        assert!(html.contains("<h1>Email Subject</h1>"));
        assert!(html.contains("<strong>John</strong>"));
        assert!(html.contains("<em>emphasis</em>"));
        assert!(html.contains("<a href=\"https://example.com\">a link</a>"));
        assert!(html.contains("<code>inline code</code>"));
        assert!(html.contains("<h2>Code Example</h2>"));
        assert!(html.contains("<pre><code"));
        assert!(html.contains("fn greet"));
        assert!(html.contains("<blockquote>"));
        assert!(html.contains("<table>"));
        assert!(html.contains("<th>Feature</th>"));
        assert!(html.contains("<td>✅</td>"));
    }
}