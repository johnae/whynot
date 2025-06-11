use crate::client::NotmuchClient;
use crate::error::NotmuchError;
use crate::search::SearchItem;
use crate::thread::{Message, Thread};
use crate::text_renderer::{HtmlToTextConverter, TextRendererConfig, TextRendererFactory};
use crate::mail_sender::{MailSender, ComposableMessage};
use std::sync::Arc;

#[derive(Debug, Default)]
pub enum AppState {
    #[default]
    EmailList,
    EmailView,
    Search,
    Compose,
    Help,
}

#[derive(Debug, Default)]
pub enum ComposeMode {
    #[default]
    New,
    Reply(String), // Thread ID
    ReplyAll(String), // Thread ID
    Forward(String), // Thread ID
}

#[derive(Debug, Default)]
pub enum ComposeField {
    #[default]
    To,
    Cc,
    Bcc,
    Subject,
    Body,
}

#[derive(Debug, Default)]
pub struct ComposeForm {
    pub mode: ComposeMode,
    pub to: String,
    pub cc: String,
    pub bcc: String,
    pub subject: String,
    pub body: String,
    pub current_field: ComposeField,
}

pub struct App {
    /// Current application state
    pub state: AppState,
    
    /// Should the application quit?
    pub should_quit: bool,
    
    /// Current search results/email list
    pub search_results: Vec<SearchItem>,
    
    /// Currently selected email index
    pub selected_email: usize,
    
    /// Current thread being viewed (if in EmailView state)
    pub current_thread: Option<Thread>,
    
    /// Index of current message within the thread
    pub current_message_index: usize,
    
    /// Current email being viewed (if in EmailView state)
    pub current_email: Option<Message>,
    
    /// Processed email body text (after HTML conversion)
    pub current_email_body: Option<String>,
    
    /// Current search query
    pub search_query: String,
    
    /// Search input buffer (for typing)
    pub search_input: String,
    
    /// Scroll position in various views
    pub scroll_position: usize,
    
    /// Status message to display
    pub status_message: Option<String>,
    
    /// Compose form data
    pub compose_form: ComposeForm,
    
    /// Notmuch client for data access
    client: Arc<dyn NotmuchClient>,
    
    /// HTML to text converter
    html_converter: Box<dyn HtmlToTextConverter>,
    
    /// Mail sender for sending emails (optional if not configured)
    mail_sender: Option<Box<dyn MailSender>>,
}

impl App {
    pub async fn new(client: Arc<dyn NotmuchClient>, mail_sender: Option<Box<dyn MailSender>>) -> Result<Self, Box<dyn std::error::Error>> {
        // Create HTML to text converter with default configuration
        let config = TextRendererConfig::default();
        let html_converter = TextRendererFactory::create_converter(&config).await?;
        
        Ok(Self {
            state: AppState::EmailList,
            should_quit: false,
            search_results: Vec::new(),
            selected_email: 0,
            current_thread: None,
            current_message_index: 0,
            current_email: None,
            current_email_body: None,
            search_query: String::new(),
            search_input: String::new(),
            scroll_position: 0,
            status_message: None,
            compose_form: ComposeForm::default(),
            client,
            html_converter,
            mail_sender,
        })
    }

    /// Initialize the app by loading the inbox
    pub async fn initialize(&mut self) -> Result<(), NotmuchError> {
        self.load_inbox().await
    }

    /// Load inbox messages
    pub async fn load_inbox(&mut self) -> Result<(), NotmuchError> {
        self.search_query = "tag:inbox".to_string();
        self.load_search_results().await
    }

    /// Load search results based on current query
    pub async fn load_search_results(&mut self) -> Result<(), NotmuchError> {
        let search_results = self.client.search(&self.search_query).await?;
        self.search_results = search_results;
        self.selected_email = 0;
        self.scroll_position = 0;
        self.state = AppState::EmailList;
        Ok(())
    }

    /// Handle navigation up
    pub fn navigate_up(&mut self) {
        if self.selected_email > 0 {
            self.selected_email -= 1;
        }
    }

    /// Handle navigation down
    pub fn navigate_down(&mut self) {
        if self.selected_email < self.search_results.len().saturating_sub(1) {
            self.selected_email += 1;
        }
    }

    /// Scroll email content up (decrease scroll position)
    pub fn scroll_up(&mut self) {
        if self.scroll_position > 0 {
            self.scroll_position -= 1;
        }
    }

    /// Scroll email content down (increase scroll position)
    pub fn scroll_down(&mut self) {
        self.scroll_position += 1;
    }

    /// Scroll email content up by page
    pub fn page_up(&mut self) {
        self.scroll_position = self.scroll_position.saturating_sub(10);
    }

    /// Scroll email content down by page
    pub fn page_down(&mut self) {
        self.scroll_position += 10;
    }

