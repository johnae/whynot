//! Configuration types for mail sender clients.

use std::path::PathBuf;

/// Configuration for creating a mail sender client.
///
/// This enum determines whether to use local msmtp execution or
/// remote execution via SSH.
#[derive(Debug, Clone)]
pub enum MailSenderConfig {
    /// Configuration for local msmtp execution.
    Local {
        /// Path to the msmtp binary (defaults to "msmtp" in PATH).
        msmtp_path: Option<PathBuf>,
        /// Path to msmtp configuration file (defaults to ~/.msmtprc).
        config_path: Option<PathBuf>,
    },
    /// Configuration for remote msmtp execution via SSH.
    Remote {
        /// The remote host to connect to.
        host: String,
        /// The username for SSH connection (defaults to current user).
        user: Option<String>,
        /// The SSH port (defaults to 22).
        port: Option<u16>,
        /// Path to SSH identity file for authentication.
        identity_file: Option<PathBuf>,
        /// Path to the msmtp binary on the remote host.
        msmtp_path: Option<PathBuf>,
        /// Path to msmtp configuration file on the remote host.
        config_path: Option<PathBuf>,
    },
}

impl MailSenderConfig {
    /// Create a local configuration from environment variables.
    ///
    /// Recognizes:
    /// - `MSMTP_PATH` - Path to msmtp binary
    /// - `MSMTP_CONFIG` - Path to msmtp config file
    pub fn from_env_local() -> Self {
        Self::Local {
            msmtp_path: std::env::var("MSMTP_PATH").ok().map(PathBuf::from),
            config_path: std::env::var("MSMTP_CONFIG").ok().map(PathBuf::from),
        }
    }

    /// Create a remote configuration from environment variables.
    ///
    /// Recognizes:
    /// - `MSMTP_HOST` - Remote host (required)
    /// - `MSMTP_USER` - SSH username
    /// - `MSMTP_PORT` - SSH port
    /// - `MSMTP_IDENTITY_FILE` - SSH identity file
    /// - `MSMTP_PATH` - Path to msmtp on remote
    /// - `MSMTP_CONFIG` - Path to config on remote
    ///
    /// Returns None if MSMTP_HOST is not set.
    pub fn from_env_remote() -> Option<Self> {
        let host = std::env::var("MSMTP_HOST").ok()?;
        Some(Self::Remote {
            host,
            user: std::env::var("MSMTP_USER").ok(),
            port: std::env::var("MSMTP_PORT")
                .ok()
                .and_then(|p| p.parse().ok()),
            identity_file: std::env::var("MSMTP_IDENTITY_FILE")
                .ok()
                .map(PathBuf::from),
            msmtp_path: std::env::var("MSMTP_PATH").ok().map(PathBuf::from),
            config_path: std::env::var("MSMTP_CONFIG").ok().map(PathBuf::from),
        })
    }

    /// Create configuration from environment, preferring remote if available.
    pub fn from_env() -> Self {
        Self::from_env_remote().unwrap_or_else(Self::from_env_local)
    }
}