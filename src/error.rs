use thiserror::Error;

#[derive(Debug, Error)]
pub enum NotmuchError {
    #[error("Command execution failed: {0}")]
    CommandFailed(String),

    #[error("SSH connection failed: {0}")]
    SshError(String),

    #[error("Parse error: {0}")]
    ParseError(#[from] serde_json::Error),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Notmuch database error: {0}")]
    DatabaseError(String),

    #[error("Invalid configuration: {0}")]
    ConfigError(String),

    #[error("Mail sending failed: {0}")]
    MailSendError(String),

    #[error("Invalid input: {0}")]
    InvalidInput(String),
}

pub type Result<T> = std::result::Result<T, NotmuchError>;
pub use NotmuchError as Error;
