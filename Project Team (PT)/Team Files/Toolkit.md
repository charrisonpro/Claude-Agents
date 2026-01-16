# Project Team Toolkit

Shared tool documentation for all Project Team agents. Run from `Project Team (PT)/` with `cargo run -- [agent]`.

---

## Running Agents

```bash
cd "Project Team (PT)"
cargo run -- pm    # Project Manager
cargo run -- pe    # Prompt Engineer
cargo run -- pc    # Project Coordinator
```

The Rust runtime loads the selected agent's `Agent Files/Instructions.md` and provides these tools.

---

## File Tools

Standard tools for managing files within an agent's directories.

### read_file
Read a file from the agent's `Agent Files/` directory.

**Parameters:**
- `filename` (required): Relative path to the file

**Example:**
```json
{"filename": "Domain_Knowledge.md"}
```

---

### write_file
Write content to a file in `Agent Files/`. Creates or overwrites.

**Parameters:**
- `filename` (required): Relative path for the file
- `content` (required): Content to write

---

### list_files
List available files in `Agent Files/`.

**Parameters:**
- `include_archive` (optional): Set `true` to include `Arch/` folder contents

---

### save_history
Save content to the `History/` directory for session persistence.

**Parameters:**
- `filename` (required): Filename for the history entry
- `content` (required): Content to save

---

### write_output
Write deliverables to the `Output/` directory.

**Parameters:**
- `filename` (required): Filename for the output
- `content` (required): Content to write

---

## Queue Tools

Tools for async task coordination via the shared queue at `Team Files/PM_Queue.md`.

### read_queue
Read the shared task queue.

**Parameters:** None

**Returns:** Current queue contents

---

### write_queue
Write updated queue content.

**Parameters:**
- `content` (required): Full queue content to write

---

## Queue Protocol

### Format
```markdown
- [ ] TASK_TYPE: Description (owner: agent_name)
```

### Task Types
| Type | Purpose |
|------|---------|
| `TRACK_QUESTION` | Log open question |
| `TRACK_ACTION` | Log action item with owner |
| `LOG_DECISION` | Record decision made |
| `CHECK_CONVENTIONS` | Validate file against standards |
| `DRAFT_VERSION_ENTRY` | Write changelog entry |
| `FLAG_HEAVY_OP` | Flag expensive operation with manual alternative |
| `STATUS_REQUEST` | Generate full status report |

### Heavy Op Check
Before proposing large rewrites or folder restructures:
> Could a human do this in fewer keystrokes than tokens I'd spend?

If yes: `- [ ] FLAG_HEAVY_OP: [operation] → Alt: [manual action]`

---

## Scaffolding Tools (PE Only)

### scaffold_project
Create a complete new agent project structure.

**Parameters:**
- `project_name` (required): Name of the agent
- `description` (required): Brief description
- `model` (optional): Target model (default: claude-sonnet-4-20250514)

---

### list_agents
Discover existing agent projects.

**Parameters:** None

---

## Tool Access by Agent

| Agent | File Tools | Queue Read | Queue Write | Scaffolding |
|-------|------------|------------|-------------|-------------|
| PM | ✓ | ✓ | ✓ | — |
| PE | ✓ | ✓ | ✓ | ✓ |
| PC | ✓ | ✓ | ✓ | — |

**PM triages all work** — reads queue to understand status, writes to assign/reassign tasks.

---

## Directory Structure

```
Project Team (PT)/
├── Cargo.toml              # Shared Rust project
├── config.toml             # API key, defaults
├── src/
│   ├── main.rs             # CLI: cargo run -- [agent]
│   ├── agent.rs            # TUI and API logic
│   └── toolkit.rs          # Tool implementations
├── Team Files/
│   ├── PM_Queue.md         # Shared task queue
│   ├── Toolkit.md          # This file
│   └── ...
├── Project Manager (PM)/
│   └── Agent Files/        # PM prompt files
├── Prompt Engineer (PE)/
│   └── Agent Files/        # PE prompt files
└── Project Coordinator (PC)/
    └── Agent Files/        # PC prompt files
```
