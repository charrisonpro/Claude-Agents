# Project Coordinator (PC)

Async task processor for the Project Team. Runs on Haiku.

## Role

- Processes queued tasks from `Team Files/PM_Queue.md`
- Tracks status, logs decisions, checks conventions
- Outputs status reports to `Output/`
- Runs during downtime to save tokens on primary agents

## Usage

From `Project Team (PT)/`:
```bash
cargo run -- pc
```

## Files

```
Agent Files/
├── Instructions.md          # Core behavior (v1.1)
├── Domain_Knowledge.md      # Task processing rules
├── Conventions.md           # Output formats
├── Evaluation_Framework.md  # Test cases, feedback
├── Roadmap.md               # Planned features
├── Bug_Report.md            # Known issues
└── Version_History.md       # Changelog
```

## Task Types

| Type | Action |
|------|--------|
| `TRACK_QUESTION` | Add to Open Questions |
| `TRACK_ACTION` | Add to Action Items |
| `LOG_DECISION` | Add to Decisions Log |
| `CHECK_CONVENTIONS` | Validate file against standards |
| `DRAFT_VERSION_ENTRY` | Write changelog entry |
| `FLAG_HEAVY_OP` | Log with manual alternative |
| `STATUS_REQUEST` | Generate full status report |

## Output

Status reports: `Output/PC_Status_[Date].md`

## Version

v1.1 — 2025-01-15
