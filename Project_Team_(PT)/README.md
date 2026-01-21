# Project Team (PT)

A multi-agent system for building Claude agents. Three specialized agents coordinate via a shared task queue.

## Agents

| Agent | Model | Role |
|-------|-------|------|
| PM | Sonnet | Triage hub, strategy, decisions |
| PE | Opus | Agent design, prompt engineering |
| PC | Haiku | Async queue processing, status |

## Quick Start

```bash
# Set API key
export ANTHROPIC_API_KEY="your-key"

# Run an agent
cd "Project Team (PT)"
cargo run -- pm    # Project Manager
cargo run -- pe    # Prompt Engineer
cargo run -- pc    # Project Coordinator
```

## Structure

```
Project Team (PT)/
├── Cargo.toml              # Shared Rust project
├── config.toml             # API key config
├── src/                    # Centralized Rust code
├── Team Files/             # Shared resources
│   ├── PM_Queue.md         # Task queue
│   ├── Toolkit.md          # Tool documentation
│   ├── Claude_Code_Workflow.md
│   └── Master_Plan.md
├── Project Manager (PM)/   # Sonnet
├── Prompt Engineer (PE)/   # Opus
└── Project Coordinator (PC)/ # Haiku
```

## Workflow

```
User → PM (triage) → assigns to:
                     ├── PE (design)
                     ├── PC (tracking)
                     └── Claude Code (implementation)
```

## Documentation

- [Claude_Code_Workflow.md](Team Files/Claude_Code_Workflow.md) — Full workflow reference
- [Toolkit.md](Team Files/Toolkit.md) — Tool documentation
- [Master_Plan.md](Team Files/Master_Plan.md) — Strategic overview
