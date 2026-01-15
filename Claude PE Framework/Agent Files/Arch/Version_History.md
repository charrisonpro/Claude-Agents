## Version Tracking

When prompts are updated, track:

```
## Version History

**v1.0** - [Date] - Initial draft
**v1.1** - [Date] - [What changed and why]
**v2.0** - [Date] - [Major restructure: what changed and why]
```

### Note Significant Changes

- Structural changes (step reordering, new steps added)
- Logic changes (scoring methods, decision rules)
- Tone/style changes
- Bug fixes or failure mode patches

---

## Version History

**v1.0** - Initial draft

**v1.1** - Added Foundational Sources section (experimental placement); expanded Claude Model Optimization to cover full 4.5 family with model-specific guidance; added Instruction Position Effects section with cognitive science grounding; updated Initialization Protocol to request target model; added Target Model to Project Context Format

**v1.2** - Externalized Foundational Sources and Version Tracking to separate files; removed redundant sections from main framework; added Reference Materials section (preferential use of Lesson_Notes.md for best practices and failure mode analysis); added Continuous Learning protocol (flag candidate lessons, propose additions); added Version Tracking Protocol (initialize tracking for all new systems, single version number covers entire system)

**v1.3** - Structural reorganization: created Conventions.md to separate formatting/naming standards from behavioral principles; moved Naming Convention, Version Tracking Protocol, and Edit Format Template from main prompt to Conventions.md; added Lesson Flagging Protocol to Conventions.md; added Reference Materials Management block to main prompt; clarified three-file system (Lesson_Notes for principles, Conventions for standards, Version_History for changes)

**v1.4** - Renamed Lesson_Notes.md to Best_Practices.md; restructured into General Best Practices, Domain-Specific Practices, and Case Studies sections; externalized Claude Model Optimization, Key Prompt Design Patterns, and Domain-Specific Patterns from main prompt to Best_Practices.md; added Quick Reference section to main prompt (mitigation for external file dependency); added first case study (Separation of Concerns); updated Reference Materials Management to four-file system

**v1.5** - Added Source Precedence rule to main prompt (project files canonical, chat artifacts are drafts); added case study "Chat Context vs. Project Files" to Best_Practices.md documenting stale-context failure mode

**v1.6** - Added Model Target section declaring Opus 4 as the framework's runtime model; positioned early (after opening paragraph) to leverage primacy effect; includes instruction for using Opus to model simpler target models; updated model version references from "4.5" to "4" in Project Context Format
