# Notmuch Client Binary

A command-line tool for testing the notmuch client functionality with nicely formatted output.

## Usage

### Local Mode (default)

```bash
# Search for messages
notmuch-client search "tag:unread"

# Show a thread
notmuch-client show "thread:00000000000276db"

# Tag operations
notmuch-client tag "from:newsletter@example.com" --add inbox --remove unread

# Refresh database
notmuch-client refresh

# Insert a test message
notmuch-client insert --subject "Test" --from "sender@example.com" --tags unread --tags inbox
```

### Remote Mode

```bash
# Basic remote search
notmuch-client --remote --host mail.example.com search "tag:inbox"

# Remote with full SSH options
notmuch-client --remote --host mail.example.com --user alice --port 2222 --identity-file ~/.ssh/id_rsa search "*"

# Remote configuration
notmuch-client --remote --host mail.example.com config-get user.name
```

### Command Options

#### Global Options
- `--remote` - Use remote mode (SSH)
- `--host <HOST>` - Remote host (required for remote mode)
- `--user <USER>` - SSH user
- `--port <PORT>` - SSH port
- `--identity-file <PATH>` - SSH identity file
- `--notmuch-path <PATH>` - Path to notmuch binary
- `--database-path <PATH>` - Local database path

#### Commands

**search** - Search for messages
- `query` - Search query
- `--full-thread-id` - Show full thread IDs

**show** - Show messages in a thread
- `query` - Query (usually thread:ID)
- `--raw` - Show raw headers (note: not available in current data model)

**tag** - Tag messages
- `query` - Query to select messages
- `--add <TAG>` - Tags to add
- `--remove <TAG>` - Tags to remove

**refresh** - Refresh the database (scan for new messages)

**config-get** - Get configuration value
- `key` - Configuration key

**config-set** - Set configuration value
- `key` - Configuration key
- `value` - Configuration value

**insert** - Insert a test message
- `--subject <SUBJECT>` - Subject
- `--from <FROM>` - From address
- `--to <TO>` - To address
- `--body <BODY>` - Message body
- `--folder <FOLDER>` - Folder
- `--tags <TAG>` - Tags to apply

## Output Formatting

The tool provides colored and formatted output:
- Thread IDs in blue
- Tags with semantic colors (unread=red, inbox=yellow, flagged=bright yellow)
- Subjects in bold
- Metadata in dimmed text
- Success indicators in green
- Warnings in yellow
- Attachments with 📎 emoji