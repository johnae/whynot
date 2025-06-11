//! Unified configuration system for Whynot Mail.
//!
//! This module provides a comprehensive configuration system that supports:
//! - Configuration files (TOML format)
//! - Environment variables (WHYNOT_ prefix)
//! - CLI arguments
//!
//! Configuration precedence (highest to lowest):
//! 1. CLI arguments - Most explicit, immediate user intent
//! 2. Environment variables - Session/deployment specific overrides
//! 3. Configuration file - Persistent defaults

use crate::error::{NotmuchError, Result};
use clap::Parser;
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Config {
    #[serde(default)]
    pub mail: MailConfig,

    #[serde(default)]
    pub ui: UiConfig,

    #[serde(default)]
    pub user: UserConfig,

    #[serde(default)]
    pub general: GeneralConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MailConfig {
    #[serde(default)]
    pub reading: MailReadingConfig,

    #[serde(default)]
    pub sending: MailSendingConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MailReadingConfig {
    #[serde(rename = "type")]
    pub connection_type: Option<String>, // "local" or "remote"
    pub host: Option<String>,
    pub user: Option<String>,
    pub port: Option<u16>,
    pub notmuch_path: Option<String>,
    pub database_path: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MailSendingConfig {
    #[serde(rename = "type")]
    pub connection_type: Option<String>, // "local" or "remote"
    pub host: Option<String>,
    pub user: Option<String>,
    pub port: Option<u16>,
    pub msmtp_path: Option<String>,
    pub config_path: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UiConfig {
    #[serde(default)]
    pub web: WebConfig,

    #[serde(default)]
    pub tui: TuiConfig, // Future TUI settings
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebConfig {
    pub bind: Option<String>,
    pub base_url: Option<String>,
    pub items_per_page: Option<usize>,
    pub default_theme: Option<String>,
    pub initial_page_size: Option<usize>,
    pub pagination_size: Option<usize>,
    pub infinite_scroll_enabled: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TuiConfig {
    pub keybindings: Option<String>,
    pub show_sidebar: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UserConfig {
    pub name: Option<String>,
    pub email: Option<String>,
    pub signature: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneralConfig {
    pub auto_refresh_interval: Option<u64>,
    pub threading_enabled: Option<bool>,
}

#[derive(Parser, Debug)]
#[command(name = "whynot-web")]
#[command(about = "Web interface for notmuch email", long_about = None)]
#[command(
    after_help = "ENVIRONMENT VARIABLES:\n  WHYNOT_NOTMUCH_HOST    Remote notmuch server hostname\n  WHYNOT_NOTMUCH_USER    Remote notmuch server username\n  WHYNOT_NOTMUCH_PORT    Remote notmuch server SSH port\n  WHYNOT_BIND_ADDRESS    Web server bind address\n  \n  Legacy environment variables (for backward compatibility):\n  NOTMUCH_HOST           Remote notmuch server hostname\n  NOTMUCH_USER           Remote notmuch server username\n  NOTMUCH_PORT           Remote notmuch server SSH port"
)]
pub struct CliArgs {
    // Web UI options
    #[arg(
        short,
        long,
        env = "WHYNOT_BIND_ADDRESS",
        help = "Bind address for the web server"
    )]
    pub bind: Option<String>,

    #[arg(long, env = "WHYNOT_BASE_URL", help = "Base URL for the application")]
    pub base_url: Option<String>,

    #[arg(long, env = "WHYNOT_ITEMS_PER_PAGE", help = "Number of items per page")]
    pub items_per_page: Option<usize>,

    #[arg(
        long,
        env = "WHYNOT_DEFAULT_THEME",
        help = "Default theme (light/dark)"
    )]
    pub default_theme: Option<String>,

    #[arg(
        long,
        env = "WHYNOT_INITIAL_PAGE_SIZE",
        help = "Number of messages to load initially"
    )]
    pub initial_page_size: Option<usize>,

    #[arg(
        long,
        env = "WHYNOT_PAGINATION_SIZE",
        help = "Number of messages to load when scrolling"
    )]
    pub pagination_size: Option<usize>,

    #[arg(
        long,
        env = "WHYNOT_INFINITE_SCROLL_ENABLED",
        help = "Enable infinite scrolling"
    )]
    pub infinite_scroll_enabled: Option<bool>,

    // Mail reading options
    #[arg(
        long,
        env = "WHYNOT_NOTMUCH_HOST",
        help = "Remote notmuch server hostname"
    )]
    pub notmuch_host: Option<String>,

    #[arg(
        long,
        env = "WHYNOT_NOTMUCH_USER",
        help = "Remote notmuch server username"
    )]
    pub notmuch_user: Option<String>,

    #[arg(
        long,
        env = "WHYNOT_NOTMUCH_PORT",
        help = "Remote notmuch server SSH port"
    )]
    pub notmuch_port: Option<u16>,

    #[arg(long, env = "WHYNOT_NOTMUCH_PATH", help = "Path to notmuch executable")]
    pub notmuch_path: Option<String>,

    #[arg(
        long,
        env = "WHYNOT_NOTMUCH_DATABASE",
        help = "Path to notmuch database (for local mode)"
    )]
    pub notmuch_database: Option<String>,

    // Mail sending options
    #[arg(long, env = "WHYNOT_MSMTP_HOST", help = "Remote msmtp server hostname")]
    pub msmtp_host: Option<String>,

    #[arg(long, env = "WHYNOT_MSMTP_USER", help = "Remote msmtp server username")]
    pub msmtp_user: Option<String>,

    #[arg(long, env = "WHYNOT_MSMTP_PORT", help = "Remote msmtp server SSH port")]
    pub msmtp_port: Option<u16>,

    #[arg(long, env = "WHYNOT_MSMTP_PATH", help = "Path to msmtp executable")]
    pub msmtp_path: Option<String>,

    #[arg(
        long,
        env = "WHYNOT_MSMTP_CONFIG",
        help = "Path to msmtp configuration file"
    )]
    pub msmtp_config_path: Option<String>,

    // User identity options
    #[arg(long, env = "WHYNOT_USER_NAME", help = "User's full name for email")]
    pub user_name: Option<String>,

    #[arg(long, env = "WHYNOT_USER_EMAIL", help = "User's email address")]
    pub user_email: Option<String>,

    #[arg(long, env = "WHYNOT_USER_SIGNATURE", help = "User's email signature")]
    pub user_signature: Option<String>,

    // General options
    #[arg(
        long,
        env = "WHYNOT_AUTO_REFRESH_INTERVAL",
        help = "Auto refresh interval in seconds"
    )]
    pub auto_refresh_interval: Option<u64>,

    #[arg(
        long,
        env = "WHYNOT_THREADING_ENABLED",
        help = "Enable email threading"
    )]
    pub threading_enabled: Option<bool>,

    // Configuration file
    #[arg(
        short,
        long,
        env = "WHYNOT_CONFIG",
        help = "Path to configuration file"
    )]
    pub config: Option<PathBuf>,

    // Legacy CLI arguments (for backward compatibility)
    #[arg(long, help = "Remote server hostname (legacy, use --notmuch-host)")]
    pub remote: Option<String>,

    #[arg(long, help = "Remote server username (legacy, use --notmuch-user)")]
    pub user: Option<String>,

    #[arg(long, help = "Remote server SSH port (legacy, use --notmuch-port)")]
    pub port: Option<u16>,

    #[arg(
        long,
        help = "Path to notmuch database (legacy, use --notmuch-database)"
    )]
    pub database: Option<String>,
}

