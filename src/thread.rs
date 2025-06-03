use crate::body::{BodyContent, BodyPart};
use crate::common::{CryptoInfo, Headers};
use serde::{Deserialize, Deserializer, Serialize};

/// Represents an email thread containing nested message levels
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Thread(pub Vec<ThreadLevel>);

impl Thread {
    /// Flatten the thread structure into a list of messages
    pub fn get_messages(&self) -> Vec<&Message> {
        let mut messages = Vec::new();
        for level in &self.0 {
            Self::collect_messages_from_level(level, &mut messages);
        }
        messages
    }

    fn collect_messages_from_level<'a>(level: &'a ThreadLevel, messages: &mut Vec<&'a Message>) {
        for node in &level.0 {
            messages.push(&node.0);
            for child_level in &node.1 {
                Self::collect_messages_from_level(child_level, messages);
            }
        }
    }
}

/// A level in the thread hierarchy containing message nodes
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ThreadLevel(pub Vec<MessageNode>);

/// A node in the thread tree containing a message and its replies
#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct MessageNode(pub Message, pub Vec<ThreadLevel>);

impl<'de> Deserialize<'de> for MessageNode {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = serde_json::Value::deserialize(deserializer)?;
        match value {
            serde_json::Value::Array(mut arr) => {
                if arr.len() >= 2 {
                    let message: Message =
                        serde_json::from_value(arr.remove(0)).map_err(serde::de::Error::custom)?;
                    let children: Vec<ThreadLevel> =
                        serde_json::from_value(serde_json::Value::Array(arr))
                            .map_err(serde::de::Error::custom)?;
                    Ok(MessageNode(message, children))
                } else {
                    Err(serde::de::Error::custom(
                        "Expected array with at least 2 elements",
                    ))
                }
            }
            serde_json::Value::Object(_) => {
                // Single message without children
                let message: Message =
                    serde_json::from_value(value).map_err(serde::de::Error::custom)?;
                Ok(MessageNode(message, vec![]))
            }
            _ => Err(serde::de::Error::custom("Expected array or object")),
        }
    }
}

/// Individual email message with all metadata and content
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Message {
    pub id: String,
    #[serde(rename = "match")]
    pub is_match: bool,
    pub excluded: bool,
    pub filename: Vec<String>,
    pub timestamp: i64,
    pub date_relative: String,
    pub tags: Vec<String>,
    pub duplicate: Option<u32>,
    pub body: Vec<BodyPart>,
    pub crypto: CryptoInfo,
    pub headers: Headers,
}

impl Message {
    /// Check if this message contains any attachments
    pub fn has_attachments(&self) -> bool {
        self.body.iter().any(|part| {
            part.is_attachment()
                || match &part.content {
                    BodyContent::Multipart(parts) => parts.iter().any(|p| p.is_attachment()),
                    _ => false,
                }
        })
    }

    /// Get all attachment body parts from this message
    pub fn get_attachments(&self) -> Vec<&BodyPart> {
        let mut attachments = Vec::new();
        for part in &self.body {
            if part.is_attachment() {
                attachments.push(part);
            }
            if let BodyContent::Multipart(parts) = &part.content {
                for subpart in parts {
                    if subpart.is_attachment() {
                        attachments.push(subpart);
                    }
                }
            }
        }
        attachments
    }

    /// Get the primary text content of the message
    /// Returns the first text/plain or text/html content found
    pub fn get_text_content(&self) -> Option<&str> {
        Self::find_text_content(&self.body)
    }

