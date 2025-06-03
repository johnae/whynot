pub mod mbox;
pub mod notmuch;

#[cfg(feature = "test-utils")]
pub use mbox::{Attachment, EmailMessage, MboxBuilder};
#[cfg(feature = "test-utils")]
pub use notmuch::TestNotmuch;

#[cfg(feature = "test-utils")]
pub fn create_test_message_with_attachment(
    subject: &str,
    from: &str,
    to: &str,
    body: &str,
    filename: &str,
    content_type: &str,
    content: &[u8],
) -> EmailMessage {
    EmailMessage::new(subject)
        .with_from(from)
        .with_to(vec![to.to_string()])
        .with_body(body)
        .with_attachment(filename, content_type, content)
}

#[cfg(feature = "test-utils")]
pub fn create_test_message_with_multiple_attachments(
    subject: &str,
    from: &str,
    to: &str,
    body: &str,
    attachments: Vec<(&str, &str, Vec<u8>)>,
) -> EmailMessage {
    let mut msg = EmailMessage::new(subject)
        .with_from(from)
        .with_to(vec![to.to_string()])
        .with_body(body);

    for (filename, content_type, content) in attachments {
        msg = msg.with_attachment(filename, content_type, &content);
    }

    msg
}
