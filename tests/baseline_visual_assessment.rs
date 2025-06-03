use headless_chrome::{Browser, LaunchOptions, protocol::cdp::Page};
use reqwest::Client;
use serde_json;
use std::fs;
use std::time::Duration;
use tokio::time::sleep;

/// Comprehensive baseline visual assessment of email rendering
///
/// This test captures the current state of email rendering quality using headless Chrome
/// to establish a baseline before implementing Phase 2b improvements.

const TEST_EMAILS: &[&str] = &[
    "bilprovningen",
    "stockholm-film-festival",
    "max-dead-rising",
];
const SERVER_URL: &str = "http://127.0.0.1:8080";

pub struct BaselineVisualAssessment {
    browser: Browser,
    client: Client,
}

#[derive(Debug, Clone)]
pub struct EmailMetrics {
    pub email_name: String,
    pub container_width: f64,
    pub is_centered: bool,
    pub wrapper_count: usize,
    pub background_color: String,
    pub max_width: String,
    pub margin_left: String,
    pub margin_right: String,
    pub color_count: usize,
    pub font_size: String,
    pub has_table_styling: bool,
    pub responsive_breakpoints: usize,
}

impl BaselineVisualAssessment {
    pub async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let launch_options = LaunchOptions::default_builder()
            .headless(true)
            .window_size(Some((1200, 800)))
            .build()
            .expect("Could not find chrome-executable");

        let browser = Browser::new(launch_options)?;
        let client = Client::new();

