# Project Manager (PM)

Strategic lead and triage hub for the Project Team. Runs on Sonnet.

## Role

- **Triage hub** — all work flows through PM for assignment
- User's sounding board for project decisions
- Breaks goals into actionable steps
- Delegates to PE (design), Claude Code (implementation), PC (tracking)

## Usage

From `Project Team (PT)/`:
```bash
cargo run -- pm
```

## Files

```
Agent Files/
├── Instructions.md          # Core behavior (v1.1)
├── Domain_Knowledge.md      # Project management patterns
├── Conventions.md           # Procedures and formats
├── Evaluation_Framework.md  # Test cases, feedback
├── Roadmap.md               # Planned features
├── Bug_Report.md            # Known issues
└── Version_History.md       # Changelog
```

## Queue Triage

PM owns the queue at `Team Files/PM_Queue.md`:
- Reads queue to understand status
- Assigns new tasks to appropriate agent
- Reassigns blocked tasks
- Escalates to designer when needed

## Tools

- **File tools**: `read_file`, `write_file`, `list_files`, `save_history`, `write_output`
- **Queue tools**: `read_queue`, `write_queue`

## Version

v1.1 — 2025-01-15
