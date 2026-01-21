# Agent File Logic (v1.0)

Base behavior patterns shared by all Project Team agents. Claude Code coordinates implementation; agents follow these conventions for file interaction.

---

## Standard Agent File Structure

Every agent maintains these files in `Agent Files/`:

| File | Purpose | Update Frequency |
|------|---------|------------------|
| `Instructions.md` | Core behavior, identity, workflow | Rare (version bumps) |
| `Domain_Knowledge.md` | Facts, patterns, learnings | As discoveries occur |
| `Conventions.md` | Formatting rules, templates, procedures | When patterns stabilize |
| `Evaluation_Framework.md` | Test cases, feedback log, metrics | After testing rounds |
| `Roadmap.md` | Planned features, improvements | When priorities shift |
| `Bug_Report.md` | Known issues, reproduction steps | When bugs found/fixed |
| `Version_History.md` | Changelog of agent evolution | Every change |

---

## Modular Instruction Pattern

Agents keep Instructions.md lean by deferring procedural details to Conventions.md.

### Hook Syntax

In Instructions.md:
```
When [trigger condition], read `Conventions.md#[section]` for the procedure, execute it, then resume normal operation.
```

### Example Hooks

```markdown
## Procedural Hooks

- **Save a lesson**: Read `Conventions.md#Lesson-Capture` → execute → resume
- **Log a decision**: Read `Conventions.md#Decision-Logging` → execute → resume
- **Flag heavy op**: Read `Conventions.md#Heavy-Op-Protocol` → execute → resume
```

### Why This Works

1. Instructions.md stays under context limits
2. Procedures update without touching core identity
3. Haiku agents especially benefit from smaller base prompts

---

## Claude Code Coordination

Agents interact with Claude Code through defined channels:

### Output Handoff

When an agent produces artifacts for implementation:
1. Write to `Output/` folder with descriptive filename
2. Include implementation notes at top of file
3. Claude Code picks up and implements in local system

### Queue Communication

Agents write to shared queues for async processing:
```markdown
- [ ] TASK_TYPE: Description (owner: agent_name)
```

PC processes queues during downtime. Critical work stays in-conversation.

### File Ownership

| Owner | Writes To | Reads From |
|-------|-----------|------------|
| PE | Own Agent Files, Output/ | All agent files (read-only) |
| PC | Own Agent Files, Queues | PE Output/, own files |
| Claude Code | All local files | Agent Output/ folders |

---

## Version Tracking

All agents use semantic versioning in Instructions.md header:

```
# [Agent Name] (v[major].[minor])
```

- **Major**: Role or workflow change
- **Minor**: Capability additions, refinements

Update Version_History.md with every change:
```markdown
| Date | Version | Change | Rationale |
|------|---------|--------|-----------|
| YYYY-MM-DD | v1.1 | Added X | Because Y |
```

---

## Radiation Protocol

Learnings propagate through the system:

### Propagate Up (to PE)
- Patterns that apply to all agents
- Framework-level improvements

### Keep Local
- Domain-specific optimizations
- Agent-unique workflows

### Receive Down (from PE)
- Updated conventions
- New procedural hooks

Track in Evaluation_Framework.md under "Radiation Notes".

---

## Quick Reference

### When to Update Each File

| Trigger | Update |
|---------|--------|
| New capability added | Instructions.md, Version_History.md |
| Pattern discovered | Domain_Knowledge.md |
| Formatting decision made | Conventions.md |
| Test completed | Evaluation_Framework.md |
| Future feature identified | Roadmap.md |
| Issue found | Bug_Report.md |
| Any file changed | Version_History.md |

### File Size Targets (for Haiku optimization)

| File | Target | Max |
|------|--------|-----|
| Instructions.md | < 2000 tokens | 3000 |
| Domain_Knowledge.md | < 1500 tokens | 2500 |
| Conventions.md | < 1000 tokens | 2000 |
| Others | No hard limit | — |
