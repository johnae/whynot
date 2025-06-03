use std::path::Path;

/// Simplified visual diff testing for email rendering quality measurement
///
/// This module provides basic structure for visual regression testing without
/// requiring complex browser automation setup.

#[derive(Debug, Clone)]
pub struct EmailRenderingQuality {
    pub email_name: String,
    pub centering_score: f64,      // 0.0-1.0, how well email is centered
    pub container_efficiency: f64, // 0.0-1.0, wrapper container optimization
    pub css_fidelity: f64,         // 0.0-1.0, CSS property preservation
    pub table_integrity: f64,      // 0.0-1.0, table layout preservation
    pub responsive_score: f64,     // 0.0-1.0, responsive design functionality
    pub overall_quality: f64,      // 0.0-1.0, weighted overall score
}

impl EmailRenderingQuality {
    pub fn new(email_name: String) -> Self {
        Self {
            email_name,
            centering_score: 0.0,
            container_efficiency: 0.0,
            css_fidelity: 0.0,
            table_integrity: 0.0,
            responsive_score: 0.0,
            overall_quality: 0.0,
        }
    }

    pub fn calculate_overall_score(&mut self) {
        // Weighted scoring based on TODO.md priorities
        self.overall_quality = self.centering_score * 0.25 +        // High priority: centering issues
            self.container_efficiency * 0.20 +   // High priority: wrapper containers
            self.css_fidelity * 0.20 +          // High priority: CSS preservation
            self.table_integrity * 0.20 +       // Medium priority: table layouts
            self.responsive_score * 0.15; // Medium priority: responsive design
    }

    pub fn generate_quality_report(qualities: &[EmailRenderingQuality]) -> String {
        let mut report = String::new();
        report.push_str("EMAIL RENDERING QUALITY REPORT\n");
        report.push_str("==============================\n\n");

        for quality in qualities {
            report.push_str(&format!("📧 {}\n", quality.email_name.to_uppercase()));
            report.push_str(&format!(
                "   Overall Quality: {:.1}% {}\n",
                quality.overall_quality * 100.0,
                if quality.overall_quality >= 0.8 {
                    "🟢"
                } else if quality.overall_quality >= 0.6 {
                    "🟡"
                } else {
                    "🔴"
                }
            ));
            report.push_str(&format!(
                "   ├─ Centering: {:.1}% {}\n",
                quality.centering_score * 100.0,
                if quality.centering_score >= 0.7 {
                    "✅"
                } else {
                    "❌"
                }
            ));
            report.push_str(&format!(
                "   ├─ Container Efficiency: {:.1}% {}\n",
                quality.container_efficiency * 100.0,
                if quality.container_efficiency >= 0.7 {
                    "✅"
                } else {
                    "❌"
                }
            ));
            report.push_str(&format!(
                "   ├─ CSS Fidelity: {:.1}% {}\n",
                quality.css_fidelity * 100.0,
                if quality.css_fidelity >= 0.7 {
                    "✅"
                } else {
                    "❌"
                }
            ));
            report.push_str(&format!(
                "   ├─ Table Integrity: {:.1}% {}\n",
                quality.table_integrity * 100.0,
                if quality.table_integrity >= 0.7 {
                    "✅"
                } else {
                    "❌"
                }
            ));
            report.push_str(&format!(
                "   └─ Responsive Design: {:.1}% {}\n\n",
                quality.responsive_score * 100.0,
                if quality.responsive_score >= 0.7 {
                    "✅"
                } else {
                    "❌"
                }
            ));
        }

        // Summary statistics
        let avg_quality: f64 =
            qualities.iter().map(|q| q.overall_quality).sum::<f64>() / qualities.len() as f64;
        let problematic_emails: Vec<_> = qualities
            .iter()
            .filter(|q| q.overall_quality < 0.7)
            .collect();

        report.push_str("SUMMARY\n");
        report.push_str("=======\n");
        report.push_str(&format!(
            "Average Quality Score: {:.1}%\n",
            avg_quality * 100.0
        ));
        report.push_str(&format!(
            "Emails with Quality Issues: {}/{}\n",
            problematic_emails.len(),
            qualities.len()
        ));

        if !problematic_emails.is_empty() {
            report.push_str("\nPROBLEMS REQUIRING ATTENTION:\n");
            for email in problematic_emails {
                report.push_str(&format!(
                    "• {}: {:.1}% overall quality\n",
                    email.email_name,
                    email.overall_quality * 100.0
                ));

                if email.centering_score < 0.7 {
                    report.push_str("  - Centering issues (max-width, margin auto patterns)\n");
                }
                if email.container_efficiency < 0.7 {
                    report.push_str("  - Excessive wrapper containers creating visual frames\n");
                }
                if email.css_fidelity < 0.7 {
                    report.push_str("  - CSS properties being stripped or not preserved\n");
                }
                if email.table_integrity < 0.7 {
                    report.push_str("  - Table layout attributes and styling lost\n");
                }
                if email.responsive_score < 0.7 {
                    report.push_str("  - Responsive design patterns not working\n");
                }
            }
        }

        report
    }
}

