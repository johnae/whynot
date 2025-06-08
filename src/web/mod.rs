use crate::body::BodyContent;
use crate::client::NotmuchClient;
use crate::config::UserConfig;
use crate::mail_sender::{MailSender, MessageBuilder};
use crate::search::SearchItem;
use askama_axum::{IntoResponse, Template};
use axum::{
    Form, Json, Router,
    extract::{Path, Query, State},
    http::{HeaderMap, StatusCode, header},
    response::Redirect,
    routing::{get, post},
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use std::sync::Arc;
use tower_http::services::ServeDir;

pub mod content_renderer;
use content_renderer::{RenderedContent, render_message_content};

#[derive(Clone)]
pub struct AppState {
    pub client: Arc<dyn NotmuchClient>,
    pub mail_sender: Option<Arc<dyn MailSender>>,
    pub config: WebConfig,
    pub user_config: UserConfig,
}

#[derive(Clone)]
pub struct WebConfig {
    pub bind_address: SocketAddr,
    pub base_url: String,
    pub items_per_page: usize,
}

#[derive(Template)]
#[template(path = "inbox.html")]
struct InboxTemplate {
    messages: Vec<SearchItem>,
    theme: String,
    active_tags: Vec<String>,
    search_query: Option<String>,
}

pub fn create_app(state: AppState) -> Router {
    Router::new()
        .route("/", get(index_handler))
        .route("/inbox", get(inbox_handler))
        .route("/search", get(search_handler))
        .route("/thread/:id", get(thread_handler))
        .route(
            "/attachment/:thread_id/:message_index/:part_id",
            get(attachment_handler),
        )
        .route(
            "/email-frame/:thread_id/:message_index",
            get(email_frame_handler),
        )
        .route("/image_proxy", get(image_proxy_handler))
        .route("/redirect", get(redirect_handler))
        .route("/tags", get(tags_handler))
        .route("/settings", get(settings_handler))
        .route("/settings/theme", post(toggle_theme_handler))
        .route("/api/log-redirect", post(log_redirect_handler))
        .route("/test/email-gallery", get(test_email_gallery_handler))
        .route(
            "/test/email-gallery/:email_name",
            get(test_email_viewer_handler),
        )
        .route("/compose", get(compose_get_handler).post(compose_post_handler))
        .route("/thread/:id/reply", get(reply_get_handler).post(reply_post_handler))
        .route("/thread/:id/forward", get(forward_get_handler).post(forward_post_handler))
        .nest_service("/static", ServeDir::new("src/web/static"))
        .with_state(state)
}

async fn index_handler() -> Redirect {
    Redirect::to("/inbox")
}

fn get_theme_from_headers(headers: &HeaderMap) -> String {
    headers
        .get(header::COOKIE)
        .and_then(|value| value.to_str().ok())
        .and_then(|cookies| {
            cookies
                .split(';')
                .find(|cookie| cookie.trim().starts_with("theme="))
                .and_then(|cookie| cookie.split('=').nth(1))
        })
        .unwrap_or("light")
        .to_string()
}

async fn inbox_handler(State(state): State<AppState>, headers: HeaderMap) -> impl IntoResponse {
    // Search for messages tagged with "inbox"
    let messages = match state.client.search("tag:inbox").await {
        Ok(results) => {
            tracing::info!("Found {} messages in inbox", results.len());
            results
        }
        Err(e) => {
            tracing::error!("Failed to search inbox: {}", e);
            vec![]
        }
    };

    let theme = get_theme_from_headers(&headers);

    InboxTemplate {
        messages,
        theme,
        active_tags: vec![],
        search_query: None,
    }
}

#[derive(Template)]
#[template(path = "settings.html")]
struct SettingsTemplate {
    theme: String,
}

async fn settings_handler(headers: HeaderMap) -> impl IntoResponse {
    let theme = get_theme_from_headers(&headers);
    SettingsTemplate { theme }
}

async fn toggle_theme_handler(headers: HeaderMap) -> impl IntoResponse {
    let current_theme = get_theme_from_headers(&headers);
    let new_theme = if current_theme == "dark" {
        "light"
    } else {
        "dark"
    };

    let cookie = format!("theme={}; Path=/; Max-Age=31536000", new_theme);

    let mut response_headers = HeaderMap::new();
    response_headers.insert(header::LOCATION, "/inbox".parse().unwrap());
    response_headers.insert(header::SET_COOKIE, cookie.parse().unwrap());

    (StatusCode::SEE_OTHER, response_headers)
}

#[derive(Deserialize)]
struct SearchParams {
    q: Option<String>,
    tag: Option<String>,
    tags: Option<Vec<String>>,
}

async fn search_handler(
    State(state): State<AppState>,
    Query(params): Query<SearchParams>,
    headers: HeaderMap,
) -> impl IntoResponse {
    // Build search query
    let mut query_parts = vec![];

    // Add text search query
    if let Some(ref q) = params.q {
        if !q.is_empty() {
            query_parts.push(q.clone());
        }
    }

    // Add single tag filter (for backward compatibility)
    if let Some(ref tag) = params.tag {
        if !tag.is_empty() {
            query_parts.push(format!("tag:{}", tag));
        }
    }

    // Add multiple tag filters
    if let Some(ref tags) = params.tags {
        for tag in tags {
            if !tag.is_empty() {
                query_parts.push(format!("tag:{}", tag));
            }
        }
    }

    let query = if query_parts.is_empty() {
        "tag:inbox".to_string()
    } else {
        query_parts.join(" AND ")
    };

    let messages = match state.client.search(&query).await {
        Ok(results) => {
            tracing::info!(
                "Search query '{}' returned {} results",
                query,
                results.len()
            );
            results
        }
        Err(e) => {
            tracing::error!("Failed to search: {}", e);
            vec![]
        }
    };

    let theme = get_theme_from_headers(&headers);

    // Extract active tags from query
    let mut active_tags = vec![];
    let mut search_query = None;

    if let Some(q) = &params.q {
        if !q.is_empty() {
            search_query = Some(q.clone());
        }
    }

    if let Some(tag) = &params.tag {
        if !tag.is_empty() {
            active_tags.push(tag.clone());
        }
    }

    if let Some(tags) = &params.tags {
        for tag in tags {
            if !tag.is_empty() {
                active_tags.push(tag.clone());
            }
        }
    }

    InboxTemplate {
        messages,
        theme,
        active_tags,
        search_query,
    }
}

use crate::thread::Message;

#[derive(Clone)]
struct MessageWithContent {
    message: Message,
    rendered_content: RenderedContent,
    thread_id: String,
    message_index: usize,
}

async fn thread_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
    headers: HeaderMap,
) -> impl IntoResponse {
    match state.client.show(&format!("thread:{}", id)).await {
        Ok(thread) => {
            let messages = thread.get_messages();
            tracing::info!("Loaded thread {} with {} messages", id, messages.len());
            let theme = get_theme_from_headers(&headers);

            // Process messages to include rendered content
            let messages_with_content: Vec<MessageWithContent> = messages
                .into_iter()
                .enumerate()
                .map(|(idx, msg)| {
                    let rendered_content = render_message_content(msg);
                    MessageWithContent {
                        message: msg.clone(),
                        rendered_content,
                        thread_id: id.clone(),
                        message_index: idx,
                    }
                })
                .collect();

            ThreadView {
                messages: messages_with_content,
                theme,
            }
            .into_response()
        }
        Err(e) => {
            tracing::error!("Failed to load thread {}: {}", id, e);
            ThreadErrorTemplate {
                message: "Thread not found".to_string(),
                theme: get_theme_from_headers(&headers),
            }
            .into_response()
        }
    }
}

