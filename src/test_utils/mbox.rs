use chrono::{DateTime, Utc};
use mail_builder::MessageBuilder;

#[derive(Debug, Clone)]
pub struct EmailMessage {
    pub from: String,
    pub to: Vec<String>,
    pub subject: String,
    pub body: String,
    pub html_body: Option<String>,
    pub body_type: BodyType,
    pub date: DateTime<Utc>,
    pub message_id: Option<String>,
    pub in_reply_to: Option<String>,
    pub attachments: Vec<Attachment>,
}

#[derive(Debug, Clone)]
pub enum BodyType {
    Plain,
    Html,
    MultipartAlternative { plain: String, html: String },
}

#[derive(Debug, Clone)]
pub struct Attachment {
    pub filename: String,
    pub content_type: String,
    pub content: Vec<u8>,
}

impl EmailMessage {
    pub fn new(subject: impl Into<String>) -> Self {
        Self {
            from: "sender@example.com".to_string(),
            to: vec!["recipient@example.com".to_string()],
            subject: subject.into(),
            body: "This is a test message.".to_string(),
            html_body: None,
            body_type: BodyType::Plain,
            date: Utc::now(),
            message_id: None,
            in_reply_to: None,
            attachments: Vec::new(),
        }
    }

    pub fn with_from(mut self, from: impl Into<String>) -> Self {
        self.from = from.into();
        self
    }

    pub fn with_to(mut self, to: Vec<String>) -> Self {
        self.to = to;
        self
    }

    pub fn with_body(mut self, body: impl Into<String>) -> Self {
        self.body = body.into();
        self.body_type = BodyType::Plain;
        self
    }

    pub fn with_html_body(mut self, html_body: impl Into<String>) -> Self {
        self.html_body = Some(html_body.into());
        self.body_type = BodyType::Html;
        self
    }

    pub fn with_multipart_alternative_body(
        mut self,
        plain: impl Into<String>,
        html: impl Into<String>,
    ) -> Self {
        let plain_text = plain.into();
        let html_text = html.into();
        self.body = plain_text.clone();
        self.html_body = Some(html_text.clone());
        self.body_type = BodyType::MultipartAlternative {
            plain: plain_text,
            html: html_text,
        };
        self
    }

    pub fn with_date(mut self, date: DateTime<Utc>) -> Self {
        self.date = date;
        self
    }

    pub fn with_message_id(mut self, id: impl Into<String>) -> Self {
        self.message_id = Some(id.into());
        self
    }

    pub fn with_in_reply_to(mut self, id: impl Into<String>) -> Self {
        self.in_reply_to = Some(id.into());
        self
    }

    pub fn with_attachment(
        mut self,
        filename: impl Into<String>,
        content_type: impl Into<String>,
        content: &[u8],
    ) -> Self {
        self.attachments.push(Attachment {
            filename: filename.into(),
            content_type: content_type.into(),
            content: content.to_vec(),
        });
        self
    }