    /// Go to top of email content
    pub fn scroll_to_top(&mut self) {
        self.scroll_position = 0;
    }

    /// Go to bottom of email content (requires content height)
    pub fn scroll_to_bottom(&mut self, content_height: usize) {
        if content_height > 10 {
            self.scroll_position = content_height.saturating_sub(10);
        }
    }

    /// Open the currently selected email
    pub async fn open_selected_email(&mut self) -> Result<(), NotmuchError> {
        if let Some(search_item) = self.search_results.get(self.selected_email) {
            // Load the full thread to get all messages
            let thread = self.client.show(&search_item.thread).await?;
            
            // Store the thread and start with the first message
            self.current_thread = Some(thread);
            self.current_message_index = 0;
            
            // Load the first message
            self.load_current_message().await?;
                
            self.state = AppState::EmailView;
            self.scroll_position = 0;
        }
        Ok(())
    }

    /// Load the current message based on thread and message index
    async fn load_current_message(&mut self) -> Result<(), NotmuchError> {
        if let Some(ref thread) = self.current_thread {
            let messages = thread.get_messages();
            if let Some(&message) = messages.get(self.current_message_index) {
                // Process the email body content
                self.current_email_body = self.process_email_body(message).await;
                self.current_email = Some(message.clone());
                self.scroll_position = 0; // Reset scroll when switching messages
            }
        }
        Ok(())
    }

    /// Navigate to the next message in the current thread
    pub async fn next_message_in_thread(&mut self) -> Result<(), NotmuchError> {
        let can_advance = if let Some(ref thread) = self.current_thread {
            let messages = thread.get_messages();
            self.current_message_index < messages.len().saturating_sub(1)
        } else {
            false
        };

        if can_advance {
            self.current_message_index += 1;
            self.load_current_message().await?;
            
            // Get message count for status
            let message_count = self.current_thread.as_ref()
                .map(|t| t.get_messages().len())
                .unwrap_or(0);
            
            self.set_status(format!("Message {}/{}", 
                self.current_message_index + 1, 
                message_count));
        } else if self.current_thread.is_some() {
            self.set_status("At last message in thread".to_string());
        }
        Ok(())
    }

    /// Navigate to the previous message in the current thread
    pub async fn prev_message_in_thread(&mut self) -> Result<(), NotmuchError> {
        let can_go_back = self.current_message_index > 0 && self.current_thread.is_some();

        if can_go_back {
            self.current_message_index -= 1;
            self.load_current_message().await?;
            
            // Get message count for status
            let message_count = self.current_thread.as_ref()
                .map(|t| t.get_messages().len())
                .unwrap_or(0);
            
            self.set_status(format!("Message {}/{}", 
                self.current_message_index + 1, 
                message_count));
        } else if self.current_thread.is_some() {
            self.set_status("At first message in thread".to_string());
        }
        Ok(())
    }

    /// Get thread info for display
    pub fn get_thread_info(&self) -> Option<String> {
        if let Some(ref thread) = self.current_thread {
            let messages = thread.get_messages();
            if messages.len() > 1 {
                Some(format!("Thread: Message {}/{}", 
                    self.current_message_index + 1, 
                    messages.len()))
            } else {
                None
            }
        } else {
            None
        }
    }

    /// Go back to the previous view
    pub fn go_back(&mut self) {
        match self.state {
            AppState::EmailView => self.state = AppState::EmailList,
            AppState::Search => self.state = AppState::EmailList,
            AppState::Compose => self.state = AppState::EmailList,
            AppState::Help => self.state = AppState::EmailList,
            AppState::EmailList => {} // Already at top level
        }
        self.scroll_position = 0;
    }

    /// Enter search mode
    pub fn enter_search_mode(&mut self) {
        self.state = AppState::Search;
        self.search_query.clear();
        self.search_input = String::new();
    }

    /// Show help overlay
    pub fn show_help(&mut self) {
        self.state = AppState::Help;
    }

    /// Set status message
    pub fn set_status(&mut self, message: String) {
        self.status_message = Some(message);
    }

    /// Clear status message
    pub fn clear_status(&mut self) {
        self.status_message = None;
    }

    /// Quit the application
    pub fn quit(&mut self) {
        self.should_quit = true;
    }

    /// Get current email count for display
    pub fn email_count(&self) -> usize {
        self.search_results.len()
    }

    /// Get the currently selected search item
    pub fn selected_search_item(&self) -> Option<&SearchItem> {
        self.search_results.get(self.selected_email)
    }

    /// Handle character input in search mode
    pub fn handle_search_char(&mut self, c: char) {
        self.search_input.push(c);
    }
    
    /// Handle backspace in search mode
    pub fn handle_search_backspace(&mut self) {
        self.search_input.pop();
    }
    