#[test]
fn test_email_rendering_quality_creation() {
    let mut quality = EmailRenderingQuality::new("test-email".to_string());

    // Set some test scores
    quality.centering_score = 0.8;
    quality.container_efficiency = 0.7;
    quality.css_fidelity = 0.9;
    quality.table_integrity = 0.6;
    quality.responsive_score = 0.5;

    quality.calculate_overall_score();

    // Check that overall score is calculated correctly
    let expected = 0.8 * 0.25 + 0.7 * 0.20 + 0.9 * 0.20 + 0.6 * 0.20 + 0.5 * 0.15;
    assert!((quality.overall_quality - expected).abs() < 0.001);

    println!(
        "Overall quality score: {:.1}%",
        quality.overall_quality * 100.0
    );
}

#[test]
fn test_quality_report_generation() {
    let mut qualities = vec![
        EmailRenderingQuality::new("bilprovningen".to_string()),
        EmailRenderingQuality::new("stockholm-film-festival".to_string()),
        EmailRenderingQuality::new("max-dead-rising".to_string()),
    ];

    // Set some test scores to demonstrate different quality levels
    qualities[0].centering_score = 0.6; // Poor centering
    qualities[0].container_efficiency = 0.8;
    qualities[0].css_fidelity = 0.7;
    qualities[0].table_integrity = 0.9;
    qualities[0].responsive_score = 0.5;
    qualities[0].calculate_overall_score();

    qualities[1].centering_score = 0.9; // Good overall
    qualities[1].container_efficiency = 0.8;
    qualities[1].css_fidelity = 0.8;
    qualities[1].table_integrity = 0.7;
    qualities[1].responsive_score = 0.8;
    qualities[1].calculate_overall_score();

    qualities[2].centering_score = 0.5; // Poor overall
    qualities[2].container_efficiency = 0.6;
    qualities[2].css_fidelity = 0.6;
    qualities[2].table_integrity = 0.5;
    qualities[2].responsive_score = 0.4;
    qualities[2].calculate_overall_score();

    let report = EmailRenderingQuality::generate_quality_report(&qualities);

    assert!(report.contains("EMAIL RENDERING QUALITY REPORT"));
    assert!(report.contains("BILPROVNINGEN"));
    assert!(report.contains("STOCKHOLM-FILM-FESTIVAL"));
    assert!(report.contains("MAX-DEAD-RISING"));
    assert!(report.contains("SUMMARY"));
    assert!(report.contains("PROBLEMS REQUIRING ATTENTION"));

    // Check that centering issues are identified
    assert!(report.contains("Centering issues"));

    println!("Quality report:\n{}", report);
}

#[test]
fn test_visual_diff_directory_creation() {
    std::fs::create_dir_all("target/visual-diffs").unwrap();
    std::fs::create_dir_all("target/quality-reports").unwrap();

    assert!(Path::new("target/visual-diffs").exists());
    assert!(Path::new("target/quality-reports").exists());

    // Create a sample quality report
    let qualities = vec![EmailRenderingQuality::new("sample-email".to_string())];

    let report = EmailRenderingQuality::generate_quality_report(&qualities);
    std::fs::write("target/quality-reports/sample_quality_report.txt", &report).unwrap();

    assert!(Path::new("target/quality-reports/sample_quality_report.txt").exists());
}
