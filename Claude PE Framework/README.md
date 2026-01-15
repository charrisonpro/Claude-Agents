# Claude PE Framework Agent

A TUI-based Prompt Engineer agent built in Rust. Helps designers build, review, and optimize multi-step prompt systems for Claude agents.

## Features

- **TUI Interface** — Full terminal UI with scrollable message history, status bar, and input
- **File Tools** — Read, write, and list files in the Agent Files directory
- **Project Scaffolding** — Generate complete new agent projects with one command
- **Async API** — Non-blocking Claude API calls with real-time status updates
- **Conversation Persistence** — Auto-saves conversations to History/

## Prerequisites

- Rust toolchain ([rustup.rs](https://rustup.rs/))
- Anthropic API key

## Setup

```bash
# Set your API key
export ANTHROPIC_API_KEY="your-api-key-here"

# Build
cd "Claude PE Framework"
cargo build --release

# Run
cargo run --release
```

## Directory Structure

```
Claude PE Framework/
├── Cargo.toml
├── src/
│   ├── main.rs           # Entry point, CLI args
│   ├── agent.rs          # TUI rendering, API communication
│   ├── toolkit.rs        # File operation tools
│   └── scaffold.rs       # Project scaffolding tools
├── Agent Files/
│   ├── Instructions.md   # PE agent system prompt
│   ├── Domain_Knowledge.md   # Best practices & patterns
│   ├── Conventions.md    # Formatting standards
│   ├── Version_History.md
│   └── Arch/             # Archived versions
├── History/
│   └── conversation_log.md
└── Output/
```

## TUI Controls

| Key | Action |
|-----|--------|
| `Enter` | Send message |
| `Ctrl+C` | Quit (saves conversation) |
| `Up/Down` | Scroll messages (3 lines) |
| `PageUp/PageDown` | Scroll messages (10 lines) |
| `Left/Right` | Move cursor in input |
| `Home/End` | Jump to start/end of input |

## Including Files

Reference files in your message with `@filename.md`:

```
@Domain_Knowledge.md What are the key instruction positioning principles?
```

Multiple files work too:
```
@Instructions.md @Conventions.md Review these for consistency
```

## Available Tools

The PE agent has access to:

### File Tools
- `read_file` — Read from Agent Files/
- `write_file` — Write to Agent Files/
- `list_files` — List available files
- `save_history` — Save to History/
- `write_output` — Write to Output/

### Scaffolding Tools
- `scaffold_project` — Create a new agent project
- `list_project_structure` — Show standard project layout

## Creating New Agents

Ask the PE agent to scaffold a new project:

> "Create a new agent called 'Code Review Agent' at /path/to/projects with description 'Reviews code for best practices and security issues'"

This generates a complete, buildable agent project:

```
Code Review Agent/
├── Cargo.toml
├── README.md
├── src/
│   ├── main.rs
│   ├── agent.rs
│   └── toolkit.rs
├── Agent Files/
│   ├── Instructions.md
│   ├── Domain_Knowledge.md
│   ├── Conventions.md
│   └── Version_History.md
├── History/
└── Output/
```

## CLI Options

```bash
cargo run --release -- [OPTIONS]

Options:
  --path <DIR>      Working directory (default: current)
  --model <MODEL>   Claude model (default: claude-sonnet-4-20250514)
  --name <NAME>     Agent display name (default: directory name)
  --help            Show help
```

## Example Session

```
$ cargo run --release

=================================================
Claude PE Framework
=================================================
Working Directory: /Users/you/Claude PE Framework
Agent Files:
  - Conventions.md
  - Domain_Knowledge.md
  - Instructions.md
  - Version_History.md
=================================================
Starting TUI...

┌─ Claude PE Framework ─────────────────────────┐
│ » Claude PE Framework - Ctrl+C to quit        │
│ » Model: claude-sonnet-4-20250514             │
│                                               │
│ You:                                          │
│   What agent are we building today?           │
│                                               │
│ Agent:                                        │
│   Framework activated. I'm your Prompt        │
│   Engineer peer—ready to build, review, or    │
│   debug prompt systems with you.              │
│   ...                                         │
└───────────────────────────────────────────────┘
 Ready
┌─ Input ───────────────────────────────────────┐
│ _                                             │
└───────────────────────────────────────────────┘
```

## Testing

```bash
cargo test
```

## Troubleshooting

**API Key Error**
```bash
echo $ANTHROPIC_API_KEY  # Should print your key
```

**Build Errors**
```bash
rustup update stable
```

**Agent Files Not Found**
Make sure you're running from the project directory that contains `Agent Files/`.
