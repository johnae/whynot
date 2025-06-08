use whynot::mail_sender::{MailSender, ComposableMessage, MailSenderConfig, create_mail_sender};
use whynot::thread::Message;
use whynot::error::Result;
use tokio::test;

#[test]
async fn test_create_mail_sender_local() {
    let config = MailSenderConfig::Local {
        msmtp_path: None,
        config_path: None,
    };
    
    let result = create_mail_sender(config);
    assert!(result.is_ok());
    let _sender = result.unwrap();
}

#[test]
async fn test_create_mail_sender_remote() {
    let config = MailSenderConfig::Remote {
        host: "mail.example.com".to_string(),
        user: Some("testuser".to_string()),
        port: None,
        identity_file: None,
        msmtp_path: None,
        config_path: None,
    };
    
    let result = create_mail_sender(config);
    assert!(result.is_ok());
    let _sender = result.unwrap();
}

#[test]
async fn test_composable_message_builder() {
    let message = ComposableMessage::builder()
        .to("recipient@example.com".to_string())
        .from("sender@example.com".to_string())
        .subject("Test Subject".to_string())
        .body("Test body content".to_string())
        .build();
    
    assert!(message.is_ok());
    let message = message.unwrap();
    assert_eq!(message.to, vec!["recipient@example.com"]);
    assert_eq!(message.from, Some("sender@example.com".to_string()));
    assert_eq!(message.subject, "Test Subject");
    assert_eq!(message.body, "Test body content");
}

#[test]
async fn test_message_builder_requires_recipient() {
    let message = ComposableMessage::builder()
        .from("sender@example.com".to_string())
        .subject("Test Subject".to_string())
        .body("Test body content".to_string())
        .build();
    
    assert!(message.is_err());
}

#[test]
async fn test_message_to_rfc822() {
    let message = ComposableMessage::builder()
        .to("recipient@example.com".to_string())
        .from("sender@example.com".to_string())
        .subject("Test Subject".to_string())
        .body("Test body content".to_string())
        .build()
        .unwrap();
    
    let rfc822 = message.to_rfc822().unwrap();
    let content = String::from_utf8(rfc822).unwrap();
    
    assert!(content.contains("From: sender@example.com\r\n"));
    assert!(content.contains("To: recipient@example.com\r\n"));
    assert!(content.contains("Subject: Test Subject\r\n"));
    assert!(content.contains("Test body content"));
}

#[test]
async fn test_message_with_multiple_recipients() {
    let message = ComposableMessage::builder()
        .to("recipient1@example.com".to_string())
        .to("recipient2@example.com".to_string())
        .cc("cc@example.com".to_string())
        .bcc("bcc@example.com".to_string())
        .from("sender@example.com".to_string())
        .subject("Test Subject".to_string())
        .body("Test body content".to_string())
        .build()
        .unwrap();
    
    let rfc822 = message.to_rfc822().unwrap();
    let content = String::from_utf8(rfc822).unwrap();
    
    assert!(content.contains("To: recipient1@example.com, recipient2@example.com\r\n"));
    assert!(content.contains("Cc: cc@example.com\r\n"));
    assert!(content.contains("Bcc: bcc@example.com\r\n"));
}

#[test]
async fn test_reply_builder() {
    use whynot::common::Headers;
    use whynot::body::BodyPart;
    use std::collections::HashMap;
    
    // Create a mock original message
    let mut headers = Headers {
        subject: "Original Subject".to_string(),
        from: "original@example.com".to_string(),
        to: "recipient@example.com".to_string(),
        reply_to: None,
        date: "Mon, 1 Jan 2024 12:00:00 +0000".to_string(),
        additional: HashMap::new(),
    };
    headers.additional.insert("references".to_string(), "<ref1@example.com> <ref2@example.com>".to_string());
    
    let original = Message {
        id: "<original@example.com>".to_string(),
        is_match: false,
        excluded: false,
        filename: vec![],
        timestamp: 1704110400,
        date_relative: "2024-01-01".to_string(),
        tags: vec![],
        duplicate: None,
        body: vec![BodyPart {
            id: 1,
            content_type: "text/plain".to_string(),
            content: whynot::body::BodyContent::Text("Original message body".to_string()),
            filename: None,
            content_id: None,
            content_length: None,
            content_disposition: None,
            content_transfer_encoding: None,
        }],
        crypto: Default::default(),
        headers,
    };
    
    let reply = ComposableMessage::reply_builder(&original, false)
        .build()
        .unwrap();
    
    assert_eq!(reply.subject, "Re: Original Subject");
    assert_eq!(reply.to, vec!["original@example.com"]);
    assert_eq!(reply.in_reply_to, Some("<original@example.com>".to_string()));
    assert_eq!(reply.references.len(), 3);
    assert!(reply.body.contains("On 2024-01-01, original@example.com wrote:"));
    assert!(reply.body.contains("> Original message body"));
}

