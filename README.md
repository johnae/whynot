# Whynot Mail

A Rust-based email interface for [notmuch](https://notmuchmail.org/) that doesn't suck. Because why not have nice things?

## What's This About?

If you use notmuch to index your email but miss having a decent interface to actually *read* it, this project might be for you. Currently features a simple, modern clean web UI that can connect to either a local notmuch database or a remote one over SSH. I.e webmail.

**Fair warning**: This is very early-stage software with plenty of rough edges and probably some bugs lurking around. Use at your own risk, but feel free to file issues if you find anything particularly broken.

## Current Features

- 🕸️ **Web Interface**: Clean, modern web UI with light/dark mode toggle
- 🔍 **Search & Browse**: Full notmuch search capabilities with tag filtering
- 📧 **Thread View**: Read email threads with proper formatting and attachments
- 🌐 **Local & Remote**: Works with local notmuch or remote over SSH
- 🎨 **Modern Stack**: Built with Axum, Askama templates, and a touch of HTMX

## The Plan

I started with a webui, but I'd like to do some more stuff:

1. ✅ **Data Layer**: Comprehensive Rust types for all notmuch JSON output
2. ✅ **Client Layer**: Unified interface for local/remote notmuch execution  
3. ✅ **Web UI**: What you see now - GitHub-inspired webmail interface
4. 🚧 **TUI**: Terminal interface using [ratatui](https://ratatui.rs/) (I really want this so I'll be working on this soon)
5. 🔮 **Mobile/Desktop**: Maybe native apps someday?

## Getting Started

### Prerequisites

- Rust (2024 edition)
- A working notmuch setup (local or remote)
- [devenv.sh](https://devenv.sh/) for the development environment (optional but recommended)

### Quick Start

```bash
# Clone the repo
git clone https://github.com/johnae/whynot
cd whynot

# For local notmuch
cargo run --bin whynot-web

# Custom bind address
cargo run --bin whynot-web -- --bind 0.0.0.0:3000

# For remote notmuch over SSH
cargo run --bin whynot-web -- --remote your-server --user the-server-user ## this is for remote/ssh access to notmuch
```

Then open http://localhost:8080 (the default port) in your browser.

### SSH Setup

If you're using remote notmuch (which is pretty cool), you'll want SSH key-based authentication set up. I use [Tailscale](https://tailscale.com) with ACLs to securely expose notmuch over my private network, which is a nice way to access my email from anywhere without exposing SSH to the entire internet.

```bash
# Environment variables work too
export NOTMUCH_HOST="mail.example.com"
export NOTMUCH_USER="yourusername"
export NOTMUCH_PORT="2222"
cargo run --bin whynot-web
```

## What's Missing

Quite a bit, actually:

- **No compose/send**: This is read-only for now (send mail support is something I really want to add)
- **No message management**: Can't delete, archive, or organize messages yet
- **Limited attachment handling**: Basic viewing but no fancy preview, you can download the attachments though
- **No offline support**: Fully dependent on notmuch being available. This may never change, we'll see.

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
```

## Architecture

- **Types**: Comprehensive Rust structs for all notmuch JSON formats
- **Client**: Unified trait-based interface supporting local and SSH execution
- **Web**: Axum-based server with Askama templates and GitHub-inspired styling
- **Testing**: Integration tests with temporary notmuch databases

## Contributing

It's early days, so don't expect much stability in the APIs. That said, if you find bugs or have ideas for improvements, issues and PRs are welcome.

## License

[MIT](https://choosealicense.com/licenses/mit)

---

*Built with Rust, powered by notmuch, inspired by the desire to have some nice things.*
