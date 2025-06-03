use clap::{Parser, Subcommand};
use colored::*;
use std::path::PathBuf;
use whynot::client::{ClientConfig, NotmuchClient, TagOperation, create_client};
use whynot::error::Result;

#[derive(Parser)]
#[command(name = "notmuch-client")]
#[command(about = "Interactive notmuch client for testing", long_about = None)]
struct Cli {
    /// Use remote mode (SSH)
    #[arg(short, long)]
    remote: bool,

    /// Remote host (required for remote mode)
    #[arg(long, required_if_eq("remote", "true"))]
    host: Option<String>,

    /// SSH user
    #[arg(long)]
    user: Option<String>,

    /// SSH port
    #[arg(long)]
    port: Option<u16>,

    /// SSH identity file
    #[arg(long)]
    identity_file: Option<PathBuf>,

    /// Path to notmuch binary
    #[arg(long)]
    notmuch_path: Option<PathBuf>,

    /// Local database path
    #[arg(long)]
    database_path: Option<PathBuf>,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Search for messages
    Search {
        /// Search query
        query: String,

        /// Show full thread IDs
        #[arg(short, long)]
        full_thread_id: bool,
    },

    /// Show messages in a thread
    Show {
        /// Query (usually thread:ID)
        query: String,

        /// Show raw headers
        #[arg(short, long)]
        raw: bool,
    },

    /// Tag messages
    Tag {
        /// Query to select messages
        query: String,

        /// Tags to add (format: +tag)
        #[arg(short, long)]
        add: Vec<String>,

        /// Tags to remove (format: -tag)
        #[arg(short, long)]
        remove: Vec<String>,
    },

    /// Refresh the database (scan for new messages)
    Refresh,

    /// Get configuration value
    ConfigGet {
        /// Configuration key
        key: String,
    },

    /// Set configuration value
    ConfigSet {
        /// Configuration key
        key: String,
        /// Configuration value
        value: String,
    },