impl Default for WebConfig {
    fn default() -> Self {
        Self {
            bind: Some("127.0.0.1:8080".to_string()),
            base_url: Some("http://localhost:8080".to_string()),
            items_per_page: Some(50),
            default_theme: Some("light".to_string()),
            initial_page_size: Some(20),
            pagination_size: Some(10),
            infinite_scroll_enabled: Some(true),
        }
    }
}

impl Default for TuiConfig {
    fn default() -> Self {
        Self {
            keybindings: Some("vim".to_string()),
            show_sidebar: Some(true),
        }
    }
}

impl Default for GeneralConfig {
    fn default() -> Self {
        Self {
            auto_refresh_interval: Some(300),
            threading_enabled: Some(true),
        }
    }
}

impl Config {
    /// Load configuration from all sources with proper precedence:
    /// CLI args > Environment variables > Configuration file > Defaults
    pub fn load(cli_args: CliArgs) -> Result<Self> {
        // Start with default configuration
        let mut config = Config::default();

        // Load from configuration file if specified or found
        if let Some(file_config) = Self::load_from_file(&cli_args.config)? {
            config = Self::merge_configs(config, file_config);
        }

        // Apply environment variables (including legacy support)
        config = Self::apply_environment_variables(config)?;

        // Apply CLI arguments (highest precedence)
        config = Self::apply_cli_args(config, cli_args);

        Ok(config)
    }

