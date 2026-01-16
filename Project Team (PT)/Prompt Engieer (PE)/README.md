# Prompt Engineer (PE)

Agent design specialist for the Project Team. Runs on Opus.

## Role

- Designs prompt instructions for new agents
- Analyzes prompts for clarity, consistency, executability
- Maintains agent template and personality stems
- Deposits deliverables in `Output/` for Claude Code

## Usage

From `Project Team (PT)/`:
```bash
cargo run -- pe
```

## Files

```
Agent Files/
├── Instructions.md          # Core behavior (v2.0)
├── Domain_Knowledge.md      # Prompt engineering patterns
├── Conventions.md           # Formatting rules
├── Evaluation_Framework.md  # Test cases, feedback
├── Roadmap.md               # Planned features
├── Bug_Report.md            # Known issues
├── Version_History.md       # Changelog
├── Templates/
│   ├── Agent Base Tools/    # Shared behavior patterns
│   └── Personality Stems/   # Coach, Helper, Assistant, SME
└── Arch/                    # Version archive
```

## Tools

- **File tools**: `read_file`, `write_file`, `list_files`, `save_history`, `write_output`
- **Queue tools**: `read_queue`, `write_queue`
- **Scaffolding**: `scaffold_project`, `list_agents`

## Version

v2.0 — 2025-01-15
