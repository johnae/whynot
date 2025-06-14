//! Message composition types for sending email.

use crate::body::BodyPart;
use crate::error::Result;
use chrono::{DateTime, Utc};
use std::collections::HashMap;
use uuid::Uuid;

/// A composable email message for sending.
///
/// This struct represents an email message that can be sent. It includes
/// all necessary headers and body content. Use `MessageBuilder` to construct
/// instances of this type.
#[derive(Debug, Clone)]
pub struct ComposableMessage {
    /// The Message-ID header (generated if not provided).
    pub message_id: String,
    /// The From header (sender email address).
    pub from: Option<String>,
    /// The To header (recipient email addresses).
    pub to: Vec<String>,
    /// The Cc header (carbon copy recipients).
    pub cc: Vec<String>,
    /// The Bcc header (blind carbon copy recipients).
    pub bcc: Vec<String>,
    /// The Subject header.
    pub subject: String,
    /// The In-Reply-To header (for replies).
    pub in_reply_to: Option<String>,
    /// The References header (for threading).
    pub references: Vec<String>,
    /// The Date header.
    pub date: DateTime<Utc>,
    /// Additional headers.
    pub headers: HashMap<String, String>,
    /// The message body (plain text).
    pub body: String,
    /// HTML alternative body.
    pub html_body: Option<String>,
    /// Attachments to include.
    pub attachments: Vec<Attachment>,
}

/// An attachment to be included in an email message.
#[derive(Debug, Clone)]
pub struct Attachment {
    /// The filename for the attachment.
    pub filename: String,
    /// The MIME content type.
    pub content_type: String,
    /// The attachment data.
    pub data: Vec<u8>,
}

impl ComposableMessage {
    /// Create a new message builder.
    pub fn builder() -> MessageBuilder {
        MessageBuilder::new()
    }

