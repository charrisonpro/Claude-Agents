# PE Framework Version History

This file tracks changes to the PE Framework itself. Target projects maintain their own separate version history files.

---

## v1.x Development Phase (Complete)

**v1.0** - Initial draft

**v1.1** - Added Foundational Sources section (experimental placement); expanded Claude Model Optimization to cover full 4.5 family with model-specific guidance; added Instruction Position Effects section with cognitive science grounding; updated Initialization Protocol to request target model; added Target Model to Project Context Format

**v1.2** - Externalized Foundational Sources and Version Tracking to separate files; removed redundant sections from main framework; added Reference Materials section (preferential use of Lesson_Notes.md for best practices and failure mode analysis); added Continuous Learning protocol (flag candidate lessons, propose additions); added Version Tracking Protocol (initialize tracking for all new systems, single version number covers entire system)

**v1.3** - Structural reorganization: created Conventions.md to separate formatting/naming standards from behavioral principles; moved Naming Convention, Version Tracking Protocol, and Edit Format Template from main prompt to Conventions.md; added Lesson Flagging Protocol to Conventions.md; added Reference Materials Management block to main prompt; clarified three-file system (Lesson_Notes for principles, Conventions for standards, Version_History for changes)

**v1.4** - Renamed Lesson_Notes.md to Best_Practices.md; restructured into General Best Practices, Domain-Specific Practices, and Case Studies sections; externalized Claude Model Optimization, Key Prompt Design Patterns, and Domain-Specific Patterns from main prompt to Best_Practices.md; added Quick Reference section to main prompt (mitigation for external file dependency); added first case study (Separation of Concerns); updated Reference Materials Management to four-file system

**v1.5** - Added Source Precedence rule to main prompt (project files canonical, chat artifacts are drafts); added case study "Chat Context vs. Project Files" to Best_Practices.md documenting stale-context failure mode

**v1.6** - Added Model Target section declaring Opus 4 as the framework's runtime model; positioned early (after opening paragraph) to leverage primacy effect; includes instruction for using Opus to model simpler target models; updated model version references from "4.5" to "4" in Project Context Format

**v1.6.1** - Split Version_History.md into two files: PE_Version_History.md (this file) for framework changes, and version tracking template moved to Conventions.md for target projects; updated File Organization table in Conventions.md; renamed from Version_History.md to PE_Version_History.md for explicit scope

**v1.6.2** - Added Source Precedence section to main prompt (was documented in v1.5 history but missing from actual prompt text); positioned after Model Target to leverage primacy effect; prevents stale chat context from overriding canonical file contents

---

## v2.0-alpha (Testing Phase)

**v2.0-alpha** - Consolidated all v1.x development; closed initial development phase; entered alpha testing

**v2.1** - Major restructure for Rust TUI implementation:
- Renamed `PE Framework Files/` → `Agent Files/`
- Renamed `PE_Framework.md` → `Instructions.md`
- Renamed `Best_Practices.md` → `Domain_Knowledge.md`
- Renamed `PE_Version_History.md` → `Version_History.md`
- Added `Toolkit.md` for tool documentation (context conservation)
- Added `History/` directory for conversation logs
- Added `Output/` directory for deliverables
- Trimmed Instructions.md to reference Toolkit.md instead of inline tool docs
- Added scaffolding tools: `scaffold_project`, `list_agents`, `list_project_structure`
- New agents created as sibling directories by default

### v2.1 Summary: What's in the Framework

**Core Files (Agent Files/):**
- Instructions.md — Main agent instructions
- Toolkit.md — Tool documentation (read on demand)
- Domain_Knowledge.md — Design patterns, model optimization, case studies
- Conventions.md — Formatting standards, templates, protocols
- Version_History.md — This changelog

**Directories:**
- History/ — Conversation logs (auto-saved)
- Output/ — Exported deliverables (packaged prompts, documentation)

**Key Capabilities:**
- Multi-model prompt engineering (Opus/Sonnet/Haiku optimization)
- Instruction positioning based on primacy/recency effects
- Modular system design for multi-step agents
- Case study documentation for failure mode learning
- Source precedence rules for long-conversation accuracy

**Documented Failure Modes Addressed:**
1. Separation of concerns (v1.3) — split behavior/conventions/knowledge
2. Chat context staleness (v1.5, v1.6.2) — source precedence rules
3. External file dependency (v1.4) — Quick Reference fallback

**Testing Focus for v2.0:**
- Validate framework on new target agent projects
- Identify additional failure modes through real use
- Refine Best_Practices.md with new case studies
