use clap::Parser;
use std::net::SocketAddr;
use whynot::client::{ClientConfig, create_client};
use whynot::web::{AppState, WebConfig, create_app};

#[derive(Parser, Debug)]
#[command(name = "whynot-web")]
#[command(about = "Web interface for notmuch email", long_about = None)]
#[command(
    after_help = "ENVIRONMENT VARIABLES:\n  NOTMUCH_HOST    Remote notmuch server hostname (alternative to --remote)\n  NOTMUCH_USER    Remote notmuch server username (alternative to --user)\n  NOTMUCH_PORT    Remote notmuch server SSH port (alternative to --port)"
)]
struct Args {
    /// Bind address for the web server
    #[arg(short, long, default_value = "127.0.0.1:8080")]
    bind: SocketAddr,

    /// Base URL for the application
    #[arg(long, default_value = "http://localhost:8080")]
    base_url: String,

    /// Number of items per page
    #[arg(long, default_value = "50")]
    items_per_page: usize,

    /// Remote server hostname (if not provided, uses local notmuch)
    #[arg(long)]
    remote: Option<String>,

    /// Remote server username
    #[arg(long)]
    user: Option<String>,

    /// Remote server SSH port
    #[arg(long, default_value = "22")]
    port: u16,

    /// Path to notmuch database (for local mode)
    #[arg(long)]
    database: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "whynot=info,tower_http=info".into()),
        )
        .init();

    let args = Args::parse();

    // Check for environment variables if command line args not provided
    let remote_host = args.remote.or_else(|| std::env::var("NOTMUCH_HOST").ok());
    let remote_user = args.user.or_else(|| std::env::var("NOTMUCH_USER").ok());
    let remote_port = if args.port != 22 {
        Some(args.port)
    } else {
        std::env::var("NOTMUCH_PORT")
            .ok()
            .and_then(|p| p.parse::<u16>().ok())
            .or(Some(22))
    };

    // Create client configuration
    let client_config = if let Some(hostname) = remote_host.clone() {
        tracing::info!(
            "Using remote notmuch at {}@{}:{}",
            remote_user.as_deref().unwrap_or("(default)"),
            hostname,
            remote_port.unwrap_or(22)
        );

        ClientConfig::Remote {
            host: hostname,
            user: remote_user.clone(),
            port: remote_port,
            identity_file: None,
            notmuch_path: None,
        }
    } else {
        tracing::info!("Using local notmuch");
        ClientConfig::Local {
            notmuch_path: None,
            database_path: args.database.clone().map(Into::into),
            mail_root: None,
        }
    };

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

    // Create web configuration
    let config = WebConfig {
        bind_address: args.bind,
        base_url: args.base_url,
        items_per_page: args.items_per_page,
    };

    let state = AppState {
        client: std::sync::Arc::from(client),
        config: config.clone(),
    };

    // Create the application
    let app = create_app(state);

    // Create the TCP listener
    let listener = tokio::net::TcpListener::bind(config.bind_address).await?;
    let local_addr = listener.local_addr()?;

    println!("Whynot Web Server");
    println!("=================");
    println!("Listening on: http://{}", local_addr);
    println!("Base URL: {}", config.base_url);
    println!("Items per page: {}", config.items_per_page);

    // Display client mode
    if let Some(hostname) = &remote_host {
        println!("Notmuch mode: Remote");
        println!("Remote host: {}", hostname);
        if let Some(user) = &remote_user {
            println!("Remote user: {}", user);
        }
        if let Some(port) = remote_port {
            if port != 22 {
                println!("Remote port: {}", port);
            }
        }
    } else {
        println!("Notmuch mode: Local");
        if let Some(db) = &args.database {
            println!("Database path: {}", db);
        }
    }

    println!();
    println!("Press Ctrl+C to stop");

    // Serve the application
    axum::serve(listener, app).await?;

    Ok(())
}