    /// Convert this message to RFC 822 format for sending.
    ///
    /// This generates the complete email message including all headers
    /// and properly formatted body with MIME parts if necessary.
    pub fn to_rfc822(&self) -> Result<Vec<u8>> {
        let mut message = String::new();

        // Required headers
        if let Some(from) = &self.from {
            message.push_str(&format!("From: {}\r\n", from));
        }

        if !self.to.is_empty() {
            message.push_str(&format!("To: {}\r\n", self.to.join(", ")));
        }

        if !self.cc.is_empty() {
            message.push_str(&format!("Cc: {}\r\n", self.cc.join(", ")));
        }

        if !self.bcc.is_empty() {
            message.push_str(&format!("Bcc: {}\r\n", self.bcc.join(", ")));
        }

        message.push_str(&format!("Subject: {}\r\n", self.subject));
        message.push_str(&format!("Message-ID: {}\r\n", self.message_id));
        message.push_str(&format!("Date: {}\r\n", self.date.to_rfc2822()));

        // Optional threading headers
        if let Some(in_reply_to) = &self.in_reply_to {
            message.push_str(&format!("In-Reply-To: {}\r\n", in_reply_to));
        }

        if !self.references.is_empty() {
            message.push_str(&format!("References: {}\r\n", self.references.join(" ")));
        }

        // Additional headers
        for (key, value) in &self.headers {
            message.push_str(&format!("{}: {}\r\n", key, value));
        }

        // Determine MIME structure based on content
        let has_html = self.html_body.is_some();
        let has_attachments = !self.attachments.is_empty();

        match (has_html, has_attachments) {
            (false, false) => {
                // Case 1: Plain text only - no multipart needed
                message.push_str("Content-Type: text/plain; charset=utf-8\r\n");
                message.push_str("\r\n");
                message.push_str(&self.body);
            }
            (true, false) => {
                // Case 2: Text + HTML - use multipart/alternative
                let boundary = format!("boundary_{}", Uuid::new_v4());
                message.push_str("MIME-Version: 1.0\r\n");
                message.push_str(&format!(
                    "Content-Type: multipart/alternative; boundary=\"{}\"\r\n",
                    boundary
                ));
                message.push_str("\r\n");

                // Plain text part
                message.push_str(&format!("--{}\r\n", boundary));
                message.push_str("Content-Type: text/plain; charset=utf-8\r\n");
                message.push_str("Content-Transfer-Encoding: quoted-printable\r\n");
                message.push_str("\r\n");
                message.push_str(&self.body);
                message.push_str("\r\n");

                // HTML part
                if let Some(html) = &self.html_body {
                    message.push_str(&format!("--{}\r\n", boundary));
                    message.push_str("Content-Type: text/html; charset=utf-8\r\n");
                    message.push_str("Content-Transfer-Encoding: quoted-printable\r\n");
                    message.push_str("\r\n");
                    message.push_str(html);
                    message.push_str("\r\n");
                }

                message.push_str(&format!("--{}--\r\n", boundary));
            }
            (false, true) => {
                // Case 3: Text + attachments - use multipart/mixed
                let boundary = format!("boundary_{}", Uuid::new_v4());
                message.push_str("MIME-Version: 1.0\r\n");
                message.push_str(&format!(
                    "Content-Type: multipart/mixed; boundary=\"{}\"\r\n",
                    boundary
                ));
                message.push_str("\r\n");

                // Plain text part
                message.push_str(&format!("--{}\r\n", boundary));
                message.push_str("Content-Type: text/plain; charset=utf-8\r\n");
                message.push_str("Content-Transfer-Encoding: quoted-printable\r\n");
                message.push_str("\r\n");
                message.push_str(&self.body);
                message.push_str("\r\n");

                // Attachments
                for attachment in &self.attachments {
                    message.push_str(&format!("--{}\r\n", boundary));
                    message.push_str(&format!("Content-Type: {}\r\n", attachment.content_type));
                    message.push_str(&format!(
                        "Content-Disposition: attachment; filename=\"{}\"\r\n",
                        attachment.filename
                    ));
                    message.push_str("Content-Transfer-Encoding: base64\r\n");
                    message.push_str("\r\n");
                    message.push_str(&base64::Engine::encode(
                        &base64::engine::general_purpose::STANDARD,
                        &attachment.data,
                    ));
                    message.push_str("\r\n");
                }

                message.push_str(&format!("--{}--\r\n", boundary));
            }
            (true, true) => {
                // Case 4: Text + HTML + attachments - nested multipart structure
                let outer_boundary = format!("boundary_{}", Uuid::new_v4());
                let inner_boundary = format!("boundary_{}", Uuid::new_v4());
                
                message.push_str("MIME-Version: 1.0\r\n");
                message.push_str(&format!(
                    "Content-Type: multipart/mixed; boundary=\"{}\"\r\n",
                    outer_boundary
                ));
                message.push_str("\r\n");

                // Nested multipart/alternative for text choices
                message.push_str(&format!("--{}\r\n", outer_boundary));
                message.push_str(&format!(
                    "Content-Type: multipart/alternative; boundary=\"{}\"\r\n",
                    inner_boundary
                ));
                message.push_str("\r\n");

                // Plain text part
                message.push_str(&format!("--{}\r\n", inner_boundary));
                message.push_str("Content-Type: text/plain; charset=utf-8\r\n");
                message.push_str("Content-Transfer-Encoding: quoted-printable\r\n");
                message.push_str("\r\n");
                message.push_str(&self.body);
                message.push_str("\r\n");

                // HTML part
                if let Some(html) = &self.html_body {
                    message.push_str(&format!("--{}\r\n", inner_boundary));
                    message.push_str("Content-Type: text/html; charset=utf-8\r\n");
                    message.push_str("Content-Transfer-Encoding: quoted-printable\r\n");
                    message.push_str("\r\n");
                    message.push_str(html);
                    message.push_str("\r\n");
                }

                // Close inner multipart/alternative
                message.push_str(&format!("--{}--\r\n", inner_boundary));

                // Attachments at outer level
                for attachment in &self.attachments {
                    message.push_str(&format!("--{}\r\n", outer_boundary));
                    message.push_str(&format!("Content-Type: {}\r\n", attachment.content_type));
                    message.push_str(&format!(
                        "Content-Disposition: attachment; filename=\"{}\"\r\n",
                        attachment.filename
                    ));
                    message.push_str("Content-Transfer-Encoding: base64\r\n");
                    message.push_str("\r\n");
                    message.push_str(&base64::Engine::encode(
                        &base64::engine::general_purpose::STANDARD,
                        &attachment.data,
                    ));
                    message.push_str("\r\n");
                }

                // Close outer multipart/mixed
                message.push_str(&format!("--{}--\r\n", outer_boundary));
            }
        }

        Ok(message.into_bytes())
    }