        Ok(Self { browser, client })
    }

    pub async fn wait_for_server(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("Waiting for test gallery server...");
        for i in 0..20 {
            match self
                .client
                .get(&format!("{}/test/email-gallery", SERVER_URL))
                .send()
                .await
            {
                Ok(response) if response.status().is_success() => {
                    println!("Server is ready!");
                    return Ok(());
                }
                _ => {
                    if i == 0 {
                        println!("Server not ready, waiting...");
                    }
                    sleep(Duration::from_millis(500)).await;
                }
            }
        }
        Err("Server did not start within 10 seconds".into())
    }

    pub async fn capture_email_screenshot(
        &self,
        email_name: &str,
    ) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let url = format!("{}/test/email-gallery/{}", SERVER_URL, email_name);
        println!("Capturing screenshot for: {}", email_name);

        let tab = self.browser.new_tab()?;
        tab.navigate_to(&url)?;
        tab.wait_until_navigated()?;

        // Wait for content to load
        sleep(Duration::from_millis(1000)).await;

        let screenshot =
            tab.capture_screenshot(Page::CaptureScreenshotFormatOption::Png, None, None, true)?;
        Ok(screenshot)
    }

    pub async fn extract_email_metrics(
        &self,
        email_name: &str,
    ) -> Result<EmailMetrics, Box<dyn std::error::Error>> {
        let url = format!("{}/test/email-gallery/{}", SERVER_URL, email_name);
        println!("Extracting metrics for: {}", email_name);

        let tab = self.browser.new_tab()?;
        tab.navigate_to(&url)?;
        tab.wait_until_navigated()?;

        // Wait for content to load
        sleep(Duration::from_millis(1000)).await;

        // Extract computed styles from the email content area
        let container_width_js = r#"
            (function() {
                const emailContent = document.querySelector('.email-content, .thread-message-content, [class*="email"]');
                if (!emailContent) return 0;
                return emailContent.getBoundingClientRect().width;
            })()
        "#;

        let centering_check_js = r#"
            (function() {
                const emailContent = document.querySelector('.email-content, .thread-message-content, [class*="email"]');
                if (!emailContent) return false;
                
                // Check if the email content itself is centered
                const containerStyles = window.getComputedStyle(emailContent);
                const containerCentered = (containerStyles.marginLeft === 'auto' || containerStyles.marginLeft.includes('auto')) && 
                                        (containerStyles.marginRight === 'auto' || containerStyles.marginRight.includes('auto'));
                
                // Check for centered elements within the email
                const centeredElements = emailContent.querySelectorAll('table[align="center"], [style*="margin: 0 auto"], [style*="margin:0 auto"]');
                const maxWidthElements = emailContent.querySelectorAll('[style*="max-width"]');
                
                return containerCentered || centeredElements.length > 0 || 
                       (maxWidthElements.length > 0 && Array.from(maxWidthElements).some(el => {
                           const style = el.getAttribute('style') || '';
                           return style.includes('margin') && (style.includes('auto') || style.includes('center'));
                       }));
            })()
        "#;

        let wrapper_count_js = r#"
            (function() {
                const emailContent = document.querySelector('.email-content, .thread-message-content, [class*="email"]');
                if (!emailContent) return 0;
                let count = 0;
                let current = emailContent;
                while (current && current !== document.body) {
                    if (current.tagName === 'DIV') count++;
                    current = current.parentElement;
                }
                return count;
            })()
        "#;

        let color_analysis_js = r#"
            (function() {
                const emailContent = document.querySelector('.email-content, .thread-message-content, [class*="email"]');
                if (!emailContent) return 0;
                const allElements = emailContent.querySelectorAll('*');
                const colors = new Set();
                allElements.forEach(el => {
                    const styles = window.getComputedStyle(el);
                    if (styles.color !== 'rgb(0, 0, 0)' && styles.color !== 'rgba(0, 0, 0, 0)') {
                        colors.add(styles.color);
                    }
                    if (styles.backgroundColor !== 'rgba(0, 0, 0, 0)' && styles.backgroundColor !== 'transparent') {
                        colors.add(styles.backgroundColor);
                    }
                });
                return colors.size;
            })()
        "#;

        let table_check_js = r#"
            (function() {
                const emailContent = document.querySelector('.email-content, .thread-message-content, [class*="email"]');
                if (!emailContent) return false;
                const tables = emailContent.querySelectorAll('table');
                return tables.length > 0 && Array.from(tables).some(table => {
                    const styles = window.getComputedStyle(table);
                    return styles.borderCollapse !== 'separate' || 
                           styles.borderSpacing !== '2px' ||
                           table.hasAttribute('cellpadding') ||
                           table.hasAttribute('cellspacing');
                });
            })()
        "#;

        let style_info_js = r#"
            (function() {
                const emailContent = document.querySelector('.email-content, .thread-message-content, [class*="email"]');
                if (!emailContent) return {backgroundColor: '', maxWidth: '', marginLeft: '', marginRight: '', fontSize: ''};
                const styles = window.getComputedStyle(emailContent);
                return {
                    backgroundColor: styles.backgroundColor,
                    maxWidth: styles.maxWidth,
                    marginLeft: styles.marginLeft,
                    marginRight: styles.marginRight,
                    fontSize: styles.fontSize
                };
            })()
        "#;

        // Execute JavaScript to extract metrics with better error handling
        let container_width: f64 = tab
            .evaluate(container_width_js, false)?
            .value
            .and_then(|v| v.as_f64())
            .unwrap_or(0.0);
        let is_centered: bool = tab
            .evaluate(centering_check_js, false)?
            .value
            .and_then(|v| v.as_bool())
            .unwrap_or(false);
        let wrapper_count: usize = tab
            .evaluate(wrapper_count_js, false)?
            .value
            .and_then(|v| v.as_u64())
            .unwrap_or(0) as usize;
        let color_count: usize = tab
            .evaluate(color_analysis_js, false)?
            .value
            .and_then(|v| v.as_u64())
            .unwrap_or(0) as usize;
        let has_table_styling: bool = tab
            .evaluate(table_check_js, false)?
            .value
            .and_then(|v| v.as_bool())
            .unwrap_or(false);

        let style_info_result = tab.evaluate(style_info_js, false)?;
        let style_info = style_info_result
            .value
            .unwrap_or_else(|| serde_json::json!({}));
        let empty_map = serde_json::Map::new();
        let style_obj = style_info.as_object().unwrap_or(&empty_map);

        let background_color = style_obj
            .get("backgroundColor")
            .and_then(|v| v.as_str())
            .unwrap_or("unknown")
            .to_string();
        let max_width = style_obj
            .get("maxWidth")
            .and_then(|v| v.as_str())
            .unwrap_or("unknown")
            .to_string();
        let margin_left = style_obj
            .get("marginLeft")
            .and_then(|v| v.as_str())
            .unwrap_or("unknown")
            .to_string();
        let margin_right = style_obj
            .get("marginRight")
            .and_then(|v| v.as_str())
            .unwrap_or("unknown")
            .to_string();
        let font_size = style_obj
            .get("fontSize")
            .and_then(|v| v.as_str())
            .unwrap_or("unknown")
            .to_string();

        Ok(EmailMetrics {
            email_name: email_name.to_string(),
            container_width,
            is_centered,
            wrapper_count,
            background_color,
            max_width,
            margin_left,
            margin_right,
            color_count,
            font_size,
            has_table_styling,
            responsive_breakpoints: 0, // Would need viewport testing for this
        })
    }

    pub async fn generate_baseline_report(&self) -> Result<String, Box<dyn std::error::Error>> {
        // Create directories for output
        fs::create_dir_all("target/baseline-assessment")?;
        fs::create_dir_all("target/baseline-assessment/screenshots")?;

        let mut report = String::new();
        report.push_str("BASELINE EMAIL RENDERING ASSESSMENT\n");
        report.push_str("=====================================\n");
        report.push_str(&format!(
            "Date: {}\n",
            chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC")
        ));
        report.push_str(&format!("Server: {}\n", SERVER_URL));
        report.push_str(&format!("Test Emails: {}\n\n", TEST_EMAILS.len()));

        for email_name in TEST_EMAILS {
            println!("Processing email: {}", email_name);

            // Capture screenshot
            match self.capture_email_screenshot(email_name).await {
                Ok(screenshot) => {
                    let screenshot_path =
                        format!("target/baseline-assessment/screenshots/{}.png", email_name);
                    fs::write(&screenshot_path, screenshot)?;
                    println!("Screenshot saved: {}", screenshot_path);
                }
                Err(e) => {
                    println!("Failed to capture screenshot for {}: {}", email_name, e);
                }
            }

            // Extract metrics
            match self.extract_email_metrics(email_name).await {
                Ok(metrics) => {
                    report.push_str(&format!("📧 {}\n", email_name.to_uppercase()));
                    report.push_str(&format!(
                        "   Container Width: {:.1}px\n",
                        metrics.container_width
                    ));
                    report.push_str(&format!(
                        "   Is Centered: {} {}\n",
                        if metrics.is_centered { "Yes" } else { "No" },
                        if metrics.is_centered { "✅" } else { "❌" }
                    ));
                    report.push_str(&format!(
                        "   Wrapper Containers: {} {}\n",
                        metrics.wrapper_count,
                        if metrics.wrapper_count <= 3 {
                            "✅"
                        } else {
                            "⚠️"
                        }
                    ));
                    report.push_str(&format!("   Max Width: {}\n", metrics.max_width));
                    report.push_str(&format!(
                        "   Margins: {} / {}\n",
                        metrics.margin_left, metrics.margin_right
                    ));
                    report.push_str(&format!("   Background: {}\n", metrics.background_color));
                    report.push_str(&format!("   Font Size: {}\n", metrics.font_size));
                    report.push_str(&format!(
                        "   Color Preservation: {} colors {}\n",
                        metrics.color_count,
                        if metrics.color_count >= 3 {
                            "✅"
                        } else {
                            "⚠️"
                        }
                    ));
                    report.push_str(&format!(
                        "   Table Styling: {} {}\n",
                        if metrics.has_table_styling {
                            "Preserved"
                        } else {
                            "Lost"
                        },
                        if metrics.has_table_styling {
                            "✅"
                        } else {
                            "❌"
                        }
                    ));
                    report.push_str("\n");
                }
                Err(e) => {
                    report.push_str(&format!(
                        "📧 {} - ERROR: {}\n\n",
                        email_name.to_uppercase(),
                        e
                    ));
                }
            }
        }

        // Summary
        report.push_str("BASELINE SUMMARY\n");
        report.push_str("================\n");
        report.push_str("This assessment captures the current state of email rendering before Phase 2b improvements.\n");
        report.push_str("Key issues to address:\n");
        report.push_str("• Email centering problems (max-width + margin auto patterns)\n");
        report.push_str("• CSS property preservation\n");
        report.push_str("• Container optimization\n");
        report.push_str("• Table layout integrity\n");
        report.push_str("• Responsive design support\n\n");
        report.push_str("Next Steps: Implement Phase 2b enhanced email fidelity improvements\n");

        Ok(report)
    }
}