#[derive(Template)]
#[template(path = "thread_simple.html")]
struct ThreadView {
    messages: Vec<MessageWithContent>,
    theme: String,
}

#[derive(Template)]
#[template(path = "error.html")]
struct ThreadErrorTemplate {
    message: String,
    theme: String,
}

#[derive(Serialize)]
struct TagsResponse {
    tags: Vec<String>,
}

async fn tags_handler(State(state): State<AppState>) -> impl IntoResponse {
    tracing::debug!("Tags endpoint called");
    match state.client.list_tags().await {
        Ok(tags) => {
            tracing::info!("Retrieved {} tags from backend", tags.len());
            if tags.is_empty() {
                tracing::warn!(
                    "No tags found - this might indicate an empty database or connection issue"
                );
            } else {
                tracing::debug!("Tags: {:?}", tags);
            }
            Json(TagsResponse { tags })
        }
        Err(e) => {
            tracing::error!("Failed to list tags: {}", e);
            Json(TagsResponse { tags: vec![] })
        }
    }
}

async fn attachment_handler(
    State(state): State<AppState>,
    Path((thread_id, message_index, part_id)): Path<(String, usize, u32)>,
) -> impl IntoResponse {
    tracing::info!(
        "Attachment request: thread={}, msg={}, part={}",
        thread_id,
        message_index,
        part_id
    );

    // Fetch the thread
    match state.client.show(&format!("thread:{}", thread_id)).await {
        Ok(thread) => {
            let messages = thread.get_messages();
            tracing::info!("Found thread with {} messages", messages.len());

            // Get the specific message
            if let Some(message) = messages.get(message_index) {
                tracing::info!("Found message {} at index {}", message.id, message_index);
                // Find the attachment by part ID to get metadata
                if let Some(attachment) = find_attachment_by_id(message, part_id) {
                    tracing::info!(
                        "Found attachment: id={}, type={}, filename={:?}",
                        attachment.id,
                        attachment.content_type,
                        attachment.filename
                    );

                    // Use notmuch part command to get raw content instead of relying on show output
                    let message_spec = format!("id:{}", message.id);
                    tracing::info!(
                        "Attempting to extract part {} from message {}",
                        part_id,
                        message_spec
                    );
                    let content = match state.client.part(&message_spec, part_id).await {
                        Ok(raw_content) => {
                            tracing::info!(
                                "Successfully extracted {} bytes for part {}",
                                raw_content.len(),
                                part_id
                            );
                            raw_content
                        }
                        Err(e) => {
                            tracing::error!(
                                "Failed to extract part {} from message {}: {}",
                                part_id,
                                message_spec,
                                e
                            );
                            return (
                                StatusCode::INTERNAL_SERVER_ERROR,
                                format!("Failed to extract attachment content: {}", e),
                            )
                                .into_response();
                        }
                    };

                    // Determine filename
                    let filename = attachment
                        .filename
                        .clone()
                        .unwrap_or_else(|| format!("attachment_{}", part_id));

                    // Sanitize filename to prevent path traversal
                    let safe_filename = sanitize_filename(&filename);

                    // Build response with appropriate headers
                    let mut headers = HeaderMap::new();

                    // Set content type
                    if let Ok(content_type) = attachment.content_type.parse::<mime::Mime>() {
                        headers
                            .insert(header::CONTENT_TYPE, content_type.as_ref().parse().unwrap());
                    }

                    // Set content disposition
                    let disposition = format!("attachment; filename=\"{}\"", safe_filename);
                    headers.insert(header::CONTENT_DISPOSITION, disposition.parse().unwrap());

                    // Add security headers
                    headers.insert("X-Content-Type-Options", "nosniff".parse().unwrap());
                    headers.insert("X-Frame-Options", "DENY".parse().unwrap());

                    return (headers, content).into_response();
                } else {
                    tracing::warn!("Attachment with part_id {} not found in message", part_id);
                }
            } else {
                tracing::warn!("Message at index {} not found in thread", message_index);
            }

            (StatusCode::NOT_FOUND, "Attachment not found").into_response()
        }
        Err(e) => {
            tracing::error!("Failed to load thread for attachment: {}", e);
            (StatusCode::NOT_FOUND, "Thread not found").into_response()
        }
    }
}

#[derive(Deserialize)]
struct EmailFrameParams {
    #[serde(default)]
    show_images: bool,
    #[serde(default)]
    theme: Option<String>,
}