    /// Create a reply builder from an original message.
    ///
    /// This sets up proper headers for replying including In-Reply-To
    /// and References headers, and quotes the original message body.
    pub fn reply_builder(original: &crate::thread::Message, reply_all: bool) -> MessageBuilder {
        let mut builder = MessageBuilder::new();

        // Set In-Reply-To to the original message ID
        builder = builder.in_reply_to(original.id.clone());

        // Build References header
        let mut references = vec![];

        // Check if original has References header
        if let Some(orig_refs) = original.headers.get("references") {
            // Parse space-separated message IDs
            references.extend(orig_refs.split_whitespace().map(|s| s.to_string()));
        }

        // Add the original message ID to references
        references.push(original.id.clone());

        for reference in references {
            builder = builder.add_reference(reference);
        }

        // Set subject with Re: prefix if not already present
        let subject = match &original.headers.subject {
            Some(subj) if subj.starts_with("Re: ") => subj.clone(),
            Some(subj) => format!("Re: {}", subj),
            None => "Re: (No subject)".to_string(),
        };
        builder = builder.subject(subject);

        // Set To field
        if reply_all {
            // Reply to sender
            builder = builder.to(original.headers.from.clone());

            // Add original To recipients (except ourselves if we can determine that)
            if let Some(to) = &original.headers.to {
                for addr in to.split(',').map(|s| s.trim()) {
                    builder = builder.to(addr.to_string());
                }
            }

            // Add original Cc recipients
            if let Some(cc_header) = original.headers.get("cc") {
                for addr in cc_header.split(',').map(|s| s.trim()) {
                    builder = builder.cc(addr.to_string());
                }
            }
        } else {
            // Just reply to sender
            builder = builder.to(original.headers.from.clone());
        }

        // Quote original message
        let quoted_body = quote_message_body(original);
        builder = builder.body(quoted_body);

        builder
    }

    /// Create a forward builder from an original message.
    ///
    /// This sets up the message for forwarding, including the original
    /// message content.
    pub fn forward_builder(original: &crate::thread::Message) -> MessageBuilder {
        let mut builder = MessageBuilder::new();

        // Set subject with Fwd: prefix if not already present
        let subject = match &original.headers.subject {
            Some(subj) if subj.starts_with("Fwd: ") => subj.clone(),
            Some(subj) => format!("Fwd: {}", subj),
            None => "Fwd: (No subject)".to_string(),
        };
        builder = builder.subject(subject);

        // Include original message info in body
        let forward_body = format_forward_body(original);
        builder = builder.body(forward_body);

        // TODO: Handle attachments from original message

        builder
    }
}

/// Builder for constructing ComposableMessage instances.
#[derive(Debug, Default)]
pub struct MessageBuilder {
    message_id: Option<String>,
    from: Option<String>,
    to: Vec<String>,
    cc: Vec<String>,
    bcc: Vec<String>,
    subject: Option<String>,
    in_reply_to: Option<String>,
    references: Vec<String>,
    date: Option<DateTime<Utc>>,
    headers: HashMap<String, String>,
    body: Option<String>,
    html_body: Option<String>,
    attachments: Vec<Attachment>,
}

impl MessageBuilder {
    /// Create a new message builder.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the Message-ID (auto-generated if not set).
    pub fn message_id(mut self, id: String) -> Self {
        self.message_id = Some(id);
        self
    }

    /// Set the From address.
    pub fn from(mut self, from: String) -> Self {
        self.from = Some(from);
        self
    }

    /// Add a To recipient.
    pub fn to(mut self, to: String) -> Self {
        self.to.push(to);
        self
    }

    /// Add a Cc recipient.
    pub fn cc(mut self, cc: String) -> Self {
        self.cc.push(cc);
        self
    }

    /// Add a Bcc recipient.
    pub fn bcc(mut self, bcc: String) -> Self {
        self.bcc.push(bcc);
        self
    }

