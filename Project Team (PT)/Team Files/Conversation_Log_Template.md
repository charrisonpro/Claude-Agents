# Conversation Log Template

Standard template for session logging. Each agent maintains logs in their `History/` folder.

---

## File Naming

```
History/Log_YYYY-MM-DD_HH-MM.md
```

Example: `Log_2025-01-15_14-30.md`

---

## Log Structure

```markdown
# Session Log: [Date]

## Context
- **Agent**: [PM/PE/PC]
- **Started**: [Time]
- **Duration**: [Approximate]

## Summary
[1-3 sentence overview of what was accomplished]

## Key Decisions
- [Decision 1]
- [Decision 2]

## Actions Taken
- [Action with outcome]
- [Action with outcome]

## Open Items
- [ ] [Item carried forward]

## Handoffs
[Any work passed to other agents or queued]
```

---

## Model-Appropriate Logging

### Haiku (PC)
Keep logs minimal — 50 words max per section. Focus on:
- Tasks processed
- Queue changes
- Errors encountered

### Sonnet (PM)
Moderate detail — capture decisions and rationale:
- Decision context
- Options considered
- Why path was chosen
- Delegation actions

### Opus (PE)
Fuller detail where design decisions matter:
- Design rationale
- Trade-offs evaluated
- Framework changes
- Template updates

---

## When to Log

Log at session end using `save_history`:

```
save_history("Log_2025-01-15_14-30.md", content)
```

### Triggers
- Session ending naturally
- Major milestone reached
- Before long pause
- User requests log

---

## Procedural Hook

Add to agent Instructions.md:

```markdown
## Session Logging

At session end: Read `../Team Files/Conversation_Log_Template.md` → create summary → `save_history`
```
