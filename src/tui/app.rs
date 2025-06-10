use crate::client::NotmuchClient;
use crate::error::NotmuchError;
use crate::search::SearchItem;
use crate::thread::Message;
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

pub struct App {
    /// Current application state
    pub state: AppState,
    
    /// Should the application quit?
    pub should_quit: bool,
    
    /// Current search results/email list
    pub search_results: Vec<SearchItem>,
    
    /// Currently selected email index
    pub selected_email: usize,
    
    /// Current email being viewed (if in EmailView state)
    pub current_email: Option<Message>,
    
    /// Current search query
    pub search_query: String,
    
    /// Scroll position in various views
    pub scroll_position: usize,
    
    /// Status message to display
    pub status_message: Option<String>,
    
    /// Notmuch client for data access
    client: Arc<dyn NotmuchClient>,
}

impl App {
    pub fn new(client: Arc<dyn NotmuchClient>) -> Self {
        Self {
            state: AppState::EmailList,
            should_quit: false,
            search_results: Vec::new(),
            selected_email: 0,
            current_email: None,
            search_query: String::new(),
            scroll_position: 0,
            status_message: None,
            client,
        }
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

    /// Open the currently selected email
    pub async fn open_selected_email(&mut self) -> Result<(), NotmuchError> {
        if let Some(search_item) = self.search_results.get(self.selected_email) {
            // Load the full thread to get the complete message
            let thread = self.client.show(&search_item.thread).await?;
            
            // For now, just take the first message in the thread
            self.current_email = thread.get_messages().into_iter().next().cloned();
                
            self.state = AppState::EmailView;
            self.scroll_position = 0;
        }
        Ok(())
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
}