    /// Set the subject.
    pub fn subject(mut self, subject: String) -> Self {
        self.subject = Some(subject);
        self
    }

    /// Set the In-Reply-To header.
    pub fn in_reply_to(mut self, id: String) -> Self {
        self.in_reply_to = Some(id);
        self
    }

    /// Add a reference to the References header.
    pub fn add_reference(mut self, reference: String) -> Self {
        self.references.push(reference);
        self
    }

    /// Set the date (defaults to now).
    pub fn date(mut self, date: DateTime<Utc>) -> Self {
        self.date = Some(date);
        self
    }

    /// Add a custom header.
    pub fn header(mut self, key: String, value: String) -> Self {
        self.headers.insert(key, value);
        self
    }

    /// Set the plain text body.
    pub fn body(mut self, body: String) -> Self {
        self.body = Some(body);
        self
    }

    /// Set the HTML body.
    pub fn html_body(mut self, html: String) -> Self {
        self.html_body = Some(html);
        self
    }

    /// Add an attachment.
    pub fn attachment(mut self, attachment: Attachment) -> Self {
        self.attachments.push(attachment);
        self
    }

    /// Build the ComposableMessage.
    ///
    /// Returns an error if required fields are missing.
    pub fn build(self) -> Result<ComposableMessage> {
        // Generate Message-ID if not provided
        let message_id = self
            .message_id
            .unwrap_or_else(|| format!("<{}@whynot>", Uuid::new_v4()));

        // Ensure we have required fields
        if self.to.is_empty() && self.cc.is_empty() && self.bcc.is_empty() {
            return Err(crate::error::Error::InvalidInput(
                "Message must have at least one recipient".to_string(),
            ));
        }

        let subject = self.subject.unwrap_or_default();
        let body = self.body.unwrap_or_default();
        let date = self.date.unwrap_or_else(Utc::now);

        Ok(ComposableMessage {
            message_id,
            from: self.from,
            to: self.to,
            cc: self.cc,
            bcc: self.bcc,
            subject,
            in_reply_to: self.in_reply_to,
            references: self.references,
            date,
            headers: self.headers,
            body,
            html_body: self.html_body,
            attachments: self.attachments,
        })
    }
}

/// Quote the body of a message for replying.
fn quote_message_body(message: &crate::thread::Message) -> String {
    let mut quoted = String::new();

    // Add attribution line
    quoted.push_str(&format!(
        "On {}, {} wrote:\n",
        message.date_relative, message.headers.from
    ));

    // Extract plain text body
    let body_text = extract_plain_text_body(message);

    // Quote each line
    for line in body_text.lines() {
        quoted.push_str("> ");
        quoted.push_str(line);
        quoted.push('\n');
    }

    quoted
}

/// Format a message for forwarding.
fn format_forward_body(message: &crate::thread::Message) -> String {
    let mut forward = String::new();

    forward.push_str("---------- Forwarded message ----------\n");
    forward.push_str(&format!("From: {}\n", message.headers.from));
    forward.push_str(&format!("Date: {}\n", message.date_relative));
    forward.push_str(&format!(
        "Subject: {}\n",
        message.headers.subject.as_deref().unwrap_or("(No subject)")
    ));

    if let Some(to) = &message.headers.to {
        forward.push_str(&format!("To: {}\n", to));
    }

    forward.push('\n');

    // Include original body
    let body_text = extract_plain_text_body(message);
    forward.push_str(&body_text);

    forward
}

/// Extract plain text body from a message.
fn extract_plain_text_body(message: &crate::thread::Message) -> String {
    for part in &message.body {
        if let Some(text) = extract_text_from_part(part) {
            return text;
        }
    }
    String::new()
}

/// Recursively extract text from a body part.
fn extract_text_from_part(part: &BodyPart) -> Option<String> {
    use crate::body::BodyContent;

    match &part.content {
        BodyContent::Text(text) if part.content_type == "text/plain" => Some(text.clone()),
        BodyContent::Multipart(parts) => {
            for subpart in parts {
                if let Some(text) = extract_text_from_part(subpart) {
                    return Some(text);
                }
            }
            None
        }
        _ => None,
    }
}