    fn find_text_content(parts: &[BodyPart]) -> Option<&str> {
        // First look for text/plain
        for part in parts {
            if part.content_type.starts_with("text/plain") {
                if let BodyContent::Text(ref content) = part.content {
                    return Some(content);
                }
            }
        }

        // Then look for text/html if no plain text found
        for part in parts {
            if part.content_type.starts_with("text/html") {
                if let BodyContent::Text(ref content) = part.content {
                    return Some(content);
                }
            }
        }

        // Recursively search in multipart content
        for part in parts {
            if let BodyContent::Multipart(ref subparts) = part.content {
                if let Some(content) = Self::find_text_content(subparts) {
                    return Some(content);
                }
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::body::BodyContent;
    use crate::common::Headers;
    use serde_json;

    #[test]
    fn test_deserialize_thread() {
        let json_data = r#"[
  [
    [
      {
        "id": "a43c8c7576ec4d2db2148b526f6be21d@financecompany.example",
        "match": true,
        "excluded": false,
        "filename": [
          "/home/user/Mail/archive/All Mail/cur/1748424311.690968_1.icarus,U=182875:2,RS"
        ],
        "timestamp": 1748424035,
        "date_relative": "Wed. 11:20",
        "tags": [
          "Important",
          "attachment",
          "inbox",
          "unread"
        ],
        "duplicate": 1,
        "body": [
          {
            "id": 1,
            "content-type": "multipart/mixed",
            "content": [
              {
                "id": 2,
                "content-type": "text/html",
                "content": "<html>Test content</html>"
              },
              {
                "id": 3,
                "content-type": "text/calendar",
                "content-disposition": "attachment",
                "filename": "calendar.ics",
                "content": "BEGIN:VCALENDAR\nEND:VCALENDAR\n"
              }
            ]
          }
        ],
        "crypto": {},
        "headers": {
          "Subject": "Quarterly Review Meeting, Finance Company",
          "From": "\"Bob Wilson\" <bob@financecompany.example>",
          "To": "\"alice@techcorp.example\" <alice@techcorp.example>",
          "Reply-To": "Bob Wilson <bob@financecompany.example>",
          "Date": "Wed, 28 May 2025 09:20:35 +0000"
        }
      },
      [
        [
          {
            "id": "reply123@techcorp.example",
            "match": true,
            "excluded": false,
            "filename": [
              "/home/user/Mail/archive/All Mail/cur/1748425511.691716_1.icarus,U=182878:2,S"
            ],
            "timestamp": 1748425489,
            "date_relative": "Wed. 11:44",
            "tags": [
              "Important",
              "attachment",
              "inbox",
              "unread"
            ],
            "duplicate": 1,
            "body": [
              {
                "id": 1,
                "content-type": "text/html",
                "content": "<p dir=\"ltr\">Hej Bob,</p>"
              }
            ],
            "crypto": {},
            "headers": {
              "Subject": "Re: Quarterly Review Meeting, Finance Company",
              "From": "\"Alice Thompson\" <alice@techcorp.example>",
              "To": "\"bob@financecompany.example\" <bob@financecompany.example>",
              "Reply-To": "Alice Thompson <alice@techcorp.example>",
              "Date": "Wed, 28 May 2025 09:44:49 +0000"
            }
          },
          []
        ]
      ]
    ]
  ]
]"#;

        let thread: Thread = serde_json::from_str(json_data).unwrap();

        // Thread is a nested array structure
        assert_eq!(thread.0.len(), 1);

        let first_level = &thread.0[0];
        assert_eq!(first_level.0.len(), 1);

        let message_node = &first_level.0[0];
        let message = &message_node.0;

        assert_eq!(message.id, "a43c8c7576ec4d2db2148b526f6be21d@financecompany.example");
        assert_eq!(message.tags.len(), 4);
        assert_eq!(message.body.len(), 1);

        let body_part = &message.body[0];
        assert_eq!(body_part.id, 1);
        assert_eq!(body_part.content_type, "multipart/mixed");

        // Check nested structure
        let replies = &message_node.1;
        assert_eq!(replies.len(), 1);
    }

    #[test]
    fn test_message_has_attachments() {
        let message_with_attachment = Message {
            id: "test@example.com".to_string(),
            is_match: true,
            excluded: false,
            filename: vec![],
            timestamp: 1234567890,
            date_relative: "Today".to_string(),
            tags: vec![],
            duplicate: Some(1),
            body: vec![BodyPart {
                id: 1,
                content_type: "multipart/mixed".to_string(),
                content: BodyContent::Multipart(vec![
                    BodyPart {
                        id: 2,
                        content_type: "text/plain".to_string(),
                        content: BodyContent::Text("Hello".to_string()),
                        content_disposition: None,
                        content_id: None,
                        filename: None,
                        content_transfer_encoding: None,
                        content_length: None,
                    },
                    BodyPart {
                        id: 3,
                        content_type: "image/png".to_string(),
                        content: BodyContent::Empty,
                        content_disposition: Some("attachment".to_string()),
                        content_id: None,
                        filename: Some("image.png".to_string()),
                        content_transfer_encoding: Some("base64".to_string()),
                        content_length: Some(1000),
                    },
                ]),
                content_disposition: None,
                content_id: None,
                filename: None,
                content_transfer_encoding: None,
                content_length: None,
            }],
            crypto: CryptoInfo::default(),
            headers: Headers {
                subject: "Test Subject".to_string(),
                from: "sender@example.com".to_string(),
                to: "recipient@example.com".to_string(),
                reply_to: None,
                date: "Mon, 1 Jan 2024 12:00:00 +0000".to_string(),
            },
        };

        assert!(message_with_attachment.has_attachments());

        let attachments = message_with_attachment.get_attachments();
        assert_eq!(attachments.len(), 1);
        assert_eq!(attachments[0].filename, Some("image.png".to_string()));
    }

    #[test]
    fn test_message_no_attachments() {
        let message_without_attachment = Message {
            id: "test@example.com".to_string(),
            is_match: true,
            excluded: false,
            filename: vec![],
            timestamp: 1234567890,
            date_relative: "Today".to_string(),
            tags: vec![],
            duplicate: Some(1),
            body: vec![BodyPart {
                id: 1,
                content_type: "text/plain".to_string(),
                content: BodyContent::Text("Hello".to_string()),
                content_disposition: None,
                content_id: None,
                filename: None,
                content_transfer_encoding: None,
                content_length: None,
            }],
            crypto: CryptoInfo::default(),
            headers: Headers {
                subject: "Test Subject".to_string(),
                from: "sender@example.com".to_string(),
                to: "recipient@example.com".to_string(),
                reply_to: None,
                date: "Mon, 1 Jan 2024 12:00:00 +0000".to_string(),
            },
        };

        assert!(!message_without_attachment.has_attachments());
        assert_eq!(message_without_attachment.get_attachments().len(), 0);
    }

    #[test]
    fn test_thread_get_messages() {
        // Create a simple thread structure with 3 messages
        let msg1 = Message {
            id: "msg1@example.com".to_string(),
            is_match: true,
            excluded: false,
            filename: vec![],
            timestamp: 1234567890,
            date_relative: "Today".to_string(),
            tags: vec![],
            duplicate: Some(1),
            body: vec![],
            crypto: CryptoInfo::default(),
            headers: Headers {
                subject: "Test Subject".to_string(),
                from: "sender@example.com".to_string(),
                to: "recipient@example.com".to_string(),
                reply_to: None,
                date: "Mon, 1 Jan 2024 12:00:00 +0000".to_string(),
            },
        };

        let msg2 = Message {
            id: "msg2@example.com".to_string(),
            is_match: true,
            excluded: false,
            filename: vec![],
            timestamp: 1234567891,
            date_relative: "Today".to_string(),
            tags: vec![],
            duplicate: Some(1),
            body: vec![],
            crypto: CryptoInfo::default(),
            headers: Headers {
                subject: "Test Subject".to_string(),
                from: "sender@example.com".to_string(),
                to: "recipient@example.com".to_string(),
                reply_to: None,
                date: "Mon, 1 Jan 2024 12:00:00 +0000".to_string(),
            },
        };

        let msg3 = Message {
            id: "msg3@example.com".to_string(),
            is_match: true,
            excluded: false,
            filename: vec![],
            timestamp: 1234567892,
            date_relative: "Today".to_string(),
            tags: vec![],
            duplicate: Some(1),
            body: vec![],
            crypto: CryptoInfo::default(),
            headers: Headers {
                subject: "Test Subject".to_string(),
                from: "sender@example.com".to_string(),
                to: "recipient@example.com".to_string(),
                reply_to: None,
                date: "Mon, 1 Jan 2024 12:00:00 +0000".to_string(),
            },
        };

        // Create nested thread structure: msg1 -> msg2 -> msg3
        let thread = Thread(vec![ThreadLevel(vec![MessageNode(
            msg1.clone(),
            vec![ThreadLevel(vec![MessageNode(
                msg2.clone(),
                vec![ThreadLevel(vec![MessageNode(msg3.clone(), vec![])])],
            )])],
        )])]);

        let messages = thread.get_messages();
        assert_eq!(messages.len(), 3);
        assert_eq!(messages[0].id, "msg1@example.com");
        assert_eq!(messages[1].id, "msg2@example.com");
        assert_eq!(messages[2].id, "msg3@example.com");
    }
}
