use std::path::PathBuf;

/// Configuration for creating notmuch clients.
///
/// This enum specifies whether to create a local or remote client,
/// along with the necessary connection and path information.
///
/// # Examples
///
/// ```
/// # use whynot::client::ClientConfig;
/// # use std::path::PathBuf;
/// // Default local configuration
/// let local = ClientConfig::local();
///
/// // Local with custom database path
/// let local_custom = ClientConfig::local_with_database(
///     PathBuf::from("/home/user/.mail/.notmuch")
/// );
///
/// // Remote configuration
/// let remote = ClientConfig::remote("mail.example.com".to_string());
///
/// // Remote with full options
/// let remote_full = ClientConfig::remote_full(
///     "mail.example.com".to_string(),
///     "alice".to_string(),
///     2222,
///     PathBuf::from("/home/alice/.ssh/id_rsa")
/// );
/// ```
#[derive(Debug, Clone)]
pub enum ClientConfig {
    /// Configuration for local notmuch execution.
    Local {
        /// Path to the notmuch binary. If None, uses "notmuch" from PATH.
        notmuch_path: Option<PathBuf>,
        /// Path to the notmuch database. If None, uses the default location.
        /// When specified, sets the NOTMUCH_DATABASE environment variable.
        database_path: Option<PathBuf>,
        /// Path to the mail root directory. Currently unused but reserved
        /// for future functionality.
        mail_root: Option<PathBuf>,
    },
    /// Configuration for remote notmuch execution via SSH.
    Remote {
        /// Hostname or IP address of the remote host.
        host: String,
        /// SSH username. If None, uses the current user or SSH config default.
        user: Option<String>,
        /// SSH port. If None, uses the default SSH port (22).
        port: Option<u16>,
        /// Path to SSH identity file (private key). If None, uses SSH defaults.
        identity_file: Option<PathBuf>,
        /// Path to notmuch binary on the remote host. If None, uses "notmuch" from PATH.
        notmuch_path: Option<PathBuf>,
    },
}

impl Default for ClientConfig {
    fn default() -> Self {
        ClientConfig::Local {
            notmuch_path: None,
            database_path: None,
            mail_root: None,
        }
    }
}

impl ClientConfig {
    /// Create a default local client configuration.
    ///
    /// Uses system defaults for all paths.
    pub fn local() -> Self {
        Self::default()
    }

    /// Create a local client configuration with a specific database path.
    ///
    /// # Arguments
    ///
    /// * `database_path` - Path to the notmuch database directory
    pub fn local_with_database(database_path: PathBuf) -> Self {
        ClientConfig::Local {
            notmuch_path: None,
            database_path: Some(database_path),
            mail_root: None,
        }
    }

    /// Create a basic remote client configuration.
    ///
    /// Uses SSH defaults for user, port, and authentication.
    ///
    /// # Arguments
    ///
    /// * `host` - Hostname or IP address of the remote host
    pub fn remote(host: String) -> Self {
        ClientConfig::Remote {
            host,
            user: None,
            port: None,
            identity_file: None,
            notmuch_path: None,
        }
    }

    /// Create a remote client configuration with a specific user.
    ///
    /// # Arguments
    ///
    /// * `host` - Hostname or IP address of the remote host
    /// * `user` - SSH username
    pub fn remote_with_user(host: String, user: String) -> Self {
        ClientConfig::Remote {
            host,
            user: Some(user),
            port: None,
            identity_file: None,
            notmuch_path: None,
        }
    }

    /// Create a remote client configuration with full SSH options.
    ///
    /// # Arguments
    ///
    /// * `host` - Hostname or IP address of the remote host
    /// * `user` - SSH username
    /// * `port` - SSH port number
    /// * `identity_file` - Path to SSH private key file
    pub fn remote_full(host: String, user: String, port: u16, identity_file: PathBuf) -> Self {
        ClientConfig::Remote {
            host,
            user: Some(user),
            port: Some(port),
            identity_file: Some(identity_file),
            notmuch_path: None,
        }
    }
}