    /// Execute the search based on current input
    pub async fn execute_search(&mut self) -> Result<(), NotmuchError> {
        if !self.search_input.is_empty() {
            self.search_query = self.search_input.clone();
            self.load_search_results().await?;
        } else {
            // Empty search returns to inbox
            self.load_inbox().await?;
        }
        Ok(())
    }
    
    /// Process email body content, converting HTML to text if needed
    pub async fn process_email_body(&self, message: &Message) -> Option<String> {
        if message.body.is_empty() {
            return Some("[No body content]".to_string());
        }

        // First try to find a plain text part
        if let Some(text_part) = message.body.iter().find(|part| part.content_type.starts_with("text/plain")) {
            if let crate::body::BodyContent::Text(text) = &text_part.content { return Some(text.clone()) }
        }

        // If no plain text, try to convert HTML to text
        if let Some(html_part) = message.body.iter().find(|part| part.content_type.starts_with("text/html")) {
            if let crate::body::BodyContent::Text(html) = &html_part.content {
                // Convert HTML to text using our text renderer
                match self.html_converter.convert(html).await {
                    Ok(converted_text) => return Some(converted_text),
                    Err(e) => {
                        // Fallback to showing raw HTML if conversion fails
                        return Some(format!("[HTML conversion failed: {}]\n\n{}", e, html));
                    }
                }
            }
        }

        // If no readable content found
        Some("[No readable content]".to_string())
    }

    /// Start composing a new email
    pub fn start_compose_new(&mut self) {
        if self.mail_sender.is_none() {
            self.set_status("Mail sending not configured".to_string());
            return;
        }
        
        self.compose_form = ComposeForm {
            mode: ComposeMode::New,
            ..Default::default()
        };
        self.state = AppState::Compose;
    }

    /// Start composing a reply to the current email
    pub fn start_compose_reply(&mut self, reply_all: bool) {
        if self.mail_sender.is_none() {
            self.set_status("Mail sending not configured".to_string());
            return;
        }

        if let Some(current_email) = &self.current_email {
            let thread_id = current_email.id.clone();
            let mode = if reply_all {
                ComposeMode::ReplyAll(thread_id)
            } else {
                ComposeMode::Reply(thread_id)
            };

            // Pre-populate reply fields
            let subject = if current_email.headers.subject.starts_with("Re: ") {
                current_email.headers.subject.clone()
            } else {
                format!("Re: {}", current_email.headers.subject)
            };

            self.compose_form = ComposeForm {
                mode,
                to: current_email.headers.from.clone(),
                subject,
                ..Default::default()
            };
            self.state = AppState::Compose;
        } else {
            self.set_status("No email selected for reply".to_string());
        }
    }

    /// Start composing a forward of the current email
    pub fn start_compose_forward(&mut self) {
        if self.mail_sender.is_none() {
            self.set_status("Mail sending not configured".to_string());
            return;
        }

        if let Some(current_email) = &self.current_email {
            let thread_id = current_email.id.clone();
            let subject = if current_email.headers.subject.starts_with("Fwd: ") {
                current_email.headers.subject.clone()
            } else {
                format!("Fwd: {}", current_email.headers.subject)
            };

            self.compose_form = ComposeForm {
                mode: ComposeMode::Forward(thread_id),
                subject,
                ..Default::default()
            };
            self.state = AppState::Compose;
        } else {
            self.set_status("No email selected for forward".to_string());
        }
    }

    /// Navigate between compose form fields
    pub fn compose_next_field(&mut self) {
        self.compose_form.current_field = match self.compose_form.current_field {
            ComposeField::To => ComposeField::Cc,
            ComposeField::Cc => ComposeField::Bcc,
            ComposeField::Bcc => ComposeField::Subject,
            ComposeField::Subject => ComposeField::Body,
            ComposeField::Body => ComposeField::To,
        };
    }

    /// Navigate to previous compose form field
    pub fn compose_prev_field(&mut self) {
        self.compose_form.current_field = match self.compose_form.current_field {
            ComposeField::To => ComposeField::Body,
            ComposeField::Cc => ComposeField::To,
            ComposeField::Bcc => ComposeField::Cc,
            ComposeField::Subject => ComposeField::Bcc,
            ComposeField::Body => ComposeField::Subject,
        };
    }

    /// Handle character input in compose mode
    pub fn compose_handle_char(&mut self, c: char) {
        match self.compose_form.current_field {
            ComposeField::To => self.compose_form.to.push(c),
            ComposeField::Cc => self.compose_form.cc.push(c),
            ComposeField::Bcc => self.compose_form.bcc.push(c),
            ComposeField::Subject => self.compose_form.subject.push(c),
            ComposeField::Body => self.compose_form.body.push(c),
        }
    }

