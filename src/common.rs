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
    pub subject: String,
    #[serde(rename = "From")]
    pub from: String,
    #[serde(rename = "To")]
    pub to: String,
    #[serde(rename = "Reply-To")]
    pub reply_to: Option<String>,
    #[serde(rename = "Date")]
    pub date: String,
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
            "Quarterly Review Meeting, Finance Company"
        );
        assert_eq!(headers.from, "\"Bob Wilson\" <bob@financecompany.example>");
        assert_eq!(headers.to, "\"alice@techcorp.example\" <alice@techcorp.example>");
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

        assert_eq!(headers.subject, "Test Email");
        assert_eq!(headers.from, "sender@example.com");
        assert_eq!(headers.to, "recipient@example.com");
        assert_eq!(headers.reply_to, None);
        assert_eq!(headers.date, "Mon, 1 Jan 2024 12:00:00 +0000");
    }
}
