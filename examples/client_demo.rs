use whynot::client::{ClientConfig, create_client};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Example 1: Create a local client
    println!("=== Local Client Example ===");
    let local_config = ClientConfig::local();
    let local_client = create_client(local_config)?;
    
    // Search for messages
    match local_client.search("date:today..").await {
        Ok(results) => {
            println!("Found {} messages from today", results.len());
            for (i, msg) in results.iter().take(5).enumerate() {
                println!("  {}. {} - {}", i + 1, msg.authors, msg.subject);
            }
        }
        Err(e) => {
            println!("Local search failed (this is expected if notmuch is not configured): {}", e);
        }
    }
    
    println!();
    
    // Example 2: Create a remote client
    println!("=== Remote Client Example ===");
    println!("To use the remote client, configure it with your server details:");
    println!();
    println!("let remote_config = ClientConfig::remote_with_user(");
    println!("    \"mail.example.com\".to_string(),");
    println!("    \"username\".to_string()");
    println!(");");
    println!();
    println!("Or with full configuration:");
    println!();
    println!("let remote_config = ClientConfig::remote_full(");
    println!("    \"mail.example.com\".to_string(),");
    println!("    \"username\".to_string(),");
    println!("    2222, // custom SSH port");
    println!("    \"/home/user/.ssh/id_rsa\".into()");
    println!(");");
    println!();
    
    // Demonstrate remote client creation (won't actually connect)
    let remote_config = ClientConfig::remote("example.com".to_string());
    let _remote_client = create_client(remote_config)?;
    println!("Remote client created successfully (no connection attempted)");
    
    Ok(())
}