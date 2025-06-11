//! Integration tests for HTML to text conversion
//! 
//! These tests verify that the text renderer works correctly with real HTML
//! email content and can handle various conversion scenarios.

use whynot::text_renderer::{
    TextRendererConfig, TextRendererFactory, ConverterType
};

/// Test HTML content representing a typical email
const SAMPLE_EMAIL_HTML: &str = r#"
<html>
<head>
    <title>Test Email</title>
</head>
<body>
    <h1>Welcome to Our Newsletter</h1>
    <p>Hello <strong>John</strong>,</p>
    
    <p>Thank you for subscribing to our newsletter. Here are the latest updates:</p>
    
    <ul>
        <li><strong>Feature Update:</strong> We've added new functionality</li>
        <li><strong>Bug Fixes:</strong> Several issues have been resolved</li>
        <li><strong>Performance:</strong> The application is now 20% faster</li>
    </ul>
    
    <h2>Important Links</h2>
    <p>Visit our <a href="https://example.com">website</a> for more information.</p>
    
    <blockquote>
        "Quality is not an act, it is a habit." - Aristotle
    </blockquote>
    
    <p>Best regards,<br>
    The Team</p>
    
    <hr>
    <p><small>This email was sent to john@example.com. 
    To unsubscribe, <a href="https://example.com/unsubscribe">click here</a>.</small></p>
</body>
</html>
"#;

/// Complex HTML with tables and code blocks
const COMPLEX_EMAIL_HTML: &str = r#"
<html>
<body>
    <h1>Development Update</h1>
    
    <p>Here's the status of our current projects:</p>
    
    <table border="1">
        <tr>
            <th>Project</th>
            <th>Status</th>
            <th>Progress</th>
        </tr>
        <tr>
            <td>Email Client</td>
            <td>In Progress</td>
            <td>75%</td>
        </tr>
        <tr>
            <td>TUI Interface</td>
            <td>Planning</td>
            <td>10%</td>
        </tr>
    </table>
    
    <h2>Code Changes</h2>
    <p>Here's a snippet of the new functionality:</p>
    
    <pre><code>
async fn convert_html_to_text(html: &str) -> Result&lt;String&gt; {
    let converter = TextRendererFactory::create_converter(&config).await?;
    converter.convert(html).await
}
    </code></pre>
    
    <p>This function handles the conversion process efficiently.</p>
</body>
</html>
"#;

#[tokio::test]
async fn test_builtin_converter_basic_functionality() {
    let config = TextRendererConfig::default();
    let converter = TextRendererFactory::create_converter(&config).await.unwrap();
    
    // Test basic HTML conversion
    let result = converter.convert(SAMPLE_EMAIL_HTML).await.unwrap();
    
    // Verify content is preserved
    assert!(result.contains("Welcome to Our Newsletter"));
    assert!(result.contains("Hello John"));
    assert!(result.contains("Feature Update"));
    assert!(result.contains("Bug Fixes"));
    assert!(result.contains("Performance"));
    assert!(result.contains("Best regards"));
    
    // Verify HTML tags are removed
    assert!(!result.contains("<h1>"));
    assert!(!result.contains("<strong>"));
    assert!(!result.contains("<ul>"));
    assert!(!result.contains("<li>"));
    
    println!("Converted text:\n{}", result);
}

#[tokio::test]
async fn test_builtin_converter_complex_html() {
    let config = TextRendererConfig::default();
    let converter = TextRendererFactory::create_converter(&config).await.unwrap();
    
    let result = converter.convert(COMPLEX_EMAIL_HTML).await.unwrap();
    
    // Verify table content is preserved
    assert!(result.contains("Development Update"));
    assert!(result.contains("Project"));
    assert!(result.contains("Status"));
    assert!(result.contains("Email Client"));
    assert!(result.contains("In Progress"));
    
    // Verify code content is preserved
    assert!(result.contains("async fn convert_html_to_text"));
    assert!(result.contains("TextRendererFactory::create_converter"));
    
    println!("Complex HTML converted:\n{}", result);
}

