# Whynot Mail

A complete Rust-based email client for [notmuch](https://notmuchmail.org/) with both web and terminal interfaces. Because why not have nice things?

## What's This About?

If you use notmuch to index your email but miss having decent interfaces to actually *use* it, this project is for you. Features both a modern web UI and a fully-functional terminal interface, with complete email reading, searching, and sending capabilities. Works with local notmuch databases or remote ones over SSH.

**Status**: Production-ready for daily email use with comprehensive functionality and thorough testing.

## Features

### 🕸️ **Web Interface**
- **Modern UI**: Clean, GitHub-inspired interface with light/dark mode toggle
- **Complete Email Workflow**: Read, compose, reply, reply-all, forward emails
- **Smart Threading**: Navigate email conversations with proper message threading
- **Auto-refresh**: Automatic inbox updates with configurable intervals
- **Infinite Scroll**: Efficient pagination for large mailboxes
- **Rich Content**: HTML emails with image toggle and link safety warnings

### 🖥️ **Terminal Interface (TUI)**
- **Full-featured**: Complete email client with vim-like navigation (j/k, /, ?)
- **Content Scrolling**: Navigate long emails with j/k, Page Up/Down, Home/G
- **Thread Navigation**: Access all messages in threads with n/p keys
- **HTML Support**: Rich HTML emails converted to readable terminal text
- **Complete Composition**: Write, reply, and send emails with multi-line support
- **Search Integration**: Full notmuch query support with modal interface

### 📧 **Email Management**
- **Reading**: Full notmuch search capabilities with tag filtering and thread view
- **Writing**: Complete composition with reply/forward, proper threading headers
- **Sending**: msmtp integration (local and remote) with connection testing
- **Attachments**: View and download email attachments safely
- **Threading**: Proper email conversation handling with References/In-Reply-To

### 🌐 **Connectivity**
- **Local & Remote**: Works with local notmuch or remote over SSH
- **Unified Config**: Single configuration system for all components
- **SSH Reliability**: Robust connection handling with automatic retries

## Implementation Status

1. ✅ **Data Layer**: Comprehensive Rust types for all notmuch JSON output
2. ✅ **Client Layer**: Unified interface for local/remote notmuch execution  
3. ✅ **Web UI**: Complete GitHub-inspired webmail interface with full functionality
4. ✅ **TUI**: Full-featured terminal interface using [ratatui](https://ratatui.rs/)
5. ✅ **Mail Sending**: Complete msmtp integration with local/remote support
6. ✅ **Configuration**: Unified config system with CLI/env/file precedence
7. 🔮 **Future**: Mobile/desktop apps, advanced features, integrations

## Getting Started

### Prerequisites

- Rust (2024 edition)
- A working notmuch setup (local or remote)
- **For sending email**: [msmtp](https://marlam.de/msmtp/) configured (local or remote)
- [devenv.sh](https://devenv.sh/) for the development environment (optional but recommended)

### Quick Start

```bash
# Clone the repo
git clone https://github.com/johnae/whynot
cd whynot

# Web Interface - local notmuch
cargo run --bin whynot-web

# Web Interface - custom bind address  
cargo run --bin whynot-web -- --bind 0.0.0.0:3000

# Web Interface - remote notmuch over SSH
cargo run --bin whynot-web -- --notmuch-host your-server --notmuch-user the-server-user

# Terminal Interface - local notmuch
cargo run --bin whynot-tui

# Terminal Interface - remote notmuch over SSH  
cargo run --bin whynot-tui -- --notmuch-host your-server --notmuch-user the-server-user
```

**Web Interface**: Open http://localhost:8080 (default port) in your browser  
**Terminal Interface**: Full vim-like email client in your terminal

### SSH Setup

If you're using remote notmuch (which is pretty cool), you'll want SSH key-based authentication set up. I use [Tailscale](https://tailscale.com) with ACLs to securely expose notmuch over my private network, which is a nice way to access my email from anywhere without exposing SSH to the entire internet.

```bash
# Environment variables work too (legacy format)
export NOTMUCH_HOST="mail.example.com"
export NOTMUCH_USER="yourusername"
export NOTMUCH_PORT="2222"
cargo run --bin whynot-web

# Or use the new WHYNOT_ prefix
export WHYNOT_NOTMUCH_HOST="mail.example.com"
export WHYNOT_NOTMUCH_USER="yourusername"
cargo run --bin whynot-web
```

### Configuration

Whynot supports multiple configuration methods with the following precedence:

1. **CLI arguments** (highest priority): `--notmuch-host mail.example.com`
2. **Environment variables**: `WHYNOT_NOTMUCH_HOST=mail.example.com` 
3. **Configuration file**: `~/.config/whynot/config.toml`
4. **Built-in defaults** (lowest priority)

See `config.example.toml` for a comprehensive example with all available options including:
- Mail reading (local/remote notmuch setup)  
- Mail sending (local/remote msmtp configuration)
- User identity (name, email, signature)
- UI customization (themes, auto-refresh, pagination)
- Advanced settings (threading, external tools)

## Key Bindings (TUI)

### Navigation
- `j/k` or `↑/↓` - Move up/down in email list, scroll content in email view
- `Enter` - Open selected email
- `Esc` - Go back to previous view
- `Page Up/Down` - Fast scroll in email view
- `Home/G` - Jump to top/bottom of email content

### Actions  
- `/` - Search (supports full notmuch query syntax)
- `c` - Compose new email (from email list)
- `r` - Reply to current email (from email view)  
- `R` - Reply-all to current email (from email view)
- `f` - Forward current email (from email view)
- `n/p` - Navigate next/previous message in thread (from email view)
- `?` - Show help
- `q` - Quit

### Compose Mode
- `Tab/Shift+Tab` - Navigate between form fields
- `Enter` - New line in body field, move to next field in headers
- `Ctrl+S` - Send email
- `Esc` - Cancel compose

## Future Enhancements

- **Message Management**: Delete, archive, tag operations
- **Advanced Features**: External editor integration, GPG support, address book
- **Performance**: Offline caching, virtual scrolling for very large mailboxes
- **Integrations**: Calendar invites, link previews, syntax highlighting

## Development

This project uses a test-driven approach with `devenv.sh` for reproducible environments. For detailed development guidelines, workflow practices, and project structure, please see [DEVELOPMENT.md](./DEVELOPMENT.md).

```bash
# Load development environment (with direnv)
direnv allow

# Run tests
devenv shell cargo test

# Check code quality  
devenv shell cargo clippy --all-targets

# Start the web server
devenv shell cargo run --bin whynot-web

# Start the terminal interface
devenv shell cargo run --bin whynot-tui
```

## Architecture

- **Types**: Comprehensive Rust structs for all notmuch JSON formats with serde support
- **Client**: Unified trait-based interface supporting local and SSH execution with connection pooling
- **Mail Sending**: Complete msmtp integration with local/remote support and connection testing
- **Web**: Axum-based server with Askama templates, GitHub-inspired styling, and HTMX interactivity
- **TUI**: Full ratatui-based terminal interface with vim-like navigation and complete functionality
- **Configuration**: Unified system with CLI/environment/file precedence and comprehensive validation
- **Testing**: Extensive test suite with temporary notmuch databases and integration testing

## Contributing

The core functionality is complete and stable, but there's always room for improvement! If you find bugs, have ideas for enhancements, or want to contribute new features, issues and PRs are welcome. Please see [DEVELOPMENT.md](./DEVELOPMENT.md) for development guidelines.

## License

[MIT](https://choosealicense.com/licenses/mit)

---

*Built with Rust 🦀, powered by notmuch 📧, designed for people who want nice things ✨*

**Ready for daily use - try both the web and terminal interfaces!**