async fn email_frame_handler(
    State(state): State<AppState>,
    Path((thread_id, message_index)): Path<(String, usize)>,
    Query(params): Query<EmailFrameParams>,
    _headers: HeaderMap,
) -> impl IntoResponse {
    tracing::info!(
        "Email frame request: thread={}, msg={}",
        thread_id,
        message_index
    );

    // Fetch the thread
    match state.client.show(&format!("thread:{}", thread_id)).await {
        Ok(thread) => {
            let messages = thread.get_messages();

            // Get the specific message
            if let Some(message) = messages.get(message_index) {
                // Render content with URL rewriting and image control for iframe
                let rendered_content = content_renderer::render_message_content_with_image_control(
                    message,
                    params.show_images,
                );

                // Determine theme colors
                let theme = params.theme.as_deref().unwrap_or("light");
                let (bg_color, text_color, link_color) = match theme {
                    "dark" => ("#0d1117", "#c9d1d9", "#58a6ff"),
                    _ => ("#ffffff", "#24292e", "#0366d6"),
                };

                // Create a minimal HTML document for the iframe with theme-aware styling
                let frame_html = format!(
                    r#"<!DOCTYPE html>
<html data-theme="{}">
<head>
    <meta charset="utf-8">
    <meta name="viewport" content="width=device-width, initial-scale=1">
    <style>
        :root {{
            --bg-primary: {};
            --text-primary: {};
            --text-link: {};
        }}
        
        body {{
            margin: 0;
            padding: 16px;
            font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Helvetica, Arial, sans-serif;
            line-height: 1.5;
            color: var(--text-primary);
            background-color: var(--bg-primary);
        }}
        
        img {{
            max-width: 100%;
            height: auto;
        }}
        
        table {{
            max-width: 100%;
        }}
        
        a {{
            color: var(--text-link);
        }}
        
        /* Ensure text is always visible by providing fallback styles for common email patterns */
        div, p, span, td, th {{
            color: inherit;
        }}
        
        /* Dark mode specific overrides for better email compatibility */
        [data-theme="dark"] {{
            color-scheme: dark;
        }}
        
        [data-theme="dark"] table {{
            color: var(--text-primary);
        }}
        
        /* Override very dark text colors that are invisible in dark mode */
        [data-theme="dark"] *[style*="color: #000"],
        [data-theme="dark"] *[style*="color:#000"],
        [data-theme="dark"] *[style*="color: black"],
        [data-theme="dark"] *[style*="color: rgb(0, 0, 0)"],
        [data-theme="dark"] *[style*="color:rgb(0, 0, 0)"],
        [data-theme="dark"] *[style*="color: rgba(0, 0, 0"],
        [data-theme="dark"] *[style*="color:rgba(0, 0, 0"],
        /* Very dark grays - common in newsletters */
        [data-theme="dark"] *[style*="color: rgb(25, 25, 25)"],
        [data-theme="dark"] *[style*="color:rgb(25, 25, 25)"],
        [data-theme="dark"] *[style*="color: rgba(25, 25, 25"],
        [data-theme="dark"] *[style*="color:rgba(25, 25, 25"],
        [data-theme="dark"] *[style*="color: rgb(41, 41, 41)"],
        [data-theme="dark"] *[style*="color:rgb(41, 41, 41)"],
        [data-theme="dark"] *[style*="color: rgba(41, 41, 41"],
        [data-theme="dark"] *[style*="color:rgba(41, 41, 41"],
        [data-theme="dark"] *[style*="color: rgb(51, 51, 50)"],
        [data-theme="dark"] *[style*="color:rgb(51, 51, 50)"],
        [data-theme="dark"] *[style*="color: rgba(51, 51, 50"],
        [data-theme="dark"] *[style*="color:rgba(51, 51, 50"],
        /* Dark hex colors */
        [data-theme="dark"] *[style*="color: #191919"],
        [data-theme="dark"] *[style*="color:#191919"],
        [data-theme="dark"] *[style*="color: #292929"],
        [data-theme="dark"] *[style*="color:#292929"],
        [data-theme="dark"] *[style*="color: #333"],
        [data-theme="dark"] *[style*="color:#333"],
        [data-theme="dark"] *[style*="color: #333333"],
        [data-theme="dark"] *[style*="color:#333333"] {{
            color: var(--text-primary) !important;
        }}
        
        /* Override light backgrounds that make text invisible in dark mode */
        [data-theme="dark"] *[style*="background-color: #fff"],
        [data-theme="dark"] *[style*="background-color:#fff"],
        [data-theme="dark"] *[style*="background-color: white"],
        [data-theme="dark"] *[style*="background-color: rgb(255, 255, 255)"],
        [data-theme="dark"] *[style*="background-color:rgb(255, 255, 255)"],
        [data-theme="dark"] *[style*="background-color: rgba(255, 255, 255"],
        [data-theme="dark"] *[style*="background-color:rgba(255, 255, 255"] {{
            background-color: var(--bg-primary) !important;
        }}
        
        /* Enhance contrast for common email elements in dark mode */
        [data-theme="dark"] h1, [data-theme="dark"] h2, [data-theme="dark"] h3, 
        [data-theme="dark"] h4, [data-theme="dark"] h5, [data-theme="dark"] h6 {{
            color: var(--text-primary) !important;
        }}
        
        [data-theme="dark"] p, [data-theme="dark"] div, [data-theme="dark"] span,
        [data-theme="dark"] td, [data-theme="dark"] th, [data-theme="dark"] li {{
            color: inherit !important;
        }}
        
        /* Override any background that might interfere with dark mode */
        [data-theme="dark"] body {{
            background-color: var(--bg-primary) !important;
            color: var(--text-primary) !important;
        }}
    </style>
</head>
<body>
    {}
</body>
</html>"#,
                    theme,
                    bg_color,
                    text_color,
                    link_color,
                    rendered_content
                        .html
                        .unwrap_or_else(|| rendered_content.plain.unwrap_or_default())
                );

                // Build response with CSP headers
                let mut response_headers = HeaderMap::new();
                response_headers.insert(
                    header::CONTENT_TYPE,
                    "text/html; charset=utf-8".parse().unwrap(),
                );
                response_headers.insert(
                    "Content-Security-Policy",
                    "script-src 'none'; img-src 'self' data:; style-src 'unsafe-inline'; frame-ancestors 'self'".parse().unwrap()
                );
                response_headers.insert("X-Frame-Options", "SAMEORIGIN".parse().unwrap());

                return (response_headers, frame_html).into_response();
            }

            (StatusCode::NOT_FOUND, "Message not found").into_response()
        }
        Err(e) => {
            tracing::error!("Failed to load thread for email frame: {}", e);
            (StatusCode::NOT_FOUND, "Thread not found").into_response()
        }
    }
}

