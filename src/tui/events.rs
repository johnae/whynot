use crossterm::event::{self, KeyCode, KeyEvent, KeyModifiers};
use std::time::Duration;

#[derive(Debug, Clone)]
pub enum Event {
    /// Key press event
    Key(KeyEvent),
    /// Terminal resize event
    Resize(u16, u16),
    /// Tick event for periodic updates
    Tick,
}

pub struct EventHandler {
    /// Tick rate for periodic updates
    tick_rate: Duration,
}

impl EventHandler {
    pub fn new(tick_rate: Duration) -> Self {
        Self { tick_rate }
    }

    /// Poll for the next event
    pub fn next(&self) -> Result<Event, std::io::Error> {
        if event::poll(self.tick_rate)? {
            match event::read()? {
                event::Event::Key(key_event) => Ok(Event::Key(key_event)),
                event::Event::Resize(width, height) => Ok(Event::Resize(width, height)),
                _ => Ok(Event::Tick),
            }
        } else {
            Ok(Event::Tick)
        }
    }
}

/// Helper functions for key handling
impl Event {
    /// Check if this is a quit key (q or Ctrl+C)
    pub fn is_quit(&self) -> bool {
        matches!(
            self,
            Event::Key(KeyEvent {
                code: KeyCode::Char('q'),
                modifiers: KeyModifiers::NONE,
                ..
            }) | Event::Key(KeyEvent {
                code: KeyCode::Char('c'),
                modifiers: KeyModifiers::CONTROL,
                ..
            })
        )
    }

    /// Check if this is a navigation up key (k or Up arrow)
    pub fn is_up(&self) -> bool {
        matches!(
            self,
            Event::Key(KeyEvent {
                code: KeyCode::Char('k') | KeyCode::Up,
                modifiers: KeyModifiers::NONE,
                ..
            })
        )
    }

    /// Check if this is a navigation down key (j or Down arrow)
    pub fn is_down(&self) -> bool {
        matches!(
            self,
            Event::Key(KeyEvent {
                code: KeyCode::Char('j') | KeyCode::Down,
                modifiers: KeyModifiers::NONE,
                ..
            })
        )
    }

    /// Check if this is an enter/select key
    pub fn is_enter(&self) -> bool {
        matches!(
            self,
            Event::Key(KeyEvent {
                code: KeyCode::Enter,
                modifiers: KeyModifiers::NONE,
                ..
            })
        )
    }

    /// Check if this is a back/escape key
    pub fn is_back(&self) -> bool {
        matches!(
            self,
            Event::Key(KeyEvent {
                code: KeyCode::Esc | KeyCode::Backspace,
                modifiers: KeyModifiers::NONE,
                ..
            })
        )
    }

    /// Check if this is a search key (/)
    pub fn is_search(&self) -> bool {
        matches!(
            self,
            Event::Key(KeyEvent {
                code: KeyCode::Char('/'),
                modifiers: KeyModifiers::NONE,
                ..
            })
        )
    }

    /// Check if this is a help key (?)
    pub fn is_help(&self) -> bool {
        matches!(
            self,
            Event::Key(KeyEvent {
                code: KeyCode::Char('?'),
                modifiers: KeyModifiers::NONE,
                ..
            })
        )
    }

    /// Check if this is a compose key (c)
    pub fn is_compose(&self) -> bool {
        matches!(
            self,
            Event::Key(KeyEvent {
                code: KeyCode::Char('c'),
                modifiers: KeyModifiers::NONE,
                ..
            })
        )
    }

    /// Check if this is a reply key (r)
    pub fn is_reply(&self) -> bool {
        matches!(
            self,
            Event::Key(KeyEvent {
                code: KeyCode::Char('r'),
                modifiers: KeyModifiers::NONE,
                ..
            })
        )
    }

    /// Check if this is a reply-all key (R or Shift+r)
    pub fn is_reply_all(&self) -> bool {
        matches!(
            self,
            Event::Key(KeyEvent {
                code: KeyCode::Char('R'),
                modifiers: KeyModifiers::NONE,
                ..
            }) | Event::Key(KeyEvent {
                code: KeyCode::Char('r'),
                modifiers: KeyModifiers::SHIFT,
                ..
            })
        )
    }

    /// Check if this is a forward key (f)
    pub fn is_forward(&self) -> bool {
        matches!(
            self,
            Event::Key(KeyEvent {
                code: KeyCode::Char('f'),
                modifiers: KeyModifiers::NONE,
                ..
            })
        )
    }

    /// Check if this is a tab key (for field navigation)
    pub fn is_tab(&self) -> bool {
        matches!(
            self,
            Event::Key(KeyEvent {
                code: KeyCode::Tab,
                modifiers: KeyModifiers::NONE,
                ..
            })
        )
    }

    /// Check if this is a shift+tab key (for reverse field navigation)
    pub fn is_shift_tab(&self) -> bool {
        matches!(
            self,
            Event::Key(KeyEvent {
                code: KeyCode::BackTab,
                modifiers: KeyModifiers::SHIFT,
                ..
            })
        )
    }

    /// Check if this is a send key (Ctrl+S)
    pub fn is_send(&self) -> bool {
        matches!(
            self,
            Event::Key(KeyEvent {
                code: KeyCode::Char('s'),
                modifiers: KeyModifiers::CONTROL,
                ..
            })
        )
    }
}