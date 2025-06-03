use serde::{Deserialize, Deserializer, Serialize};
use serde_json;

/// Represents a body part of an email (text, HTML, attachment, etc.)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BodyPart {
    pub id: u32,
    #[serde(rename = "content-type")]
    pub content_type: String,
    #[serde(default, deserialize_with = "deserialize_body_content")]
    pub content: BodyContent,
    #[serde(rename = "content-disposition")]
    pub content_disposition: Option<String>,
    #[serde(rename = "content-id")]
    pub content_id: Option<String>,
    pub filename: Option<String>,
    #[serde(rename = "content-transfer-encoding")]
    pub content_transfer_encoding: Option<String>,
    #[serde(rename = "content-length")]
    pub content_length: Option<u64>,
}

impl BodyPart {
    /// Check if this body part represents an attachment
    pub fn is_attachment(&self) -> bool {
        if let Some(disposition) = &self.content_disposition {
            disposition == "attachment" || disposition == "inline"
        } else {
            false
        }
    }
}

/// Content of a body part - either text, multipart container, or empty
#[derive(Debug, Clone, Serialize, PartialEq, Default)]
pub enum BodyContent {
    Text(String),
    Multipart(Vec<BodyPart>),
    #[default]
    Empty,
}

impl<'de> Deserialize<'de> for BodyContent {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = serde_json::Value::deserialize(deserializer)?;
        match value {
            serde_json::Value::String(s) => Ok(BodyContent::Text(s)),
            serde_json::Value::Array(arr) => {
                let parts: Vec<BodyPart> = serde_json::from_value(serde_json::Value::Array(arr))
                    .map_err(serde::de::Error::custom)?;
                Ok(BodyContent::Multipart(parts))
            }
            _ => Ok(BodyContent::Empty),
        }
    }
}

fn deserialize_body_content<'de, D>(deserializer: D) -> Result<BodyContent, D::Error>
where
    D: Deserializer<'de>,
{
    let opt: Option<serde_json::Value> = Option::deserialize(deserializer)?;
    match opt {
        Some(value) => serde_json::from_value(value).map_err(serde::de::Error::custom),
        None => Ok(BodyContent::Empty),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_deserialize_body_part_text() {
        let json_data = r#"{
            "id": 1,
            "content-type": "text/html",
            "content": "<html>Test content</html>"
        }"#;

        let body_part: BodyPart = serde_json::from_str(json_data).unwrap();

        assert_eq!(body_part.id, 1);
        assert_eq!(body_part.content_type, "text/html");
        assert!(
            matches!(body_part.content, BodyContent::Text(ref s) if s == "<html>Test content</html>")
        );
    }

    #[test]
    fn test_deserialize_body_part_multipart() {
        let json_data = r#"{
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
        }"#;

        let body_part: BodyPart = serde_json::from_str(json_data).unwrap();

        assert_eq!(body_part.id, 1);
        assert_eq!(body_part.content_type, "multipart/mixed");

        if let BodyContent::Multipart(parts) = &body_part.content {
            assert_eq!(parts.len(), 2);
            assert_eq!(parts[0].id, 2);
            assert_eq!(parts[1].id, 3);
            assert_eq!(parts[1].filename, Some("calendar.ics".to_string()));
        } else {
            panic!("Expected multipart content");
        }
    }

    #[test]
    fn test_deserialize_body_part_attachment() {
        let json_data = r#"{
            "id": 4,
            "content-type": "image/png",
            "content-disposition": "attachment",
            "content-id": "image001.png@01DBCFC2.879537F0",
            "filename": "image001.png",
            "content-transfer-encoding": "base64",
            "content-length": 61851
        }"#;

        let body_part: BodyPart = serde_json::from_str(json_data).unwrap();

        assert_eq!(body_part.id, 4);
        assert_eq!(body_part.content_type, "image/png");
        assert_eq!(
            body_part.content_disposition,
            Some("attachment".to_string())
        );
        assert_eq!(body_part.filename, Some("image001.png".to_string()));
        assert_eq!(
            body_part.content_transfer_encoding,
            Some("base64".to_string())
        );
        assert_eq!(body_part.content_length, Some(61851));
        assert!(matches!(body_part.content, BodyContent::Empty));
    }

    #[test]
    fn test_is_attachment() {
        // Test attachment with content-disposition
        let attachment = BodyPart {
            id: 1,
            content_type: "image/png".to_string(),
            content: BodyContent::Empty,
            content_disposition: Some("attachment".to_string()),
            content_id: None,
            filename: Some("image.png".to_string()),
            content_transfer_encoding: None,
            content_length: None,
        };
        assert!(attachment.is_attachment());

        // Test inline attachment
        let inline = BodyPart {
            id: 2,
            content_type: "image/png".to_string(),
            content: BodyContent::Empty,
            content_disposition: Some("inline".to_string()),
            content_id: None,
            filename: Some("image.png".to_string()),
            content_transfer_encoding: None,
            content_length: None,
        };
        assert!(inline.is_attachment());

        // Test regular text part
        let text = BodyPart {
            id: 3,
            content_type: "text/plain".to_string(),
            content: BodyContent::Text("Hello".to_string()),
            content_disposition: None,
            content_id: None,
            filename: None,
            content_transfer_encoding: None,
            content_length: None,
        };
        assert!(!text.is_attachment());

        // Test multipart container
        let multipart = BodyPart {
            id: 4,
            content_type: "multipart/mixed".to_string(),
            content: BodyContent::Multipart(vec![]),
            content_disposition: None,
            content_id: None,
            filename: None,
            content_transfer_encoding: None,
            content_length: None,
        };
        assert!(!multipart.is_attachment());
    }
}
