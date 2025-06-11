use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Placeholder for cryptographic information (currently empty)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct CryptoInfo {
    #[serde(flatten)]
    pub fields: HashMap<String, serde_json::Value>,
}

/// Email headers containing standard fields
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Headers {
    #[serde(rename = "Subject")]
    pub subject: Option<String>,
    #[serde(rename = "From")]
    pub from: String,
    #[serde(rename = "To")]
    pub to: Option<String>,
    #[serde(rename = "Reply-To")]
    pub reply_to: Option<String>,
    #[serde(rename = "Date")]
    pub date: String,
    #[serde(flatten)]
    pub additional: HashMap<String, String>,
}

impl Headers {
    /// Get a header value by name (case-insensitive).
    pub fn get(&self, key: &str) -> Option<&String> {
        let key_lower = key.to_lowercase();
        match key_lower.as_str() {
            "subject" => self.subject.as_ref(),
            "from" => Some(&self.from),
            "to" => self.to.as_ref(),
            "reply-to" => self.reply_to.as_ref(),
            "date" => Some(&self.date),
            _ => self.additional.get(&key_lower),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_deserialize_headers() {
        let json_data = r#"{
            "Subject": "Quarterly Review Meeting, Finance Company",
            "From": "\"Bob Wilson\" <bob@financecompany.example>",
            "To": "\"alice@techcorp.example\" <alice@techcorp.example>",
            "Reply-To": "Bob Wilson <bob@financecompany.example>",
            "Date": "Wed, 28 May 2025 09:20:35 +0000"
        }"#;

        let headers: Headers = serde_json::from_str(json_data).unwrap();

        assert_eq!(
            headers.subject,
            Some("Quarterly Review Meeting, Finance Company".to_string())
        );
        assert_eq!(headers.from, "\"Bob Wilson\" <bob@financecompany.example>");
        assert_eq!(
            headers.to,
            Some("\"alice@techcorp.example\" <alice@techcorp.example>".to_string())
        );
        assert_eq!(
            headers.reply_to,
            Some("Bob Wilson <bob@financecompany.example>".to_string())
        );
        assert_eq!(headers.date, "Wed, 28 May 2025 09:20:35 +0000");
    }

    #[test]
    fn test_deserialize_headers_without_reply_to() {
        let json_data = r#"{
            "Subject": "Test Email",
            "From": "sender@example.com",
            "To": "recipient@example.com",
            "Date": "Mon, 1 Jan 2024 12:00:00 +0000"
        }"#;

        let headers: Headers = serde_json::from_str(json_data).unwrap();

        assert_eq!(headers.subject, Some("Test Email".to_string()));
        assert_eq!(headers.from, "sender@example.com");
        assert_eq!(headers.to, Some("recipient@example.com".to_string()));
        assert_eq!(headers.reply_to, None);
        assert_eq!(headers.date, "Mon, 1 Jan 2024 12:00:00 +0000");
    }

    #[test]
    fn test_deserialize_headers_without_to() {
        let json_data = r#"{
            "Subject": "Infobrev juni 2025",
            "From": "\"rektor@grindstugan.se\" <rektor@grindstugan.se>",
            "Date": "Wed, 11 Jun 2025 14:00:35 +0000"
        }"#;

        let headers: Headers = serde_json::from_str(json_data).unwrap();

        assert_eq!(headers.subject, Some("Infobrev juni 2025".to_string()));
        assert_eq!(
            headers.from,
            "\"rektor@grindstugan.se\" <rektor@grindstugan.se>"
        );
        assert_eq!(headers.to, None);
        assert_eq!(headers.reply_to, None);
        assert_eq!(headers.date, "Wed, 11 Jun 2025 14:00:35 +0000");
    }

    #[test]
    fn test_deserialize_headers_minimal() {
        // Only From and Date are required per RFC 5322
        let json_data = r#"{
            "From": "sender@example.com",
            "Date": "Mon, 1 Jan 2024 12:00:00 +0000"
        }"#;

        let headers: Headers = serde_json::from_str(json_data).unwrap();

        assert_eq!(headers.subject, None);
        assert_eq!(headers.from, "sender@example.com");
        assert_eq!(headers.to, None);
        assert_eq!(headers.reply_to, None);
        assert_eq!(headers.date, "Mon, 1 Jan 2024 12:00:00 +0000");
    }
}
