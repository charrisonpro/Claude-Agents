# Project Manager Conventions

Formatting rules and procedures.

---

## Decision-Logging

When logging a decision:

1. Write to queue:
   ```markdown
   - [ ] LOG_DECISION: [Brief decision statement]
   ```

2. For significant decisions, also note in conversation:
   ```
   **Decision**: [What we decided]
   **Trade-offs**: [What we considered]
   **Next**: [Immediate action]
   ```

---

## PE-Handoff

When handing work to PE:

1. Provide context:
   - Goal of the prompt/agent
   - Target model (Opus/Sonnet/Haiku)
   - Key constraints or requirements

2. Format:
   ```
   **PE Task**: [What needs designing]
   **Goal**: [What success looks like]
   **Model**: [Target model]
   **Constraints**: [Any limitations]
   ```

3. PE will deposit deliverable in `PE/Output/`

---

## Queue-Task

When adding to PC queue:

```markdown
- [ ] TASK_TYPE: Description (owner: PM)
```

Valid types:
- `TRACK_QUESTION` - Open question to track
- `TRACK_ACTION` - Action item with owner
- `LOG_DECISION` - Decision to record
- `FLAG_HEAVY_OP` - Expensive operation with alternative
- `STATUS_REQUEST` - Request full status report

---

## Meeting Notes Format

When summarizing a planning session:

```markdown
# Session Summary — YYYY-MM-DD

## Discussed
- [Topic 1]
- [Topic 2]

## Decided
- [Decision 1]
- [Decision 2]

## Next Steps
- [ ] [Action] (owner)
- [ ] [Action] (owner)

## Parked
- [Item for later]
```

---

## Priority Framework

When prioritizing work:

| Priority | Criteria | Action |
|----------|----------|--------|
| P0 | Blocking all progress | Handle immediately |
| P1 | Blocking current goal | Handle this session |
| P2 | Important but not blocking | Queue or schedule |
| P3 | Nice to have | Backlog |

---

## Communication Defaults

- Lead with recommendation, not options
- Keep status updates to 3 bullets max
- Use tables for comparisons
- Bold key terms and decisions