    /// Handle Enter key in compose mode
    pub fn compose_handle_enter(&mut self) {
        match self.compose_form.current_field {
            ComposeField::Body => {
                // In the body field, Enter should insert a newline
                self.compose_form.body.push('\n');
            }
            _ => {
                // In other fields, Enter moves to the next field
                self.compose_next_field();
            }
        }
    }

    /// Handle backspace in compose mode
    pub fn compose_handle_backspace(&mut self) {
        match self.compose_form.current_field {
            ComposeField::To => { self.compose_form.to.pop(); }
            ComposeField::Cc => { self.compose_form.cc.pop(); }
            ComposeField::Bcc => { self.compose_form.bcc.pop(); }
            ComposeField::Subject => { self.compose_form.subject.pop(); }
            ComposeField::Body => { self.compose_form.body.pop(); }
        }
    }

    /// Send the composed email
    pub async fn send_composed_email(&mut self) -> Result<(), NotmuchError> {
        if let Some(ref mail_sender) = self.mail_sender {
            // Validate required fields
            if self.compose_form.to.trim().is_empty() {
                return Err(NotmuchError::ConfigError("To field is required".to_string()));
            }

            match &self.compose_form.mode {
                ComposeMode::New => {
                    let mut builder = ComposableMessage::builder()
                        .to(self.compose_form.to.clone())
                        .subject(self.compose_form.subject.clone())
                        .body(self.compose_form.body.clone());

                    // Add CC recipients if any
                    for cc_email in self.parse_email_list(&self.compose_form.cc) {
                        builder = builder.cc(cc_email);
                    }

                    // Add BCC recipients if any
                    for bcc_email in self.parse_email_list(&self.compose_form.bcc) {
                        builder = builder.bcc(bcc_email);
                    }

                    let message = builder.build()
                        .map_err(|e| NotmuchError::ConfigError(format!("Failed to build message: {}", e)))?;

                    let _message_id = mail_sender.send(message).await
                        .map_err(|e| NotmuchError::MailSendError(format!("Failed to send message: {}", e)))?;
                    
                    self.set_status("Email sent successfully".to_string());
                }
                ComposeMode::Reply(thread_id) | ComposeMode::ReplyAll(thread_id) => {
                    // Get the original message for reply
                    let thread = self.client.show(thread_id).await?;
                    if let Some(original_message) = thread.get_messages().into_iter().next() {
                        let is_reply_all = matches!(self.compose_form.mode, ComposeMode::ReplyAll(_));
                        
                        let reply = ComposableMessage::builder()
                            .to("dummy@example.com".to_string()) // Will be replaced by reply builder
                            .body(self.compose_form.body.clone())
                            .build()
                            .map_err(|e| NotmuchError::ConfigError(format!("Failed to build reply: {}", e)))?;

                        let _message_id = mail_sender.reply(original_message, reply, is_reply_all).await
                            .map_err(|e| NotmuchError::MailSendError(format!("Failed to send reply: {}", e)))?;
                        
                        self.set_status("Reply sent successfully".to_string());
                    } else {
                        return Err(NotmuchError::ConfigError("Original message not found".to_string()));
                    }
                }
                ComposeMode::Forward(thread_id) => {
                    // Get the original message for forward
                    let thread = self.client.show(thread_id).await?;
                    if let Some(original_message) = thread.get_messages().into_iter().next() {
                        let mut builder = ComposableMessage::builder()
                            .to(self.compose_form.to.clone())
                            .body(self.compose_form.body.clone());

                        // Add CC recipients if any
                        for cc_email in self.parse_email_list(&self.compose_form.cc) {
                            builder = builder.cc(cc_email);
                        }

                        // Add BCC recipients if any
                        for bcc_email in self.parse_email_list(&self.compose_form.bcc) {
                            builder = builder.bcc(bcc_email);
                        }

                        let forward = builder.build()
                            .map_err(|e| NotmuchError::ConfigError(format!("Failed to build forward: {}", e)))?;

                        let _message_id = mail_sender.forward(original_message, forward).await
                            .map_err(|e| NotmuchError::MailSendError(format!("Failed to send forward: {}", e)))?;
                        
                        self.set_status("Email forwarded successfully".to_string());
                    } else {
                        return Err(NotmuchError::ConfigError("Original message not found".to_string()));
                    }
                }
            }

            // Return to email list after sending
            self.state = AppState::EmailList;
            self.compose_form = ComposeForm::default();
        } else {
            return Err(NotmuchError::ConfigError("Mail sender not configured".to_string()));
        }

        Ok(())
    }

    /// Helper method to parse comma-separated email list
    fn parse_email_list(&self, input: &str) -> Vec<String> {
        input.split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect()
    }
}