# Claude Code Workflow

Reference document for Claude Code when working with the Project Team (PT) agent system.

---

## Team Hierarchy

```
Designer (User)
    ↓
Claude Code (You) ←→ Project Manager (PM)
                          ↓
              ┌───────────┴───────────┐
              ↓                       ↓
    Prompt Engineer (PE)    Project Coordinator (PC)
```

| Agent | Model | Role |
|-------|-------|------|
| Claude Code | Opus | Master implementation, file management, git |
| PM | Sonnet | **Triages all work**, strategy, user sounding board |
| PE | Opus | Agent design, framework maintenance |
| PC | Haiku | Async queue processing, status tracking |

**PM is the hub** — triages tasks between all agents including the designer.

---

## Running Agents

From `Project Team (PT)/`:
```bash
cargo run -- pm    # Project Manager
cargo run -- pe    # Prompt Engineer
cargo run -- pc    # Project Coordinator
```

The Rust runtime loads the selected agent's `Agent Files/Instructions.md`.

---

## Directory Structure

```
Project Team (PT)/
├── Cargo.toml                  # Shared Rust project
├── config.toml                 # API key, model defaults
├── src/
│   ├── main.rs                 # CLI: cargo run -- [agent]
│   ├── agent.rs                # TUI and API logic
│   └── toolkit.rs              # Tool implementations
│
├── Team Files/                 # Shared resources
│   ├── Master_Plan.md
│   ├── PM_Queue.md             # Task queue (PM triages)
│   ├── Toolkit.md              # Tool documentation
│   ├── Claude_Code_Workflow.md # This file
│   └── Project_History.md
│
├── Project Manager (PM)/
│   ├── Agent Files/            # 7 standard files
│   ├── History/
│   └── Output/
│
├── Prompt Engineer (PE)/
│   ├── Agent Files/
│   │   ├── Templates/          # Stems, base tools
│   │   └── Arch/               # Version archive
│   ├── History/
│   └── Output/                 # Designs for Claude Code
│
└── Project Coordinator (PC)/
    ├── Agent Files/
    ├── History/
    └── Output/                 # Status reports
```

---

## Standard Agent File Template

Every agent maintains these files in `Agent Files/`:

| File | Purpose |
|------|---------|
| `Instructions.md` | Core identity, behavior, workflow |
| `Domain_Knowledge.md` | Facts, patterns, learnings |
| `Conventions.md` | Formatting rules, procedures |
| `Evaluation_Framework.md` | Test cases, feedback, metrics |
| `Roadmap.md` | Planned features |
| `Bug_Report.md` | Known issues |
| `Version_History.md` | Changelog |

---

## Workflow

### Task Flow
```
User/Claude Code → PM (triage) → assigns to:
                                  ├── PE (design work)
                                  ├── PC (token-saving tasks)
                                  └── Claude Code (complex implementation)
```

### PM Triage
PM reads queue, understands status, and:
- Assigns new tasks to appropriate agent
- Reassigns blocked tasks
- Escalates to designer when needed
- Logs decisions

### Assignment Rules
| Task Type | Assign To | Examples |
|-----------|-----------|----------|
| Agent design, prompts | PE | New agent, prompt revision |
| Token-saving tasks | PC | Align language, convention checks, status |
| Complex implementation | Claude Code | Multi-file changes, Rust code |
| Quick file edits | Claude Code | Single file fix |

### PE Design → Claude Code Implementation
1. PE designs agent prompts
2. PE deposits in `Output/` with implementation notes
3. Claude Code picks up and implements
4. Claude Code pushes to repo

### PC Token-Saving Tasks
PC runs on Haiku to save tokens. Route these to PC:
- Language alignment across files
- Convention/formatting checks
- Status report generation
- Changelog drafting
- Routine documentation updates

---

## Queue Protocol

### Location
`Team Files/PM_Queue.md`

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

### Queue Access
| Agent | Read | Write |
|-------|------|-------|
| PM | ✓ (triage) | ✓ (assign/reassign) |
| PE | ✓ | ✓ (add tasks) |
| PC | ✓ (process) | ✓ (mark complete) |

---

## Claude Code Responsibilities

### You Own
- Local file system operations
- Git commits and pushes
- Implementing PE designs
- Building/maintaining Rust tooling
- Directory structure management

### You Coordinate With
- **PM**: For strategic decisions, task prioritization
- **PE**: For prompt design, agent architecture
- **PC**: Via queue only (async)

### You Don't
- Make unilateral design decisions on agent behavior
- Modify agent Instructions.md without PE input
- Process queue items directly (that's PC's job)
- Triage tasks (that's PM's job)

---

## Version Control

### Semantic Versioning
```
# Agent Name (vMAJOR.MINOR)
```
- **Major**: Role or workflow change
- **Minor**: Capability additions

### Commit Convention
```
[Agent] vX.Y: Brief description

- Detail 1
- Detail 2
```

---

## Quick Reference

### Create New Agent
1. PM determines need and priority
2. PE designs prompt (deposits in Output/)
3. Claude Code scaffolds folder structure
4. Claude Code populates Agent Files
5. Add to Master_Plan.md

### Update Existing Agent
1. PM approves change
2. PE designs update if behavioral
3. Claude Code implements
4. Update Version_History.md
5. Commit with convention

### Add to Queue
```markdown
- [ ] TASK_TYPE: Description (owner: agent_name)
```