#[derive(Deserialize)]
struct ProxyParams {
    url: String,
    #[serde(default)]
    blocked: bool,
}

async fn image_proxy_handler(Query(params): Query<ProxyParams>) -> impl IntoResponse {
    let url = &params.url;

    // Validate URL (only allow http and https)
    if !url.starts_with("http://") && !url.starts_with("https://") {
        return (StatusCode::BAD_REQUEST, "Invalid URL scheme").into_response();
    }

    // If images are blocked, return placeholder
    if params.blocked {
        tracing::info!("Image blocked for: {}", url);
        return generate_blocked_image_placeholder(url).into_response();
    }

    tracing::info!("Image proxy request for: {}", url);

    // Fetch the image server-side
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(10))
        .user_agent("whynot-email-client/1.0")
        .build()
        .unwrap();

    match client.get(url).send().await {
        Ok(response) => {
            if !response.status().is_success() {
                return (StatusCode::BAD_GATEWAY, "Failed to fetch image").into_response();
            }

            // Get content type, default to octet-stream if not specified
            let content_type = response
                .headers()
                .get("content-type")
                .and_then(|ct| ct.to_str().ok())
                .unwrap_or("application/octet-stream")
                .to_string();

            // Validate it's actually an image content type
            if !content_type.starts_with("image/") {
                tracing::warn!("Non-image content type {} for URL: {}", content_type, url);
                return (StatusCode::BAD_REQUEST, "URL does not serve image content")
                    .into_response();
            }

            match response.bytes().await {
                Ok(body) => {
                    let mut headers = HeaderMap::new();
                    headers.insert(header::CONTENT_TYPE, content_type.parse().unwrap());
                    headers.insert("Cache-Control", "public, max-age=3600".parse().unwrap());
                    headers.insert("X-Content-Type-Options", "nosniff".parse().unwrap());

                    (headers, body).into_response()
                }
                Err(e) => {
                    tracing::error!("Failed to read image body: {}", e);
                    (StatusCode::BAD_GATEWAY, "Failed to read image content").into_response()
                }
            }
        }
        Err(e) => {
            tracing::error!("Failed to fetch image from {}: {}", url, e);
            (StatusCode::BAD_GATEWAY, "Failed to fetch image").into_response()
        }
    }
}

#[derive(Template)]
#[template(path = "redirect_warning.html")]
struct RedirectWarningTemplate {
    url: String,
    domain: String,
    theme: String,
}

async fn redirect_handler(
    Query(params): Query<ProxyParams>,
    headers: HeaderMap,
) -> impl IntoResponse {
    let url = &params.url;

    // Validate URL scheme
    if !url.starts_with("http://") && !url.starts_with("https://") {
        return (StatusCode::BAD_REQUEST, "Invalid URL scheme").into_response();
    }

    // Extract domain from URL for display and security analysis
    let domain = url
        .split("://")
        .nth(1)
        .and_then(|part| part.split('/').next())
        .unwrap_or("unknown")
        .to_string();

    tracing::info!("Redirect request for: {} (domain: {})", url, domain);

    // Basic phishing protection - check for suspicious patterns
    let suspicious_patterns = [
        "secure-bank-login",
        "paypal-verify",
        "amazon-security",
        "microsoft-login",
        "google-verify",
        "apple-id-verify",
    ];

    let is_suspicious = suspicious_patterns
        .iter()
        .any(|pattern| domain.to_lowercase().contains(pattern));

    if is_suspicious {
        tracing::warn!("Suspicious domain detected: {}", domain);
        // Could implement additional security measures here
    }

    // Check for known safe domains that can bypass the warning
    let safe_domains = [
        "github.com",
        "stackoverflow.com",
        "docs.rs",
        "crates.io",
        "rust-lang.org",
    ];

    let is_safe_domain = safe_domains.iter().any(|safe| domain.ends_with(safe));

    // For safe domains, redirect directly but still log
    if is_safe_domain {
        tracing::info!("Safe domain redirect: {}", domain);
        return Redirect::temporary(url).into_response();
    }

    // Show warning page for all other external links
    let theme = get_theme_from_headers(&headers);

    RedirectWarningTemplate {
        url: url.to_string(),
        domain,
        theme,
    }
    .into_response()
}

#[derive(Deserialize)]
struct RedirectLogEntry {
    url: String,
    domain: String,
    timestamp: String,
    user_confirmed: bool,
}

async fn log_redirect_handler(Json(log_entry): Json<RedirectLogEntry>) -> impl IntoResponse {
    // Log the redirect for security monitoring
    tracing::info!(
        "User confirmed redirect: url={}, domain={}, timestamp={}, confirmed={}",
        log_entry.url,
        log_entry.domain,
        log_entry.timestamp,
        log_entry.user_confirmed
    );

    // In a production system, you might want to:
    // 1. Store this in a database for security analysis
    // 2. Check against threat intelligence feeds
    // 3. Rate limit suspicious activity
    // 4. Send alerts for high-risk domains

    (StatusCode::OK, "Logged").into_response()
}