#[tokio::test]
async fn test_baseline_visual_assessment() {
    println!("Starting baseline visual assessment...");

    match BaselineVisualAssessment::new().await {
        Ok(assessment) => {
            // Wait for server to be ready
            match assessment.wait_for_server().await {
                Ok(_) => {
                    println!("Server is ready, generating baseline report...");

                    match assessment.generate_baseline_report().await {
                        Ok(report) => {
                            println!("Baseline assessment completed successfully!");
                            println!("\n{}", report);

                            // Save report to file
                            let report_path = "target/baseline-assessment/baseline_report.txt";
                            std::fs::write(report_path, &report).unwrap();
                            println!("Report saved to: {}", report_path);
                        }
                        Err(e) => {
                            println!("Failed to generate baseline report: {}", e);
                            assert!(false, "Baseline assessment failed");
                        }
                    }
                }
                Err(e) => {
                    println!("Server not available: {}", e);
                    println!("This test requires the test-gallery-server to be running");
                    println!("Run: devenv shell cargo run --bin test-gallery-server");
                    // Don't fail the test if server isn't running - this is expected in CI
                }
            }
        }
        Err(e) => {
            println!(
                "Failed to create visual assessment (Chrome not available): {}",
                e
            );
            // Don't fail the test if Chrome isn't available - this is expected in some environments
        }
    }
}