    fn load_from_file(config_path: &Option<PathBuf>) -> Result<Option<Config>> {
        let path = match config_path {
            Some(path) => path.clone(),
            None => {
                // Try to find config file in standard locations
                if let Some(config_dir) = dirs::config_dir() {
                    let whynot_config = config_dir.join("whynot").join("config.toml");
                    if whynot_config.exists() {
                        whynot_config
                    } else {
                        // Try XDG location
                        let xdg_config = config_dir.join("whynot.toml");
                        if xdg_config.exists() {
                            xdg_config
                        } else {
                            return Ok(None);
                        }
                    }
                } else {
                    return Ok(None);
                }
            }
        };

        if !path.exists() {
            if config_path.is_some() {
                return Err(NotmuchError::ConfigError(format!(
                    "Configuration file not found: {}",
                    path.display()
                )));
            }
            return Ok(None);
        }

        let content = std::fs::read_to_string(&path).map_err(|e| {
            NotmuchError::ConfigError(format!(
                "Failed to read configuration file {}: {}",
                path.display(),
                e
            ))
        })?;

        let config: Config = toml::from_str(&content).map_err(|e| {
            NotmuchError::ConfigError(format!(
                "Failed to parse configuration file {}: {}",
                path.display(),
                e
            ))
        })?;

        Ok(Some(config))
    }

