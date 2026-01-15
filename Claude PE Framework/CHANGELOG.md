# Code Changelog

Tracks changes to the Rust implementation. For prompt/framework changes, see `Agent Files/Version_History.md`.

---

## v1.1.0 (2025-01-14)

### Major Changes
- **Modular architecture**: Refactored from single file to `src/` structure
  - `main.rs` — Entry point, CLI argument parsing
  - `agent.rs` — TUI rendering, Claude API communication
  - `toolkit.rs` — File operation tools
  - `scaffold.rs` — Project scaffolding tools

- **TUI interface**: Full terminal UI using ratatui
  - Scrollable message history
  - Status bar with thinking/tool indicators
  - Input box with cursor navigation
  - Color-coded message types (user, agent, system, tool, error)

- **Project scaffolding**: Create new agent projects
  - `scaffold_project` — Full project with Rust source and Agent Files
  - `list_agents` — Discover sibling agent projects
  - `list_project_structure` — Show standard layout
  - Default placement as sibling directories

### New Features
- Conversation auto-save to `History/`
- `write_output` tool for deliverables
- `save_history` tool for persistence
- CLI flags: `--path`, `--model`, `--name`

### Directory Structure
```
src/
├── main.rs
├── agent.rs
├── toolkit.rs
└── scaffold.rs
```

---

## v1.0.0 (2025-01-14)

### Initial Release
- Single-file implementation (`claude_agent_init.rs`)
- Basic TUI with ratatui
- File tools: read, write, list
- Interactive mode with `@filename.md` syntax
- Async Claude API calls with tool execution loop