/// Generate a placeholder image for blocked external images
fn generate_blocked_image_placeholder(original_url: &str) -> (HeaderMap, Vec<u8>) {
    // Create a simple SVG placeholder image
    let domain = original_url.split("/").nth(2).unwrap_or("external site");

    let svg_content = format!(
        "<svg width=\"200\" height=\"100\" xmlns=\"http://www.w3.org/2000/svg\">\
            <rect width=\"200\" height=\"100\" fill=\"#f0f0f0\" stroke=\"#ccc\" stroke-width=\"1\"/>\
            <text x=\"100\" y=\"40\" text-anchor=\"middle\" font-family=\"Arial\" font-size=\"12\" fill=\"#666\">\
                Image blocked\
            </text>\
            <text x=\"100\" y=\"60\" text-anchor=\"middle\" font-family=\"Arial\" font-size=\"10\" fill=\"#999\">\
                from {}\
            </text>\
            <text x=\"100\" y=\"80\" text-anchor=\"middle\" font-family=\"Arial\" font-size=\"8\" fill=\"#aaa\">\
                Click Show Images to load\
            </text>\
        </svg>",
        domain
    );

    let mut headers = HeaderMap::new();
    headers.insert(header::CONTENT_TYPE, "image/svg+xml".parse().unwrap());
    headers.insert(
        "Cache-Control",
        "no-cache, no-store, must-revalidate".parse().unwrap(),
    );
    headers.insert("X-Image-Blocked", "true".parse().unwrap());
    headers.insert(
        "X-Original-URL",
        original_url
            .parse()
            .unwrap_or_else(|_| "invalid".parse().unwrap()),
    );

    (headers, svg_content.into_bytes())
}

// Test email gallery handlers
#[derive(Template)]
#[template(path = "test_email_gallery.html")]
struct TestEmailGalleryTemplate {
    emails: Vec<TestEmailInfo>,
    theme: String,
}

#[derive(Clone)]
struct TestEmailInfo {
    name: String,
    display_name: String,
    description: String,
}

async fn test_email_gallery_handler(headers: HeaderMap) -> impl IntoResponse {
    let emails = vec![
        TestEmailInfo {
            name: "bilprovningen".to_string(),
            display_name: "Bilprovningen".to_string(),
            description: "Service notification with circular profile image and layout issues"
                .to_string(),
        },
        TestEmailInfo {
            name: "stockholm-film-festival".to_string(),
            display_name: "Stockholm Film Festival".to_string(),
            description: "Newsletter with complex multi-column layout breaking".to_string(),
        },
        TestEmailInfo {
            name: "max-dead-rising".to_string(),
            display_name: "Max Dead Rising".to_string(),
            description: "Promotional email with hero images and responsive design issues"
                .to_string(),
        },
        TestEmailInfo {
            name: "rubygems-notice".to_string(),
            display_name: "RubyGems Notice".to_string(),
            description: "Policy update email with text rendering issues in iframe".to_string(),
        },
        TestEmailInfo {
            name: "medium-article".to_string(),
            display_name: "Medium Newsletter".to_string(),
            description: "Medium newsletter with dark text that's invisible in dark mode"
                .to_string(),
        },
    ];

    let theme = get_theme_from_headers(&headers);

    TestEmailGalleryTemplate { emails, theme }
}

#[derive(Template)]
#[template(path = "test_email_viewer_simple.html")]
struct TestEmailViewerTemplate {
    email_name: String,
    email_content: RenderedContent,
    theme: String,
}

async fn test_email_viewer_handler(
    Path(email_name): Path<String>,
    Query(_params): Query<ViewerParams>,
    headers: HeaderMap,
) -> impl IntoResponse {
    // Load the email JSON from the examples directory
    let email_path = format!("examples/problematic-emails/{}.json", email_name);

    match std::fs::read_to_string(&email_path) {
        Ok(json_content) => {
            // Parse the notmuch JSON format - expecting [[[message], []]]
            match serde_json::from_str::<crate::thread::Thread>(&json_content) {
                Ok(thread) => {
                    // Get the first message from the thread using the Thread API
                    let messages = thread.get_messages();
                    if let Some(message) = messages.first() {
                        let rendered_content = render_message_content(message);
                        let theme = get_theme_from_headers(&headers);

                        TestEmailViewerTemplate {
                            email_name,
                            email_content: rendered_content,
                            theme,
                        }
                        .into_response()
                    } else {
                        (StatusCode::NOT_FOUND, "No messages in thread").into_response()
                    }
                }
                Err(e) => {
                    tracing::error!("Failed to parse email JSON: {}", e);
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        format!("Failed to parse email: {}", e),
                    )
                        .into_response()
                }
            }
        }
        Err(e) => {
            tracing::error!("Failed to read email file {}: {}", email_path, e);
            (
                StatusCode::NOT_FOUND,
                format!("Email not found: {}", email_name),
            )
                .into_response()
        }
    }
}

#[derive(Deserialize)]
struct ViewerParams {
    #[allow(dead_code)]
    mode: Option<String>,
}

fn find_attachment_by_id(
    message: &crate::thread::Message,
    part_id: u32,
) -> Option<&crate::body::BodyPart> {
    fn search_parts(
        parts: &[crate::body::BodyPart],
        target_id: u32,
    ) -> Option<&crate::body::BodyPart> {
        for part in parts {
            if part.id == target_id {
                return Some(part);
            }
            if let BodyContent::Multipart(nested) = &part.content {
                if let Some(found) = search_parts(nested, target_id) {
                    return Some(found);
                }
            }
        }
        None
    }

    search_parts(&message.body, part_id)
}

fn sanitize_filename(filename: &str) -> String {
    // Remove any path components and dangerous characters
    filename
        .chars()
        .filter(|c| c.is_alphanumeric() || matches!(c, '.' | '-' | '_'))
        .collect::<String>()
        .split('/')
        .next_back()
        .unwrap_or("attachment")
        .to_string()
}

// Compose/Reply/Forward structures and handlers
#[derive(Template)]
#[template(path = "compose.html")]
struct ComposeTemplate {
    title: String,
    action_url: String,
    back_url: String,
    mode: String,
    to: String,
    cc: String,
    bcc: String,
    subject: String,
    body: String,
    in_reply_to: String,
    references: String,
    original_message_id: String,
    error: Option<String>,
    theme: String,
}

#[derive(Deserialize)]
struct ComposeFormData {
    to: String,
    cc: Option<String>,
    bcc: Option<String>,
    subject: String,
    body: String,
    in_reply_to: Option<String>,
    references: Option<String>,
    original_message_id: Option<String>,
}