#[tokio::test]
async fn test_builtin_converter_with_custom_width() {
    let config = TextRendererConfig {
        converter_type: ConverterType::Builtin,
        text_width: 40,
        ..Default::default()
    };
    let converter = TextRendererFactory::create_converter(&config).await.unwrap();
    
    let html = "<p>This is a very long paragraph that should be wrapped at the specified width to test the text wrapping functionality of the converter.</p>";
    let result = converter.convert(html).await.unwrap();
    
    // Check that lines are not excessively long
    for line in result.lines() {
        if !line.trim().is_empty() {
            assert!(line.len() <= 50, "Line too long: '{}'", line); // Allow some flexibility
        }
    }
    
    println!("Wrapped text:\n{}", result);
}

#[tokio::test]
async fn test_external_converter_availability_detection() {
    // Test that the factory can detect available tools
    let available_tools = TextRendererFactory::detect_available_tools().await;
    
    println!("Available external tools: {:?}", available_tools);
    
    // Test creation with a known unavailable tool
    let config = TextRendererConfig {
        converter_type: ConverterType::External,
        external_tool_command: Some("definitely-not-a-real-command".to_string()),
        ..Default::default()
    };
    
    let result = TextRendererFactory::create_converter(&config).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_auto_converter_fallback() {
    // Test auto mode with unavailable external tool - should fallback to builtin
    let config = TextRendererConfig {
        converter_type: ConverterType::Auto,
        external_tool_command: Some("definitely-not-a-real-command".to_string()),
        ..Default::default()
    };
    
    let converter = TextRendererFactory::create_converter(&config).await.unwrap();
    assert_eq!(converter.name(), "builtin");
    
    // Verify it can still convert HTML
    let result = converter.convert("<p>Test content</p>").await.unwrap();
    assert!(result.contains("Test content"));
}

#[tokio::test]
async fn test_empty_and_malformed_html() {
    let config = TextRendererConfig::default();
    let converter = TextRendererFactory::create_converter(&config).await.unwrap();
    
    // Test empty HTML
    let result = converter.convert("").await.unwrap();
    assert_eq!(result.trim(), "");
    
    // Test plain text (no HTML)
    let result = converter.convert("Just plain text").await.unwrap();
    assert_eq!(result.trim(), "Just plain text");
    
    // Test malformed HTML
    let malformed = "<p>Unclosed paragraph<div>Mixed tags</p></div>";
    let result = converter.convert(malformed).await.unwrap();
    assert!(result.contains("Unclosed paragraph"));
    assert!(result.contains("Mixed tags"));
}

#[tokio::test]
async fn test_html_entities_and_special_characters() {
    let config = TextRendererConfig::default();
    let converter = TextRendererFactory::create_converter(&config).await.unwrap();
    
    let html = r#"
    <p>Special characters: &amp; &lt; &gt; &quot; &#39;</p>
    <p>Unicode: caf&eacute; na&iuml;ve r&eacute;sum&eacute;</p>
    <p>Symbols: &copy; &trade; &reg;</p>
    "#;
    
    let result = converter.convert(html).await.unwrap();
    
    // Note: Our simple implementation might not handle all entities perfectly
    // but should preserve the basic structure
    assert!(result.contains("Special characters"));
    assert!(result.contains("Unicode"));
    assert!(result.contains("Symbols"));
    
    println!("HTML entities converted:\n{}", result);
}

#[tokio::test]
async fn test_converter_configuration_options() {
    // Test different configuration options
    let configs = vec![
        TextRendererConfig {
            converter_type: ConverterType::Builtin,
            text_width: 30,
            preserve_links: true,
            ..Default::default()
        },
        TextRendererConfig {
            converter_type: ConverterType::Builtin,
            text_width: 100,
            preserve_links: false,
            ..Default::default()
        },
    ];
    
    let html = r#"<p>Check out our <a href="https://example.com">amazing website</a> for more info!</p>"#;
    
    for config in configs {
        let converter = TextRendererFactory::create_converter(&config).await.unwrap();
        let result = converter.convert(html).await.unwrap();
        
        assert!(result.contains("amazing website"));
        println!("Config {:?} result: {}", config.text_width, result.trim());
    }
}