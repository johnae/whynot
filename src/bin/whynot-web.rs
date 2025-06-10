use clap::Parser;
use whynot::client::create_client;
use whynot::config::{Config, CliArgs};
use whynot::mail_sender::create_mail_sender;
use whynot::web::{AppState, WebConfig, create_app};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "whynot=info,tower_http=info".into()),
        )
        .init();

    // Parse CLI arguments and load configuration
    let cli_args = CliArgs::parse();
    let config = Config::load(cli_args)?;

    // Create client configuration from unified config
    let client_config = config.to_client_config()?;
    
    // Log configuration mode
    let is_remote = config.mail.reading.connection_type.as_deref() == Some("remote") 
        || config.mail.reading.host.is_some();
    
    if is_remote {
        tracing::info!(
            "Using remote notmuch at {}@{}:{}",
            config.mail.reading.user.as_deref().unwrap_or("(default)"),
            config.mail.reading.host.as_deref().unwrap_or("(unknown)"),
            config.mail.reading.port.unwrap_or(22)
        );
    } else {
        tracing::info!("Using local notmuch");
    }

    // Create the notmuch client
    let client = create_client(client_config)?;

    // Test the connection by trying to list tags
    tracing::info!("Testing notmuch connection...");
    match client.list_tags().await {
        Ok(tags) => {
            tracing::info!("Connection successful. Found {} tags", tags.len());
            if tags.is_empty() {
                tracing::warn!(
                    "No tags found - the database might be empty or the connection might not be working correctly"
                );
            }
        }
        Err(e) => {
            tracing::error!("Failed to connect to notmuch: {}", e);
            eprintln!("ERROR: Failed to connect to notmuch: {}", e);
            eprintln!("Please check your configuration and try again.");
            std::process::exit(1);
        }
    }

    // Create mail sender if configured
    let mail_sender = if let Ok(mail_sender_config) = config.to_mail_sender_config() {
        tracing::info!("Creating mail sender...");
        match create_mail_sender(mail_sender_config) {
            Ok(sender) => {
                // Test the mail sender connection
                match sender.test_connection().await {
                    Ok(()) => {
                        tracing::info!("Mail sender connection successful");
                        Some(sender)
                    }
                    Err(e) => {
                        tracing::warn!("Mail sender test failed: {}. Mail sending will be disabled.", e);
                        None
                    }
                }
            }
            Err(e) => {
                tracing::warn!("Failed to create mail sender: {}. Mail sending will be disabled.", e);
                None
            }
        }
    } else {
        tracing::info!("Mail sending not configured");
        None
    };

    // Create web configuration from unified config
    let web_config = WebConfig {
        bind_address: config.bind_address()?,
        base_url: config.base_url(),
        items_per_page: config.items_per_page(),
        auto_refresh_interval: config.general.auto_refresh_interval.unwrap_or(30),
    };

    let state = AppState {
        client: std::sync::Arc::from(client),
        mail_sender: mail_sender.map(std::sync::Arc::from),
        config: web_config.clone(),
        user_config: config.user.clone(),
    };

    // Create the application
    let app = create_app(state);

    // Create the TCP listener
    let listener = tokio::net::TcpListener::bind(web_config.bind_address).await?;
    let local_addr = listener.local_addr()?;

    println!("Whynot Web Server");
    println!("=================");
    println!("Listening on: http://{}", local_addr);
    println!("Base URL: {}", web_config.base_url);
    println!("Items per page: {}", web_config.items_per_page);

    // Display client configuration info
    display_client_info(&config);

    println!();
    println!("Press Ctrl+C to stop");

    // Serve the application
    axum::serve(listener, app).await?;

    Ok(())
}


fn display_client_info(config: &Config) {
    let is_remote = config.mail.reading.connection_type.as_deref() == Some("remote") 
        || config.mail.reading.host.is_some();

    if is_remote {
        println!("Notmuch mode: Remote");
        if let Some(host) = &config.mail.reading.host {
            println!("Remote host: {}", host);
        }
        if let Some(user) = &config.mail.reading.user {
            println!("Remote user: {}", user);
        }
        if let Some(port) = config.mail.reading.port {
            if port != 22 {
                println!("Remote port: {}", port);
            }
        }
        if let Some(path) = &config.mail.reading.notmuch_path {
            println!("Remote notmuch path: {}", path);
        }
    } else {
        println!("Notmuch mode: Local");
        if let Some(db) = &config.mail.reading.database_path {
            println!("Database path: {}", db);
        }
        if let Some(path) = &config.mail.reading.notmuch_path {
            println!("Notmuch path: {}", path);
        }
    }

    // Display user configuration if available
    if let Some(name) = &config.user.name {
        println!("User name: {}", name);
    }
    if let Some(email) = &config.user.email {
        println!("User email: {}", email);
    }

    // Display mail sending configuration if configured
    let sending_configured = config.mail.sending.connection_type.is_some() 
        || config.mail.sending.host.is_some() 
        || config.mail.sending.msmtp_path.is_some();
    
    if sending_configured {
        let is_sending_remote = config.mail.sending.connection_type.as_deref() == Some("remote") 
            || config.mail.sending.host.is_some();
        
        if is_sending_remote {
            println!("Mail sending: Remote (via SSH)");
            if let Some(host) = &config.mail.sending.host {
                println!("MSMTP host: {}", host);
            }
            if let Some(user) = &config.mail.sending.user {
                println!("MSMTP user: {}", user);
            }
        } else {
            println!("Mail sending: Local");
        }
        
        if let Some(path) = &config.mail.sending.msmtp_path {
            println!("MSMTP path: {}", path);
        }
        if let Some(config_path) = &config.mail.sending.config_path {
            println!("MSMTP config: {}", config_path);
        }
    }
}