    fn apply_environment_variables(mut config: Config) -> Result<Config> {
        use std::env;

        // Web configuration
        if let Ok(bind) = env::var("WHYNOT_BIND_ADDRESS") {
            config.ui.web.bind = Some(bind);
        }
        if let Ok(base_url) = env::var("WHYNOT_BASE_URL") {
            config.ui.web.base_url = Some(base_url);
        }
        if let Ok(items) = env::var("WHYNOT_ITEMS_PER_PAGE") {
            config.ui.web.items_per_page = Some(items.parse().map_err(|e| {
                NotmuchError::ConfigError(format!("Invalid WHYNOT_ITEMS_PER_PAGE: {}", e))
            })?);
        }
        if let Ok(theme) = env::var("WHYNOT_DEFAULT_THEME") {
            config.ui.web.default_theme = Some(theme);
        }
        if let Ok(initial_size) = env::var("WHYNOT_INITIAL_PAGE_SIZE") {
            config.ui.web.initial_page_size = Some(initial_size.parse().map_err(|e| {
                NotmuchError::ConfigError(format!("Invalid WHYNOT_INITIAL_PAGE_SIZE: {}", e))
            })?);
        }
        if let Ok(pagination_size) = env::var("WHYNOT_PAGINATION_SIZE") {
            config.ui.web.pagination_size = Some(pagination_size.parse().map_err(|e| {
                NotmuchError::ConfigError(format!("Invalid WHYNOT_PAGINATION_SIZE: {}", e))
            })?);
        }
        if let Ok(infinite_scroll) = env::var("WHYNOT_INFINITE_SCROLL_ENABLED") {
            config.ui.web.infinite_scroll_enabled = Some(infinite_scroll.parse().map_err(|e| {
                NotmuchError::ConfigError(format!("Invalid WHYNOT_INFINITE_SCROLL_ENABLED: {}", e))
            })?);
        }

        // Mail reading configuration
        if let Ok(host) = env::var("WHYNOT_NOTMUCH_HOST") {
            config.mail.reading.host = Some(host);
            config.mail.reading.connection_type = Some("remote".to_string());
        }
        if let Ok(user) = env::var("WHYNOT_NOTMUCH_USER") {
            config.mail.reading.user = Some(user);
        }
        if let Ok(port) = env::var("WHYNOT_NOTMUCH_PORT") {
            config.mail.reading.port = Some(port.parse().map_err(|e| {
                NotmuchError::ConfigError(format!("Invalid WHYNOT_NOTMUCH_PORT: {}", e))
            })?);
        }
        if let Ok(path) = env::var("WHYNOT_NOTMUCH_PATH") {
            config.mail.reading.notmuch_path = Some(path);
        }
        if let Ok(db) = env::var("WHYNOT_NOTMUCH_DATABASE") {
            config.mail.reading.database_path = Some(db);
        }

        // Mail sending configuration
        if let Ok(host) = env::var("WHYNOT_MSMTP_HOST") {
            config.mail.sending.host = Some(host);
            config.mail.sending.connection_type = Some("remote".to_string());
        }
        if let Ok(user) = env::var("WHYNOT_MSMTP_USER") {
            config.mail.sending.user = Some(user);
        }
        if let Ok(port) = env::var("WHYNOT_MSMTP_PORT") {
            config.mail.sending.port = Some(port.parse().map_err(|e| {
                NotmuchError::ConfigError(format!("Invalid WHYNOT_MSMTP_PORT: {}", e))
            })?);
        }
        if let Ok(path) = env::var("WHYNOT_MSMTP_PATH") {
            config.mail.sending.msmtp_path = Some(path);
        }
        if let Ok(config_path) = env::var("WHYNOT_MSMTP_CONFIG") {
            config.mail.sending.config_path = Some(config_path);
        }

        // User configuration
        if let Ok(name) = env::var("WHYNOT_USER_NAME") {
            config.user.name = Some(name);
        }
        if let Ok(email) = env::var("WHYNOT_USER_EMAIL") {
            config.user.email = Some(email);
        }
        if let Ok(signature) = env::var("WHYNOT_USER_SIGNATURE") {
            config.user.signature = Some(signature);
        }

        // General configuration
        if let Ok(interval) = env::var("WHYNOT_AUTO_REFRESH_INTERVAL") {
            config.general.auto_refresh_interval = Some(interval.parse().map_err(|e| {
                NotmuchError::ConfigError(format!("Invalid WHYNOT_AUTO_REFRESH_INTERVAL: {}", e))
            })?);
        }
        if let Ok(threading) = env::var("WHYNOT_THREADING_ENABLED") {
            config.general.threading_enabled = Some(threading.parse().map_err(|e| {
                NotmuchError::ConfigError(format!("Invalid WHYNOT_THREADING_ENABLED: {}", e))
            })?);
        }

        // Legacy environment variable support
        if config.mail.reading.host.is_none() {
            if let Ok(host) = env::var("NOTMUCH_HOST") {
                config.mail.reading.host = Some(host);
                config.mail.reading.connection_type = Some("remote".to_string());
            }
        }
        if config.mail.reading.user.is_none() {
            if let Ok(user) = env::var("NOTMUCH_USER") {
                config.mail.reading.user = Some(user);
            }
        }
        if config.mail.reading.port.is_none() {
            if let Ok(port) = env::var("NOTMUCH_PORT") {
                config.mail.reading.port = Some(port.parse().map_err(|e| {
                    NotmuchError::ConfigError(format!("Invalid NOTMUCH_PORT: {}", e))
                })?);
            }
        }

        Ok(config)
    }