    /// Insert a test message
    Insert {
        /// Subject
        #[arg(short, long, default_value = "Test Message")]
        subject: String,

        /// From address
        #[arg(short, long, default_value = "test@example.com")]
        from: String,

        /// To address
        #[arg(short, long, default_value = "recipient@example.com")]
        to: String,

        /// Message body
        #[arg(short, long, default_value = "This is a test message.")]
        body: String,

        /// Folder
        #[arg(long)]
        folder: Option<String>,

        /// Tags to apply
        #[arg(long)]
        tags: Vec<String>,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    // Create client configuration
    let config = if cli.remote {
        let host = cli.host.expect("Host is required for remote mode");
        ClientConfig::Remote {
            host,
            user: cli.user,
            port: cli.port,
            identity_file: cli.identity_file,
            notmuch_path: cli.notmuch_path,
        }
    } else {
        ClientConfig::Local {
            notmuch_path: cli.notmuch_path,
            database_path: cli.database_path,
            mail_root: None,
        }
    };

    // Create client
    let client = create_client(config)?;

    // Execute command
    match cli.command {
        Commands::Search {
            query,
            full_thread_id,
        } => {
            search(&*client, &query, full_thread_id).await?;
        }
        Commands::Show { query, raw } => {
            show(&*client, &query, raw).await?;
        }
        Commands::Tag { query, add, remove } => {
            tag(&*client, &query, &add, &remove).await?;
        }
        Commands::Refresh => {
            refresh(&*client).await?;
        }
        Commands::ConfigGet { key } => {
            config_get(&*client, &key).await?;
        }
        Commands::ConfigSet { key, value } => {
            config_set(&*client, &key, &value).await?;
        }
        Commands::Insert {
            subject,
            from,
            to,
            body,
            folder,
            tags,
        } => {
            insert(
                &*client,
                &subject,
                &from,
                &to,
                &body,
                folder.as_deref(),
                &tags,
            )
            .await?;
        }
    }

    Ok(())
}

async fn search(client: &dyn NotmuchClient, query: &str, full_thread_id: bool) -> Result<()> {
    println!("{}", "Searching...".dimmed());
    let results = client.search(query).await?;

    if results.is_empty() {
        println!("{}", "No results found.".yellow());
        return Ok(());
    }

    println!(
        "{} {} found\n",
        results.len().to_string().green(),
        "threads".green()
    );

    for (i, item) in results.iter().enumerate() {
        // Thread ID
        let thread_id = if full_thread_id {
            item.thread_id().to_string()
        } else {
            item.thread_id().chars().take(12).collect::<String>()
        };
        print!("{} ", thread_id.blue());

        // Date
        let date = chrono::DateTime::from_timestamp(item.timestamp, 0)
            .map(|dt| dt.format("%Y-%m-%d").to_string())
            .unwrap_or_else(|| "unknown".to_string());
        print!("{} ", date.dimmed());

        // Tags
        if !item.tags.is_empty() {
            print!("[");
            for (j, tag) in item.tags.iter().enumerate() {
                if j > 0 {
                    print!(" ");
                }
                let colored_tag = match tag.as_str() {
                    "unread" => tag.red(),
                    "inbox" => tag.yellow(),
                    "flagged" => tag.bright_yellow(),
                    _ => tag.normal(),
                };
                print!("{}", colored_tag);
            }
            print!("] ");
        }

        // Subject
        println!("{}", item.subject.bold());

        // Authors
        println!("  {} {}", "from:".dimmed(), item.authors);

        // Message count
        if item.total > 1 {
            println!("  {} messages in thread", item.total);
        }

        if i < results.len() - 1 {
            println!();
        }
    }

    Ok(())
}

async fn show(client: &dyn NotmuchClient, query: &str, raw: bool) -> Result<()> {
    println!("{}", "Loading thread...".dimmed());
    let thread = client.show(query).await?;

    let messages = thread.get_messages();
    println!(
        "{} {} in thread\n",
        messages.len().to_string().green(),
        "messages".green()
    );

    for (i, message) in messages.iter().enumerate() {
        // Message header
        println!(
            "{}",
            format!("Message {}/{}", i + 1, messages.len())
                .blue()
                .bold()
        );

        // Basic headers
        println!("{} {}", "From:".dimmed(), message.headers.from);
        println!("{} {}", "To:".dimmed(), message.headers.to);
        if let Some(reply_to) = &message.headers.reply_to {
            println!("{} {}", "Reply-To:".dimmed(), reply_to);
        }
        println!("{} {}", "Subject:".dimmed(), message.headers.subject.bold());
        println!("{} {}", "Date:".dimmed(), message.headers.date);

        // Tags
        if !message.tags.is_empty() {
            print!("{} ", "Tags:".dimmed());
            for (j, tag) in message.tags.iter().enumerate() {
                if j > 0 {
                    print!(", ");
                }
                let colored_tag = match tag.as_str() {
                    "unread" => tag.red(),
                    "inbox" => tag.yellow(),
                    "flagged" => tag.bright_yellow(),
                    _ => tag.normal(),
                };
                print!("{}", colored_tag);
            }
            println!();
        }

        // Message ID
        println!("{} {}", "Message-ID:".dimmed(), message.id.dimmed());

        // Body
        println!("\n{}", "Body:".dimmed());
        print_body_parts(&message.body, 0);

        // Raw headers if requested
        if raw {
            println!(
                "\n{}",
                "Note: Raw headers are not available in the current data model".dimmed()
            );
        }

        if i < messages.len() - 1 {
            println!("\n{}", "─".repeat(80).dimmed());
            println!();
        }
    }

    Ok(())
}

fn print_body_parts(parts: &[whynot::body::BodyPart], indent: usize) {
    let indent_str = "  ".repeat(indent);

    for part in parts {
        match &part.content {
            whynot::body::BodyContent::Text(text) => {
                // Print text content with proper indentation
                for line in text.lines() {
                    println!("{}{}", indent_str, line);
                }
            }
            whynot::body::BodyContent::Multipart(subparts) => {
                println!("{}[{} multipart]", indent_str, part.content_type.dimmed());
                print_body_parts(subparts, indent + 1);
            }
            whynot::body::BodyContent::Empty => {
                if let Some(filename) = &part.filename {
                    println!(
                        "{}📎 {} ({})",
                        indent_str,
                        filename.yellow(),
                        part.content_type.dimmed()
                    );
                } else if part.is_attachment() {
                    println!(
                        "{}📎 [attachment] ({})",
                        indent_str,
                        part.content_type.dimmed()
                    );
                } else {
                    println!(
                        "{}[empty part] ({})",
                        indent_str,
                        part.content_type.dimmed()
                    );
                }
            }
        }
    }
}

async fn tag(
    client: &dyn NotmuchClient,
    query: &str,
    add: &[String],
    remove: &[String],
) -> Result<()> {
    let mut operations = Vec::new();

    for tag in add {
        operations.push(TagOperation::Add(tag.clone()));
    }

    for tag in remove {
        operations.push(TagOperation::Remove(tag.clone()));
    }

    if operations.is_empty() {
        println!("{}", "No tag operations specified.".yellow());
        return Ok(());
    }

    println!("{}", "Applying tags...".dimmed());
    client.tag(query, &operations).await?;

    println!("{} Tag operations applied:", "✓".green());
    for op in operations {
        match op {
            TagOperation::Add(tag) => println!("  {} {}", "+".green(), tag),
            TagOperation::Remove(tag) => println!("  {} {}", "-".red(), tag),
        }
    }

    Ok(())
}

async fn refresh(client: &dyn NotmuchClient) -> Result<()> {
    println!("{}", "Refreshing database...".dimmed());
    client.refresh().await?;
    println!("{} Database refreshed.", "✓".green());
    Ok(())
}

async fn config_get(client: &dyn NotmuchClient, key: &str) -> Result<()> {
    let value = client.config_get(key).await?;
    println!("{}: {}", key.cyan(), value);
    Ok(())
}

async fn config_set(client: &dyn NotmuchClient, key: &str, value: &str) -> Result<()> {
    client.config_set(key, value).await?;
    println!("{} Configuration updated:", "✓".green());
    println!("  {}: {}", key.cyan(), value);
    Ok(())
}

async fn insert(
    client: &dyn NotmuchClient,
    subject: &str,
    from: &str,
    to: &str,
    body: &str,
    folder: Option<&str>,
    tags: &[String],
) -> Result<()> {
    use chrono::Utc;

    // Create a simple RFC 822 message
    let message = format!(
        "From: {}\r\n\
         To: {}\r\n\
         Subject: {}\r\n\
         Date: {}\r\n\
         Message-ID: <{}@example.com>\r\n\
         \r\n\
         {}\r\n",
        from,
        to,
        subject,
        Utc::now().format("%a, %d %b %Y %H:%M:%S +0000"),
        uuid::Uuid::new_v4(),
        body
    );

    println!("{}", "Inserting message...".dimmed());
    let tags_refs: Vec<&str> = tags.iter().map(|s| s.as_str()).collect();
    let message_id = client
        .insert(message.as_bytes(), folder, &tags_refs)
        .await?;

    println!("{} Message inserted successfully!", "✓".green());
    println!("  {}: {}", "Message-ID".dimmed(), message_id);
    if let Some(folder) = folder {
        println!("  {}: {}", "Folder".dimmed(), folder);
    }
    if !tags.is_empty() {
        println!("  {}: {}", "Tags".dimmed(), tags.join(", "));
    }

    Ok(())
}