    /// Create an Outlook-style table-based HTML email layout
    pub fn with_outlook_table_layout(mut self) -> Self {
        let html = r#"
            <html>
            <head>
                <style>
                    .email-header { background-color: #f0f0f0; padding: 10px; border: 1px solid #ccc; }
                    .email-body { background-color: white; padding: 20px; border: 1px solid #ddd; }
                    .signature { font-size: 12px; color: #666; border-top: 1px solid #eee; padding-top: 10px; }
                    table { width: 100%; border-collapse: collapse; }
                    td { padding: 8px; border: 1px solid #ddd; }
                </style>
            </head>
            <body>
                <table class="email-container" width="600" cellpadding="0" cellspacing="0">
                    <tr>
                        <td class="email-header">
                            <h2 style="margin: 0; color: #333;">Important Business Update</h2>
                        </td>
                    </tr>
                    <tr>
                        <td class="email-body">
                            <p>Dear Team,</p>
                            <table class="data-table" border="1">
                                <tr style="background-color: #f9f9f9;">
                                    <th>Quarter</th>
                                    <th>Revenue</th>
                                    <th>Growth</th>
                                </tr>
                                <tr>
                                    <td>Q1</td>
                                    <td style="color: green; font-weight: bold;">$100K</td>
                                    <td>+15%</td>
                                </tr>
                                <tr>
                                    <td>Q2</td>
                                    <td style="color: green; font-weight: bold;">$120K</td>
                                    <td>+20%</td>
                                </tr>
                            </table>
                            <p>Please review the attached documents.</p>
                        </td>
                    </tr>
                    <tr>
                        <td class="signature">
                            <p>Best regards,<br/>Business Team</p>
                        </td>
                    </tr>
                </table>
            </body>
            </html>
        "#;
        self.html_body = Some(html.to_string());
        self.body_type = BodyType::Html;
        self
    }

    /// Create a Gmail-style nested div layout
    pub fn with_gmail_nested_layout(mut self) -> Self {
        let html = r###"
            <div class="gmail-wrapper" style="font-family: Arial, sans-serif; max-width: 700px;">
                <div class="header-section" style="background: linear-gradient(135deg, rgb(102, 126, 234) 0%, rgb(118, 75, 162) 100%); color: white; padding: 20px; border-radius: 8px 8px 0 0;">
                    <h1 style="margin: 0; font-size: 24px;">Newsletter Update</h1>
                    <p style="margin: 5px 0 0 0; opacity: 0.9;">Your weekly digest</p>
                </div>
                <div class="content-section" style="background: white; padding: 30px; border: 1px solid rgb(224, 224, 224);">
                    <div class="article-container" style="margin-bottom: 25px;">
                        <h2 style="color: rgb(51, 51, 51); border-bottom: 2px solid rgb(102, 126, 234); padding-bottom: 5px;">Featured Article</h2>
                        <div class="article-content" style="display: flex; gap: 20px; align-items: flex-start;">
                            <div class="text-content" style="flex: 2;">
                                <p style="line-height: 1.6; color: rgb(85, 85, 85);">
                                    Lorem ipsum dolor sit amet, consectetur adipiscing elit. 
                                    Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.
                                </p>
                                <a href="#link" style="color: rgb(102, 126, 234); text-decoration: none; font-weight: bold;">Read more →</a>
                            </div>
                            <div class="image-placeholder" style="flex: 1; background: rgb(240, 240, 240); height: 100px; border-radius: 4px; display: flex; align-items: center; justify-content: center; color: rgb(153, 153, 153);">
                                [Image]
                            </div>
                        </div>
                    </div>
                    <div class="cta-section" style="background: rgb(248, 249, 250); padding: 20px; border-radius: 6px; text-align: center;">
                        <h3 style="margin: 0 0 10px 0; color: rgb(51, 51, 51);">Don't miss out!</h3>
                        <button style="background: rgb(102, 126, 234); color: white; border: none; padding: 12px 24px; border-radius: 4px; font-size: 16px; cursor: pointer;">
                            Subscribe Now
                        </button>
                    </div>
                </div>
                <div class="footer-section" style="background: rgb(51, 51, 51); color: rgb(204, 204, 204); padding: 15px; text-align: center; border-radius: 0 0 8px 8px; font-size: 12px;">
                    <p style="margin: 0;">© 2024 Newsletter Co. | Unsubscribe | Privacy Policy</p>
                </div>
            </div>
        "###;
        self.html_body = Some(html.to_string());
        self.body_type = BodyType::Html;
        self
    }

    /// Create an email with potentially dangerous HTML that should be sanitized
    pub fn with_dangerous_html(mut self) -> Self {
        let html = r#"
            <html>
            <head>
                <style>
                    body { position: fixed; top: 0; left: 0; width: 100vw; height: 100vh; z-index: 999999; }
                    .overlay { position: absolute; top: 0; left: 0; width: 100%; height: 100%; background: red; }
                </style>
            </head>
            <body>
                <script>alert('XSS attempt!');</script>
                <div class="overlay">This should not break the main UI!</div>
                <iframe src="javascript:alert('Another XSS attempt!')"></iframe>
                <img src="x" onerror="alert('Image XSS!')">
                <style>
                    .main-app { display: none !important; }
                    * { color: red !important; background: yellow !important; }
                </style>
                <p>This email contains dangerous content that should be sanitized.</p>
            </body>
            </html>
        "#;
        self.html_body = Some(html.to_string());
        self.body_type = BodyType::Html;
        self
    }

    /// Create a newsletter-style multi-column layout
    pub fn with_newsletter_layout(mut self) -> Self {
        let html = format!(
            r#"
            <html>
            <head>
                <style>
                    .newsletter-container {{ max-width: 600px; margin: 0 auto; font-family: 'Helvetica', Arial, sans-serif; }}
                    .header {{ background: #2c3e50; color: white; padding: 20px; text-align: center; }}
                    .two-column {{ display: flex; gap: 20px; padding: 20px; }}
                    .column {{ flex: 1; background: #f8f9fa; padding: 15px; border-radius: 5px; }}
                    .highlight-box {{ background: #e74c3c; color: white; padding: 15px; margin: 20px 0; text-align: center; }}
                    .footer {{ background: #34495e; color: #bdc3c7; padding: 15px; text-align: center; font-size: 12px; }}
                </style>
            </head>
            <body>
                <div class="newsletter-container">
                    <div class="header">
                        <h1 style="margin: 0; font-size: 28px;">Tech Weekly</h1>
                        <p style="margin: 5px 0 0 0;">Issue #42 - January 2024</p>
                    </div>
                    
                    <div class="highlight-box">
                        <h2 style="margin: 0 0 10px 0;">🚀 Breaking: New AI Model Released!</h2>
                        <p style="margin: 0;">Revolutionary capabilities in natural language processing</p>
                    </div>
                    
                    <div class="two-column">
                        <div class="column">
                            <h3 style="color: #2c3e50; margin-top: 0;">Development News</h3>
                            <ul style="line-height: 1.6;">
                                <li>React 19 Beta Released</li>
                                <li>TypeScript 5.3 Features</li>
                                <li>Rust 1.75 Improvements</li>
                            </ul>
                        </div>
                        <div class="column">
                            <h3 style="color: #2c3e50; margin-top: 0;">Industry Updates</h3>
                            <ul style="line-height: 1.6;">
                                <li>Open Source Funding Models</li>
                                <li>Security Best Practices</li>
                                <li>Remote Work Trends</li>
                            </ul>
                        </div>
                    </div>
                    
                    <div style="padding: 20px; background: white; border: 1px solid #dee2e6;">
                        <h3 style="color: #2c3e50;">Featured Tutorial</h3>
                        <p style="line-height: 1.6; color: #495057;">
                            Learn how to build scalable web applications using modern frameworks. 
                            This comprehensive guide covers architecture patterns, performance optimization, 
                            and deployment strategies.
                        </p>
                        <a href="/tutorial" style="display: inline-block; background: #007bff; color: white; padding: 10px 20px; text-decoration: none; border-radius: 3px;">
                            Read Tutorial
                        </a>
                    </div>
                    
                    <div class="footer">
                        <p style="margin: 0;">You received this because you subscribed to Tech Weekly</p>
                        <p style="margin: 5px 0 0 0;">
                            <a href="/unsubscribe" style="color: #bdc3c7;">Unsubscribe</a> | 
                            <a href="/preferences" style="color: #bdc3c7;">Update Preferences</a>
                        </p>
                    </div>
                </div>
            </body>
            </html>
        "#
        );
        self.html_body = Some(html);
        self.body_type = BodyType::Html;
        self
    }

    pub fn to_mbox_entry(&self) -> Vec<u8> {
        let message_id = self
            .message_id
            .clone()
            .unwrap_or_else(|| format!("<{}@example.com>", uuid::Uuid::new_v4()));

        let mut builder = MessageBuilder::new()
            .from(self.from.clone())
            .subject(self.subject.clone())
            .date(self.date.timestamp())
            .message_id(message_id);

        for to in &self.to {
            builder = builder.to(to.clone());
        }

        if let Some(in_reply_to) = &self.in_reply_to {
            builder = builder.in_reply_to(in_reply_to.clone());
        }

        // Handle different body types
        match &self.body_type {
            BodyType::Plain => {
                builder = builder.text_body(self.body.clone());
            }
            BodyType::Html => {
                if let Some(html) = &self.html_body {
                    builder = builder.html_body(html.clone());
                } else {
                    builder = builder.text_body(self.body.clone());
                }
            }
            BodyType::MultipartAlternative { plain, html } => {
                builder = builder.text_body(plain.clone()).html_body(html.clone());
            }
        }

        for attachment in &self.attachments {
            builder = builder.attachment(
                attachment.content_type.clone(),
                attachment.filename.clone(),
                attachment.content.clone(),
            );
        }

        let message = builder.write_to_vec().expect("Failed to build message");

        let mbox_from_line = format!(
            "From {} {}\n",
            self.from.split('@').next().unwrap_or("MAILER-DAEMON"),
            self.date.format("%a %b %e %H:%M:%S %Y")
        );

        let mut result = mbox_from_line.into_bytes();
        result.extend_from_slice(&message);
        if !message.ends_with(b"\n") {
            result.push(b'\n');
        }
        result
    }
}

pub struct MboxBuilder {
    messages: Vec<EmailMessage>,
}

impl MboxBuilder {
    pub fn new() -> Self {
        Self {
            messages: Vec::new(),
        }
    }

    pub fn add_message(mut self, message: EmailMessage) -> Self {
        self.messages.push(message);
        self
    }

    pub fn add_thread(mut self, subject: &str, num_replies: usize) -> Self {
        let thread_id = format!("<thread-{}@example.com>", uuid::Uuid::new_v4());

        let root_message = EmailMessage::new(subject).with_message_id(thread_id.clone());
        self.messages.push(root_message);

        for i in 0..num_replies {
            let reply = EmailMessage::new(format!("Re: {}", subject))
                .with_from(format!("replier{}@example.com", i + 1))
                .with_body(format!("This is reply number {}.", i + 1))
                .with_in_reply_to(thread_id.clone())
                .with_message_id(format!(
                    "<reply-{}-{}@example.com>",
                    i + 1,
                    uuid::Uuid::new_v4()
                ));
            self.messages.push(reply);
        }

        self
    }

    pub fn build(&self) -> Vec<u8> {
        let mut result = Vec::new();

        for message in &self.messages {
            result.extend_from_slice(&message.to_mbox_entry());
        }

        result
    }
}

impl Default for MboxBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_message_to_mbox() {
        let message = EmailMessage::new("Test Subject");
        let mbox_data = message.to_mbox_entry();
        let mbox_str = String::from_utf8_lossy(&mbox_data);

        assert!(mbox_str.starts_with("From sender"));
        assert!(mbox_str.contains("From: <sender@example.com>"));
        assert!(mbox_str.contains("To: <recipient@example.com>"));
        assert!(mbox_str.contains("Subject: Test Subject"));
        assert!(mbox_str.contains("This is a test message."));
    }

    #[test]
    fn test_message_with_attachment() {
        let message = EmailMessage::new("Message with attachment").with_attachment(
            "test.txt",
            "text/plain",
            b"Hello, World!",
        );
        let mbox_data = message.to_mbox_entry();
        let mbox_str = String::from_utf8_lossy(&mbox_data);

        assert!(mbox_str.contains("test.txt"));
        assert!(mbox_str.contains("Content-Type: multipart/mixed"));
    }

    #[test]
    fn test_mbox_builder() {
        let mbox = MboxBuilder::new()
            .add_message(EmailMessage::new("First message"))
            .add_message(EmailMessage::new("Second message"))
            .build();

        let mbox_str = String::from_utf8_lossy(&mbox);
        assert!(mbox_str.contains("Subject: First message"));
        assert!(mbox_str.contains("Subject: Second message"));

        let from_count = mbox_str.matches("From sender").count();
        assert_eq!(from_count, 2);
    }

    #[test]
    fn test_thread_builder() {
        let mbox = MboxBuilder::new().add_thread("Original Thread", 2).build();

        let mbox_str = String::from_utf8_lossy(&mbox);
        assert!(mbox_str.contains("Subject: Original Thread"));
        assert!(mbox_str.contains("Subject: Re: Original Thread"));
        assert!(mbox_str.contains("From: <replier1@example.com>"));
        assert!(mbox_str.contains("From: <replier2@example.com>"));
        assert!(mbox_str.contains("In-Reply-To:"));
    }
}