    fn apply_cli_args(mut config: Config, args: CliArgs) -> Config {
        // Web configuration
        if let Some(bind) = args.bind {
            config.ui.web.bind = Some(bind);
        }
        if let Some(base_url) = args.base_url {
            config.ui.web.base_url = Some(base_url);
        }
        if let Some(items) = args.items_per_page {
            config.ui.web.items_per_page = Some(items);
        }
        if let Some(theme) = args.default_theme {
            config.ui.web.default_theme = Some(theme);
        }
        if let Some(initial_size) = args.initial_page_size {
            config.ui.web.initial_page_size = Some(initial_size);
        }
        if let Some(pagination_size) = args.pagination_size {
            config.ui.web.pagination_size = Some(pagination_size);
        }
        if let Some(infinite_scroll) = args.infinite_scroll_enabled {
            config.ui.web.infinite_scroll_enabled = Some(infinite_scroll);
        }

        // Mail reading configuration
        if let Some(host) = args.notmuch_host.or(args.remote) {
            config.mail.reading.host = Some(host);
            config.mail.reading.connection_type = Some("remote".to_string());
        }
        if let Some(user) = args.notmuch_user.or(args.user) {
            config.mail.reading.user = Some(user);
        }
        if let Some(port) = args.notmuch_port.or(args.port) {
            config.mail.reading.port = Some(port);
        }
        if let Some(path) = args.notmuch_path {
            config.mail.reading.notmuch_path = Some(path);
        }
        if let Some(db) = args.notmuch_database.or(args.database) {
            config.mail.reading.database_path = Some(db);
        }

        // Mail sending configuration
        if let Some(host) = args.msmtp_host {
            config.mail.sending.host = Some(host);
            config.mail.sending.connection_type = Some("remote".to_string());
        }
        if let Some(user) = args.msmtp_user {
            config.mail.sending.user = Some(user);
        }
        if let Some(port) = args.msmtp_port {
            config.mail.sending.port = Some(port);
        }
        if let Some(path) = args.msmtp_path {
            config.mail.sending.msmtp_path = Some(path);
        }
        if let Some(config_path) = args.msmtp_config_path {
            config.mail.sending.config_path = Some(config_path);
        }

        // User configuration
        if let Some(name) = args.user_name {
            config.user.name = Some(name);
        }
        if let Some(email) = args.user_email {
            config.user.email = Some(email);
        }
        if let Some(signature) = args.user_signature {
            config.user.signature = Some(signature);
        }

        // General configuration
        if let Some(interval) = args.auto_refresh_interval {
            config.general.auto_refresh_interval = Some(interval);
        }
        if let Some(threading) = args.threading_enabled {
            config.general.threading_enabled = Some(threading);
        }

        config
    }

    fn merge_configs(mut base: Config, other: Config) -> Config {
        // Merge mail reading config
        if other.mail.reading.connection_type.is_some() {
            base.mail.reading.connection_type = other.mail.reading.connection_type;
        }
        if other.mail.reading.host.is_some() {
            base.mail.reading.host = other.mail.reading.host;
        }
        if other.mail.reading.user.is_some() {
            base.mail.reading.user = other.mail.reading.user;
        }
        if other.mail.reading.port.is_some() {
            base.mail.reading.port = other.mail.reading.port;
        }
        if other.mail.reading.notmuch_path.is_some() {
            base.mail.reading.notmuch_path = other.mail.reading.notmuch_path;
        }
        if other.mail.reading.database_path.is_some() {
            base.mail.reading.database_path = other.mail.reading.database_path;
        }

        // Merge mail sending config
        if other.mail.sending.connection_type.is_some() {
            base.mail.sending.connection_type = other.mail.sending.connection_type;
        }
        if other.mail.sending.host.is_some() {
            base.mail.sending.host = other.mail.sending.host;
        }
        if other.mail.sending.user.is_some() {
            base.mail.sending.user = other.mail.sending.user;
        }
        if other.mail.sending.port.is_some() {
            base.mail.sending.port = other.mail.sending.port;
        }
        if other.mail.sending.msmtp_path.is_some() {
            base.mail.sending.msmtp_path = other.mail.sending.msmtp_path;
        }
        if other.mail.sending.config_path.is_some() {
            base.mail.sending.config_path = other.mail.sending.config_path;
        }

        // Merge web config
        if other.ui.web.bind.is_some() {
            base.ui.web.bind = other.ui.web.bind;
        }
        if other.ui.web.base_url.is_some() {
            base.ui.web.base_url = other.ui.web.base_url;
        }
        if other.ui.web.items_per_page.is_some() {
            base.ui.web.items_per_page = other.ui.web.items_per_page;
        }
        if other.ui.web.default_theme.is_some() {
            base.ui.web.default_theme = other.ui.web.default_theme;
        }
        if other.ui.web.initial_page_size.is_some() {
            base.ui.web.initial_page_size = other.ui.web.initial_page_size;
        }
        if other.ui.web.pagination_size.is_some() {
            base.ui.web.pagination_size = other.ui.web.pagination_size;
        }
        if other.ui.web.infinite_scroll_enabled.is_some() {
            base.ui.web.infinite_scroll_enabled = other.ui.web.infinite_scroll_enabled;
        }

        // Merge TUI config
        if other.ui.tui.keybindings.is_some() {
            base.ui.tui.keybindings = other.ui.tui.keybindings;
        }
        if other.ui.tui.show_sidebar.is_some() {
            base.ui.tui.show_sidebar = other.ui.tui.show_sidebar;
        }

        // Merge user config
        if other.user.name.is_some() {
            base.user.name = other.user.name;
        }
        if other.user.email.is_some() {
            base.user.email = other.user.email;
        }
        if other.user.signature.is_some() {
            base.user.signature = other.user.signature;
        }

        // Merge general config
        if other.general.auto_refresh_interval.is_some() {
            base.general.auto_refresh_interval = other.general.auto_refresh_interval;
        }
        if other.general.threading_enabled.is_some() {
            base.general.threading_enabled = other.general.threading_enabled;
        }

        base
    }

