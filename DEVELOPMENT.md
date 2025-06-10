# Development Guide

This document provides comprehensive guidance for developers working on the Whynot Mail project.

## Project Overview

This is a Rust project that uses devenv.sh for development environment management. The first two iterations have been completed, implementing Rust types for notmuch JSON output and a flexible client for local/remote notmuch execution.

This project aims to create different interfaces for interacting with the notmuch email indexer, including responding to email. The two interfaces currently planned are a web mail UI and a TUI.

For current work and todos, please see ./TODO.md

## Development Workflow

### Version Control with Jujutsu (jj)

This project uses [Jujutsu (jj)](https://github.com/martinvonz/jj) instead of git for version control. Jj is fully interoperable with git but provides a more intuitive workflow for development.

**Starting a new feature:**
```bash
# Create a new change for your feature - please always do this as you start new work. You can combine it with -m to avoid the describe step.
jj new

# Describe what you're implementing before starting work
jj describe -m 'feat(search): add advanced filtering options'
# or for bug fixes:
jj describe -m 'fix(web): resolve CSS layout issue in thread view'
```

**Commit conventions:**
- Use conventional commit format: `feat(scope): description` or `fix(scope): description`
- Commit frequently, especially after tests pass
- Each logical change should be a separate commit

**Completing a feature:**
```bash
# When done with current feature, start a new change
jj new

# With jj, your changes are automatically tracked (i.e "committed") whenever you run any jj command basically.
# You tend to start a change and then evolve it through whatever changes you make. When you're happy, you can
# start a new change via "jj new". If you want to, you may edit an existing change through "jj edit <rev>"
```

**Key differences from git:**
- No explicit commit step - changes are automatically tracked
- `jj new` both commits current work and starts a new change
- Each change has a unique ID and can be easily amended
- Working copy is always on a change, never detached

### Test-Driven Development

This project follows a test-driven workflow. For changes where it makes sense, start by creating a test. Do not create any mocks but rather, create a test and run it expecting it to fail. After the tests are in place, go ahead and implement the actual feature, fix or change. Do not change the test(s) to make it pass, unless the test is obviously wrong.

### Code Organization

When a piece of functionality is repeated again and again, it could be some id extracting logic for instance, please extract that logic to a function and use that instead. This also enables us to write a test for verifying that the function does what we want it to do while encapsulating the logic itself. For a one-liner though, it may be overkill to extract into a function. It's a balancing act whether to extract the logic or not. If only used in a single place for example, it may be reasonable to not extract the logic into a function if it's only a few lines.

When a new tool or library dependency is needed (not a rust one but something else), please add that dependency to the devenv.nix file.

## Configuration Management

Whynot Mail uses a unified configuration system with the following precedence:
1. CLI arguments (highest priority)
2. Environment variables (`WHYNOT_*` prefix)
3. Configuration file (`~/.config/whynot/config.toml`)
4. Built-in defaults (lowest priority)

**Configuration Development Requirements:**

- **Always update `config.example.toml`** when adding new configuration options
- Add corresponding CLI arguments with `--option-name` format
- Add environment variable support with `WHYNOT_OPTION_NAME` format
- Update the `CliArgs` struct in `src/config.rs` with proper help text
- Update configuration structs and merging logic as needed
- Test all three configuration methods (CLI, env vars, config file) work correctly

The example configuration file serves as both documentation and a template for users. It must be kept in sync with all available options.

## Code Quality

Please follow clippy hints and suggestions. Run `devenv shell cargo clippy` regularly and address the warnings and suggestions it provides. Clippy helps ensure idiomatic Rust code and catches common mistakes.

- When running clippy, always run with --all-targets (so we fix any tests as well for example).

## Notmuch Documentation

Please use the notmuch documentation freely for understanding how to use the cli tool. You can find the documentation here:

https://notmuchmail.org/manpages/

## Development Environment

The project uses:
- **devenv.sh** - Nix-based reproducible development environments
- **direnv** - Automatic environment loading
- **Helix editor** (`hx`) - Configured in workspace.yaml as a suggested editor
- **Rust** - Enabled via devenv.nix

## Commands

### Environment Setup
```bash
# Load the development environment (automatic with direnv)
direnv allow

# Manual environment activation if needed
devenv shell
```

### Running Commands
To ensure we are using the latest environment and tooling defined in the devenv.nix file, all commands should be run through `devenv shell`. Like this:

```bash
devenv shell cargo --version
devenv shell ls -lah
```

### Rust Development Commands
```bash
devenv shell cargo build      # Build the project
devenv shell cargo run        # Run the project
devenv shell cargo test       # Run tests
devenv shell cargo fmt        # Format code
devenv shell cargo clippy     # Run linter
```

Whenever a new tool dependency has been added in devenv.nix, you may run it via `devenv shell`.

## Project Structure

```
├── Cargo.toml              # Rust project manifest
├── askama.toml             # Askama template configuration
├── src/
│   ├── lib.rs             # Library root
│   ├── search.rs          # Search result types
│   ├── thread.rs          # Thread and message types
│   ├── body.rs            # Email body and attachment types
│   ├── common.rs          # Shared types (Headers, etc.)
│   ├── error.rs           # Error handling
│   ├── client/            # Notmuch client implementations
│   │   ├── mod.rs         # Client trait and factory
│   │   ├── config.rs      # Client configuration
│   │   ├── local.rs       # Local command execution
│   │   └── remote.rs      # SSH-based remote execution
│   ├── web/               # Web UI implementation
│   │   ├── mod.rs         # Web module with routes and handlers
│   │   ├── templates/     # Askama HTML templates
│   │   │   ├── base.html  # Base layout template
│   │   │   ├── inbox.html # Inbox/search view
│   │   │   ├── thread_simple.html # Thread view
│   │   │   ├── settings.html # Settings page
│   │   │   └── error.html # Error page
│   │   └── static/        # Static assets
│   │       └── css/       # CSS files
│   │           └── main.css # GitHub-inspired styling
│   ├── test_utils/        # Testing utilities
│   │   ├── mod.rs         
│   │   ├── mbox.rs        # Mbox file generation
│   │   └── notmuch.rs     # Temporary notmuch setup
│   └── bin/               # Binary executables
│       ├── notmuch-client.rs  # CLI client example
│       └── whynot-web.rs      # Web server binary
├── examples/              # Example code and data
│   ├── client_demo.rs     # Client usage examples
│   └── notmuch/           # Example JSON outputs from notmuch
├── tests/                 # Integration tests
│   ├── web_integration_tests.rs # Web UI tests
│   └── ...                # Other integration tests
├── devenv.nix             # Development environment configuration
├── devenv.yaml            # Nix input sources
├── workspace.yaml         # Terminal workspace layout
└── PLAN-iteration-*.md    # Iteration planning documents
```

## Running the Web UI

```bash
# Local notmuch
devenv shell cargo run --bin whynot-web

# Remote notmuch over SSH
devenv shell cargo run --bin whynot-web -- --remote mail.example.com --user username

# Custom port
devenv shell cargo run --bin whynot-web -- --bind 0.0.0.0:3000

# With logging
RUST_LOG=whynot=debug devenv shell cargo run --bin whynot-web -- --remote mail.example.com --user username
```

## Notes

- The workspace is configured to use Helix editor with various development tools in a split pane layout
- Use `devenv shell` prefix for all commands to ensure proper environment
- The web UI is accessible at http://localhost:8080 by default
- Theme preference is stored in a cookie and persists across sessions