#[test]
async fn test_forward_builder() {
    use whynot::common::Headers;
    use whynot::body::BodyPart;
    use std::collections::HashMap;
    
    // Create a mock original message
    let headers = Headers {
        subject: "Original Subject".to_string(),
        from: "original@example.com".to_string(),
        to: "recipient@example.com".to_string(),
        reply_to: None,
        date: "Mon, 1 Jan 2024 12:00:00 +0000".to_string(),
        additional: HashMap::new(),
    };
    
    let original = Message {
        id: "<original@example.com>".to_string(),
        is_match: false,
        excluded: false,
        filename: vec![],
        timestamp: 1704110400,
        date_relative: "2024-01-01".to_string(),
        tags: vec![],
        duplicate: None,
        body: vec![BodyPart {
            id: 1,
            content_type: "text/plain".to_string(),
            content: whynot::body::BodyContent::Text("Original message body".to_string()),
            filename: None,
            content_id: None,
            content_length: None,
            content_disposition: None,
            content_transfer_encoding: None,
        }],
        crypto: Default::default(),
        headers,
    };
    
    let forward = ComposableMessage::forward_builder(&original)
        .to("newrecipient@example.com".to_string())
        .build()
        .unwrap();
    
    assert_eq!(forward.subject, "Fwd: Original Subject");
    assert_eq!(forward.to, vec!["newrecipient@example.com"]);
    assert!(forward.body.contains("---------- Forwarded message ----------"));
    assert!(forward.body.contains("From: original@example.com"));
    assert!(forward.body.contains("Original message body"));
}

// Mock implementation for testing
struct MockMailSender {
    sent_messages: std::sync::Arc<std::sync::Mutex<Vec<ComposableMessage>>>,
}

impl MockMailSender {
    fn new() -> Self {
        Self {
            sent_messages: std::sync::Arc::new(std::sync::Mutex::new(Vec::new())),
        }
    }
}

#[async_trait::async_trait]
impl MailSender for MockMailSender {
    async fn send(&self, message: ComposableMessage) -> Result<String> {
        let message_id = message.message_id.clone();
        self.sent_messages.lock().unwrap().push(message);
        Ok(message_id)
    }

    async fn reply(&self, original: &Message, reply: ComposableMessage, reply_all: bool) -> Result<String> {
        let mut full_reply = ComposableMessage::reply_builder(original, reply_all)
            .body(reply.body.clone())
            .build()?;
        
        // Merge in any additional fields from the reply
        if let Some(from) = reply.from {
            full_reply.from = Some(from);
        }
        
        self.send(full_reply).await
    }

    async fn forward(&self, original: &Message, forward: ComposableMessage) -> Result<String> {
        let mut full_forward = ComposableMessage::forward_builder(original)
            .body(forward.body.clone())
            .build()?;
        
        // Merge in recipients from forward
        full_forward.to = forward.to;
        full_forward.cc = forward.cc;
        full_forward.bcc = forward.bcc;
        
        if let Some(from) = forward.from {
            full_forward.from = Some(from);
        }
        
        self.send(full_forward).await
    }

    async fn test_connection(&self) -> Result<()> {
        Ok(())
    }

    async fn get_from_address(&self) -> Result<String> {
        Ok("test@example.com".to_string())
    }
}

#[test]
async fn test_mock_mail_sender_send() {
    let sender = MockMailSender::new();
    
    let message = ComposableMessage::builder()
        .to("recipient@example.com".to_string())
        .from("sender@example.com".to_string())
        .subject("Test".to_string())
        .body("Test body".to_string())
        .build()
        .unwrap();
    
    let result = sender.send(message.clone()).await;
    assert!(result.is_ok());
    
    let sent = sender.sent_messages.lock().unwrap();
    assert_eq!(sent.len(), 1);
    assert_eq!(sent[0].to, vec!["recipient@example.com"]);
}

#[test]
async fn test_mock_mail_sender_reply() {
    use whynot::common::Headers;
    use std::collections::HashMap;
    
    let sender = MockMailSender::new();
    
    let headers = Headers {
        subject: "Original".to_string(),
        from: "original@example.com".to_string(),
        to: "me@example.com".to_string(),
        reply_to: None,
        date: "Mon, 1 Jan 2024 12:00:00 +0000".to_string(),
        additional: HashMap::new(),
    };
    
    let original = Message {
        id: "<original@example.com>".to_string(),
        is_match: false,
        excluded: false,
        filename: vec![],
        timestamp: 1704110400,
        date_relative: "2024-01-01".to_string(),
        tags: vec![],
        duplicate: None,
        body: vec![],
        crypto: Default::default(),
        headers,
    };
    
    let reply = ComposableMessage::builder()
        .to("dummy@example.com".to_string())  // This will be replaced by reply builder
        .body("My reply".to_string())
        .build()
        .unwrap();
    
    let result = sender.reply(&original, reply, false).await;
    assert!(result.is_ok());
    
    let sent = sender.sent_messages.lock().unwrap();
    assert_eq!(sent.len(), 1);
    assert_eq!(sent[0].subject, "Re: Original");
    assert_eq!(sent[0].to, vec!["original@example.com"]);
    assert_eq!(sent[0].in_reply_to, Some("<original@example.com>".to_string()));
}