    /// Convert to a socket address for web server binding
    pub fn bind_address(&self) -> Result<SocketAddr> {
        let bind_str =
            self.ui.web.bind.as_ref().ok_or_else(|| {
                NotmuchError::ConfigError("No bind address configured".to_string())
            })?;

        bind_str.parse().map_err(|e| {
            NotmuchError::ConfigError(format!("Invalid bind address '{}': {}", bind_str, e))
        })
    }

    /// Get the base URL for the web application
    pub fn base_url(&self) -> String {
        self.ui
            .web
            .base_url
            .as_ref()
            .unwrap_or(&"http://localhost:8080".to_string())
            .clone()
    }

    /// Get the number of items per page
    pub fn items_per_page(&self) -> usize {
        self.ui.web.items_per_page.unwrap_or(50)
    }

    /// Create a ClientConfig for notmuch from this configuration
    pub fn to_client_config(&self) -> Result<crate::client::ClientConfig> {
        let is_remote = self.mail.reading.connection_type.as_deref() == Some("remote")
            || self.mail.reading.host.is_some();

        if is_remote {
            let host = self
                .mail
                .reading
                .host
                .as_ref()
                .ok_or_else(|| {
                    NotmuchError::ConfigError(
                        "Remote host not configured for mail reading".to_string(),
                    )
                })?
                .clone();

            Ok(crate::client::ClientConfig::Remote {
                host,
                user: self.mail.reading.user.clone(),
                port: self.mail.reading.port,
                identity_file: None,
                notmuch_path: self.mail.reading.notmuch_path.clone().map(Into::into),
            })
        } else {
            Ok(crate::client::ClientConfig::Local {
                notmuch_path: self.mail.reading.notmuch_path.clone().map(Into::into),
                database_path: self.mail.reading.database_path.clone().map(Into::into),
                mail_root: None,
            })
        }
    }

    /// Create a MailSenderConfig from this configuration
    pub fn to_mail_sender_config(&self) -> Result<crate::mail_sender::MailSenderConfig> {
        let is_remote = self.mail.sending.connection_type.as_deref() == Some("remote")
            || self.mail.sending.host.is_some();

        if is_remote {
            let host = self
                .mail
                .sending
                .host
                .as_ref()
                .ok_or_else(|| {
                    NotmuchError::ConfigError(
                        "Remote host not configured for mail sending".to_string(),
                    )
                })?
                .clone();

            Ok(crate::mail_sender::MailSenderConfig::Remote {
                host,
                user: self.mail.sending.user.clone(),
                port: self.mail.sending.port,
                identity_file: None,
                msmtp_path: self.mail.sending.msmtp_path.clone().map(Into::into),
                config_path: self.mail.sending.config_path.clone().map(Into::into),
            })
        } else {
            Ok(crate::mail_sender::MailSenderConfig::Local {
                msmtp_path: self.mail.sending.msmtp_path.clone().map(Into::into),
                config_path: self.mail.sending.config_path.clone().map(Into::into),
            })
        }
    }
}