async fn compose_get_handler(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> impl IntoResponse {
    let theme = get_theme_from_headers(&headers);
    
    // Check if mail sending is configured
    if state.mail_sender.is_none() {
        return ThreadErrorTemplate {
            message: "Mail sending is not configured. Please configure msmtp in your settings.".to_string(),
            theme,
        }
        .into_response();
    }
    
    // Get default from address (currently unused but may be displayed in UI later)
    let _from_email = state.user_config.email.as_deref().unwrap_or("user@example.com");
    
    ComposeTemplate {
        title: "Compose New Email".to_string(),
        action_url: "/compose".to_string(),
        back_url: "/inbox".to_string(),
        mode: "compose".to_string(),
        to: "".to_string(),
        cc: "".to_string(),
        bcc: "".to_string(),
        subject: "".to_string(),
        body: format!("\n\n--\n{}", state.user_config.signature.as_deref().unwrap_or("")),
        in_reply_to: "".to_string(),
        references: "".to_string(),
        original_message_id: "".to_string(),
        error: None,
        theme,
    }
    .into_response()
}

async fn compose_post_handler(
    State(state): State<AppState>,
    headers: HeaderMap,
    Form(form_data): Form<ComposeFormData>,
) -> impl IntoResponse {
    let theme = get_theme_from_headers(&headers);
    
    // Check if mail sending is configured
    let mail_sender = match &state.mail_sender {
        Some(sender) => sender,
        None => {
            return ThreadErrorTemplate {
                message: "Mail sending is not configured.".to_string(),
                theme,
            }
            .into_response();
        }
    };
    
    // Build the message
    let mut builder = MessageBuilder::new()
        .to(form_data.to.clone())
        .subject(form_data.subject.clone())
        .body(form_data.body.clone());
    
    // Add optional fields
    if let Some(from_email) = &state.user_config.email {
        builder = builder.from(from_email.clone());
    }
    
    if let Some(cc) = form_data.cc.as_ref() {
        if !cc.is_empty() {
            builder = builder.cc(cc.clone());
        }
    }
    
    if let Some(bcc) = form_data.bcc.as_ref() {
        if !bcc.is_empty() {
            builder = builder.bcc(bcc.clone());
        }
    }
    
    // Set reply headers if present
    if let Some(in_reply_to) = form_data.in_reply_to.as_ref() {
        if !in_reply_to.is_empty() {
            builder = builder.in_reply_to(in_reply_to.clone());
        }
    }
    
    if let Some(references) = form_data.references.as_ref() {
        if !references.is_empty() {
            // Split references and add each one
            for reference in references.split_whitespace() {
                builder = builder.add_reference(reference.to_string());
            }
        }
    }
    
    // Build and send the message
    match builder.build() {
        Ok(message) => {
            match mail_sender.send(message).await {
                Ok(message_id) => {
                    tracing::info!("Successfully sent email with ID: {}", message_id);
                    // Redirect to inbox with success message
                    // TODO: Add flash message support for success notification
                    Redirect::to("/inbox").into_response()
                }
                Err(e) => {
                    tracing::error!("Failed to send email: {}", e);
                    ComposeTemplate {
                        title: "Compose New Email".to_string(),
                        action_url: "/compose".to_string(),
                        back_url: "/inbox".to_string(),
                        mode: "compose".to_string(),
                        to: form_data.to,
                        cc: form_data.cc.unwrap_or_default(),
                        bcc: form_data.bcc.unwrap_or_default(),
                        subject: form_data.subject,
                        body: form_data.body,
                        in_reply_to: form_data.in_reply_to.unwrap_or_default(),
                        references: form_data.references.unwrap_or_default(),
                        original_message_id: form_data.original_message_id.unwrap_or_default(),
                        error: Some(format!("Failed to send email: {}", e)),
                        theme,
                    }
                    .into_response()
                }
            }
        }
        Err(e) => {
            tracing::error!("Failed to build email: {}", e);
            ComposeTemplate {
                title: "Compose New Email".to_string(),
                action_url: "/compose".to_string(),
                back_url: "/inbox".to_string(),
                mode: "compose".to_string(),
                to: form_data.to,
                cc: form_data.cc.unwrap_or_default(),
                bcc: form_data.bcc.unwrap_or_default(),
                subject: form_data.subject,
                body: form_data.body,
                in_reply_to: form_data.in_reply_to.unwrap_or_default(),
                references: form_data.references.unwrap_or_default(),
                original_message_id: form_data.original_message_id.unwrap_or_default(),
                error: Some(format!("Failed to build email: {}", e)),
                theme,
            }
            .into_response()
        }
    }
}

#[derive(Deserialize)]
struct ReplyParams {
    message: usize,
    all: Option<bool>,
}

async fn reply_get_handler(
    State(state): State<AppState>,
    Path(thread_id): Path<String>,
    Query(params): Query<ReplyParams>,
    headers: HeaderMap,
) -> impl IntoResponse {
    let theme = get_theme_from_headers(&headers);
    
    // Check if mail sending is configured
    if state.mail_sender.is_none() {
        return ThreadErrorTemplate {
            message: "Mail sending is not configured. Please configure msmtp in your settings.".to_string(),
            theme,
        }
        .into_response();
    }
    
    // Fetch the thread
    match state.client.show(&format!("thread:{}", thread_id)).await {
        Ok(thread) => {
            let messages = thread.get_messages();
            
            // Get the specific message to reply to
            if let Some(original_message) = messages.get(params.message) {
                let reply_all = params.all.unwrap_or(false);
                
                // Determine recipients
                let to = original_message.headers.from.clone();
                let mut cc = String::new();
                
                if reply_all {
                    // Add original To recipients to CC
                    cc = original_message.headers.to.clone();
                    
                    // CC field is not available in headers, skip for now
                    // TODO: Parse CC from raw message headers if needed
                    
                    // Remove self from CC if present
                    if let Some(user_email) = &state.user_config.email {
                        cc = cc.split(',')
                            .map(|s| s.trim())
                            .filter(|email| !email.contains(user_email))
                            .collect::<Vec<_>>()
                            .join(", ");
                    }
                }
                
                // Prepare subject with Re: prefix if not already present
                let subject = if original_message.headers.subject.starts_with("Re: ") {
                    original_message.headers.subject.clone()
                } else {
                    format!("Re: {}", original_message.headers.subject)
                };
                
                // Build reply body with quoted original
                let quoted_body = original_message.get_text_content()
                    .unwrap_or_default()
                    .lines()
                    .map(|line| format!("> {}", line))
                    .collect::<Vec<_>>()
                    .join("\n");
                
                let body = format!(
                    "\n\nOn {}, {} wrote:\n{}\n\n--\n{}",
                    original_message.date_relative,
                    original_message.headers.from,
                    quoted_body,
                    state.user_config.signature.as_deref().unwrap_or("")
                );
                
                // Extract message ID for In-Reply-To
                let in_reply_to = original_message.headers.additional.get("message-id")
                    .cloned()
                    .unwrap_or_else(|| format!("<{}@unknown>", original_message.id));
                
                // Build references chain
                let mut references = Vec::new();
                if let Some(orig_refs) = original_message.headers.additional.get("references") {
                    references.push(orig_refs.clone());
                }
                references.push(in_reply_to.clone());
                let references_str = references.join(" ");
                
                let title = if reply_all {
                    "Reply All".to_string()
                } else {
                    "Reply".to_string()
                };
                
                ComposeTemplate {
                    title,
                    action_url: format!("/thread/{}/reply?message={}&all={}", thread_id, params.message, reply_all),
                    back_url: format!("/thread/{}", thread_id),
                    mode: if reply_all { "reply_all".to_string() } else { "reply".to_string() },
                    to,
                    cc,
                    bcc: "".to_string(),
                    subject,
                    body,
                    in_reply_to,
                    references: references_str,
                    original_message_id: "".to_string(),
                    error: None,
                    theme,
                }
                .into_response()
            } else {
                ThreadErrorTemplate {
                    message: "Message not found in thread".to_string(),
                    theme,
                }
                .into_response()
            }
        }
        Err(e) => {
            tracing::error!("Failed to load thread {}: {}", thread_id, e);
            ThreadErrorTemplate {
                message: format!("Failed to load thread: {}", e),
                theme,
            }
            .into_response()
        }
    }
}

async fn reply_post_handler(
    State(state): State<AppState>,
    Path(thread_id): Path<String>,
    Query(params): Query<ReplyParams>,
    headers: HeaderMap,
    Form(form_data): Form<ComposeFormData>,
) -> impl IntoResponse {
    let theme = get_theme_from_headers(&headers);
    
    // Check if mail sending is configured
    let mail_sender = match &state.mail_sender {
        Some(sender) => sender,
        None => {
            return ThreadErrorTemplate {
                message: "Mail sending is not configured.".to_string(),
                theme,
            }
            .into_response();
        }
    };
    
    // Build the reply message
    let mut builder = MessageBuilder::new()
        .to(form_data.to.clone())
        .subject(form_data.subject.clone())
        .body(form_data.body.clone());
    
    // Add optional fields
    if let Some(from_email) = &state.user_config.email {
        builder = builder.from(from_email.clone());
    }
    
    if let Some(cc) = form_data.cc.as_ref() {
        if !cc.is_empty() {
            builder = builder.cc(cc.clone());
        }
    }
    
    if let Some(bcc) = form_data.bcc.as_ref() {
        if !bcc.is_empty() {
            builder = builder.bcc(bcc.clone());
        }
    }
    
    // Set reply headers
    if let Some(in_reply_to) = form_data.in_reply_to.as_ref() {
        if !in_reply_to.is_empty() {
            builder = builder.in_reply_to(in_reply_to.clone());
        }
    }
    
    if let Some(references) = form_data.references.as_ref() {
        if !references.is_empty() {
            // Split references and add each one
            for reference in references.split_whitespace() {
                builder = builder.add_reference(reference.to_string());
            }
        }
    }
    
    // Build and send the message
    match builder.build() {
        Ok(message) => {
            match mail_sender.send(message).await {
                Ok(message_id) => {
                    tracing::info!("Successfully sent reply with ID: {}", message_id);
                    // Redirect back to the thread
                    Redirect::to(&format!("/thread/{}", thread_id)).into_response()
                }
                Err(e) => {
                    tracing::error!("Failed to send reply: {}", e);
                    let reply_all = params.all.unwrap_or(false);
                    ComposeTemplate {
                        title: if reply_all { "Reply All".to_string() } else { "Reply".to_string() },
                        action_url: format!("/thread/{}/reply?message={}&all={}", thread_id, params.message, reply_all),
                        back_url: format!("/thread/{}", thread_id),
                        mode: if reply_all { "reply_all".to_string() } else { "reply".to_string() },
                        to: form_data.to,
                        cc: form_data.cc.unwrap_or_default(),
                        bcc: form_data.bcc.unwrap_or_default(),
                        subject: form_data.subject,
                        body: form_data.body,
                        in_reply_to: form_data.in_reply_to.unwrap_or_default(),
                        references: form_data.references.unwrap_or_default(),
                        original_message_id: form_data.original_message_id.unwrap_or_default(),
                        error: Some(format!("Failed to send reply: {}", e)),
                        theme,
                    }
                    .into_response()
                }
            }
        }
        Err(e) => {
            tracing::error!("Failed to build reply: {}", e);
            let reply_all = params.all.unwrap_or(false);
            ComposeTemplate {
                title: if reply_all { "Reply All".to_string() } else { "Reply".to_string() },
                action_url: format!("/thread/{}/reply?message={}&all={}", thread_id, params.message, reply_all),
                back_url: format!("/thread/{}", thread_id),
                mode: if reply_all { "reply_all".to_string() } else { "reply".to_string() },
                to: form_data.to,
                cc: form_data.cc.unwrap_or_default(),
                bcc: form_data.bcc.unwrap_or_default(),
                subject: form_data.subject,
                body: form_data.body,
                in_reply_to: form_data.in_reply_to.unwrap_or_default(),
                references: form_data.references.unwrap_or_default(),
                original_message_id: form_data.original_message_id.unwrap_or_default(),
                error: Some(format!("Failed to build reply: {}", e)),
                theme,
            }
            .into_response()
        }
    }
}

#[derive(Deserialize)]
struct ForwardParams {
    message: usize,
}

async fn forward_get_handler(
    State(state): State<AppState>,
    Path(thread_id): Path<String>,
    Query(params): Query<ForwardParams>,
    headers: HeaderMap,
) -> impl IntoResponse {
    let theme = get_theme_from_headers(&headers);
    
    // Check if mail sending is configured
    if state.mail_sender.is_none() {
        return ThreadErrorTemplate {
            message: "Mail sending is not configured. Please configure msmtp in your settings.".to_string(),
            theme,
        }
        .into_response();
    }
    
    // Fetch the thread
    match state.client.show(&format!("thread:{}", thread_id)).await {
        Ok(thread) => {
            let messages = thread.get_messages();
            
            // Get the specific message to forward
            if let Some(original_message) = messages.get(params.message) {
                // Prepare subject with Fwd: prefix if not already present
                let subject = if original_message.headers.subject.starts_with("Fwd: ") {
                    original_message.headers.subject.clone()
                } else {
                    format!("Fwd: {}", original_message.headers.subject)
                };
                
                // Build forward body with original message
                let original_text = original_message.get_text_content()
                    .unwrap_or("[No text content]");
                
                let body = format!(
                    "\n\n---------- Forwarded message ----------\nFrom: {}\nDate: {}\nSubject: {}\nTo: {}\n\n{}\n\n--\n{}",
                    original_message.headers.from,
                    original_message.date_relative,
                    original_message.headers.subject,
                    original_message.headers.to,
                    original_text,
                    state.user_config.signature.as_deref().unwrap_or("")
                );
                
                // Store original message ID for attachment forwarding (future feature)
                let original_message_id = original_message.id.clone();
                
                ComposeTemplate {
                    title: "Forward Email".to_string(),
                    action_url: format!("/thread/{}/forward?message={}", thread_id, params.message),
                    back_url: format!("/thread/{}", thread_id),
                    mode: "forward".to_string(),
                    to: "".to_string(), // User needs to fill this in
                    cc: "".to_string(),
                    bcc: "".to_string(),
                    subject,
                    body,
                    in_reply_to: "".to_string(),
                    references: "".to_string(),
                    original_message_id,
                    error: None,
                    theme,
                }
                .into_response()
            } else {
                ThreadErrorTemplate {
                    message: "Message not found in thread".to_string(),
                    theme,
                }
                .into_response()
            }
        }
        Err(e) => {
            tracing::error!("Failed to load thread {}: {}", thread_id, e);
            ThreadErrorTemplate {
                message: format!("Failed to load thread: {}", e),
                theme,
            }
            .into_response()
        }
    }
}

async fn forward_post_handler(
    State(state): State<AppState>,
    Path(thread_id): Path<String>,
    Query(params): Query<ForwardParams>,
    headers: HeaderMap,
    Form(form_data): Form<ComposeFormData>,
) -> impl IntoResponse {
    let theme = get_theme_from_headers(&headers);
    
    // Check if mail sending is configured
    let mail_sender = match &state.mail_sender {
        Some(sender) => sender,
        None => {
            return ThreadErrorTemplate {
                message: "Mail sending is not configured.".to_string(),
                theme,
            }
            .into_response();
        }
    };
    
    // Build the forward message
    let mut builder = MessageBuilder::new()
        .to(form_data.to.clone())
        .subject(form_data.subject.clone())
        .body(form_data.body.clone());
    
    // Add optional fields
    if let Some(from_email) = &state.user_config.email {
        builder = builder.from(from_email.clone());
    }
    
    if let Some(cc) = form_data.cc.as_ref() {
        if !cc.is_empty() {
            builder = builder.cc(cc.clone());
        }
    }
    
    if let Some(bcc) = form_data.bcc.as_ref() {
        if !bcc.is_empty() {
            builder = builder.bcc(bcc.clone());
        }
    }
    
    // Build and send the message
    match builder.build() {
        Ok(message) => {
            match mail_sender.send(message).await {
                Ok(message_id) => {
                    tracing::info!("Successfully forwarded email with ID: {}", message_id);
                    // Redirect back to the thread
                    Redirect::to(&format!("/thread/{}", thread_id)).into_response()
                }
                Err(e) => {
                    tracing::error!("Failed to forward email: {}", e);
                    ComposeTemplate {
                        title: "Forward Email".to_string(),
                        action_url: format!("/thread/{}/forward?message={}", thread_id, params.message),
                        back_url: format!("/thread/{}", thread_id),
                        mode: "forward".to_string(),
                        to: form_data.to,
                        cc: form_data.cc.unwrap_or_default(),
                        bcc: form_data.bcc.unwrap_or_default(),
                        subject: form_data.subject,
                        body: form_data.body,
                        in_reply_to: form_data.in_reply_to.unwrap_or_default(),
                        references: form_data.references.unwrap_or_default(),
                        original_message_id: form_data.original_message_id.unwrap_or_default(),
                        error: Some(format!("Failed to forward email: {}", e)),
                        theme,
                    }
                    .into_response()
                }
            }
        }
        Err(e) => {
            tracing::error!("Failed to build forward message: {}", e);
            ComposeTemplate {
                title: "Forward Email".to_string(),
                action_url: format!("/thread/{}/forward?message={}", thread_id, params.message),
                back_url: format!("/thread/{}", thread_id),
                mode: "forward".to_string(),
                to: form_data.to,
                cc: form_data.cc.unwrap_or_default(),
                bcc: form_data.bcc.unwrap_or_default(),
                subject: form_data.subject,
                body: form_data.body,
                in_reply_to: form_data.in_reply_to.unwrap_or_default(),
                references: form_data.references.unwrap_or_default(),
                original_message_id: form_data.original_message_id.unwrap_or_default(),
                error: Some(format!("Failed to build forward message: {}", e)),
                theme,
            }
            .into_response()
        }
    }
}
