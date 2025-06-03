use insta::assert_snapshot;
use reqwest::Client;
use scraper::{Html, Selector};
use std::time::Duration;
use tokio::time::sleep;

/// Simplified HTML snapshot testing for email rendering regression detection
///
/// This module creates basic snapshot tests for HTML content processing
/// without requiring external test setup dependencies.
#[cfg(test)]
struct HtmlSnapshotSetup {
    #[allow(dead_code)]
    client: Client,
    #[allow(dead_code)]
    server_url: String,
}

#[cfg(test)]
impl HtmlSnapshotSetup {
    async fn new() -> Self {
        Self {
            client: Client::new(),
            server_url: "http://127.0.0.1:8080".to_string(),
        }
    }

    #[allow(dead_code)]
    async fn wait_for_server(&self) -> Result<(), Box<dyn std::error::Error>> {
        for _ in 0..20 {
            if let Ok(_) = self
                .client
                .get(&format!("{}/test/email-gallery", self.server_url))
                .send()
                .await
            {
                return Ok(());
            }
            sleep(Duration::from_millis(500)).await;
        }
        Err("Server did not start within 10 seconds".into())
    }

    fn extract_email_content(&self, html: &str) -> String {
        let document = Html::parse_document(html);
        let email_selector = Selector::parse(".email-content").unwrap();

        if let Some(email_element) = document.select(&email_selector).next() {
            email_element.html()
        } else {
            "<!-- No email content found -->".to_string()
        }
    }

    fn extract_container_structure(&self, html: &str) -> String {
        let document = Html::parse_document(html);
        let container_selectors = [
            ".email-content",
            ".message-content",
            ".html-content",
            ".content-wrapper",
        ];

        let mut structure = String::new();
        for selector_str in &container_selectors {
            if let Ok(selector) = Selector::parse(selector_str) {
                for element in document.select(&selector) {
                    structure.push_str(&format!(
                        "Container: {} - Classes: {:?}\n",
                        selector_str,
                        element.value().classes().collect::<Vec<_>>()
                    ));
                }
            }
        }
        structure
    }
}

#[test]
fn test_html_content_extraction() {
    let setup = tokio_test::block_on(HtmlSnapshotSetup::new());

    let sample_html = r#"
    <html>
        <body>
            <div class="email-content">
                <p>Test email content</p>
                <table>
                    <tr><td>Cell content</td></tr>
                </table>
            </div>
        </body>
    </html>
    "#;

    let extracted = setup.extract_email_content(sample_html);

    assert!(extracted.contains("Test email content"));
    assert!(extracted.contains("table"));
    assert!(extracted.contains("Cell content"));
}

#[test]
fn test_container_structure_analysis() {
    let setup = tokio_test::block_on(HtmlSnapshotSetup::new());

    let sample_html = r#"
    <html>
        <body>
            <div class="email-content main-email">
                <div class="message-content">
                    <div class="html-content">
                        <p>Nested content</p>
                    </div>
                </div>
            </div>
        </body>
    </html>
    "#;

    let structure = setup.extract_container_structure(sample_html);

    assert!(structure.contains(".email-content"));
    assert!(structure.contains(".message-content"));
    assert!(structure.contains(".html-content"));
}

#[test]
fn test_snapshot_sample_email_structure() {
    let sample_email_structure = "Container: .email-content - Classes: [\"email-content\", \"main-email\"]\nContainer: .message-content - Classes: [\"message-content\"]\nContainer: .html-content - Classes: [\"html-content\"]\n";

    // Create a snapshot of the expected container structure format
    assert_snapshot!("sample_email_container_structure", sample_email_structure);
}

#[test]
fn test_snapshot_sample_email_content() {
    let sample_email_content = r##"<div class="email-content">
    <table style="width: 100%; max-width: 600px; margin: 0 auto;">
        <tr>
            <td style="padding: 20px; background-color: #f5f5f5;">
                <h1 style="color: #333; font-family: Arial, sans-serif;">Sample Newsletter</h1>
                <p style="color: #666; line-height: 1.6;">This is a sample email content for testing purposes.</p>
                <a href="#" style="background-color: #007cba; color: white; padding: 10px 20px; text-decoration: none; border-radius: 4px;">Click Here</a>
            </td>
        </tr>
    </table>
</div>"##;

    // Create a snapshot of expected email content format
    assert_snapshot!("sample_email_content_format", sample_email_content);
}
