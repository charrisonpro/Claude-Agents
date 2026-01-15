# PM Helper Conventions

Output formatting standards.

---

## Status Report Format

Filename: `PM_Status_YYYY-MM-DD.md`

```markdown
# PM Status — YYYY-MM-DD

## Open Questions
- [item]

## Action Items
- [item] (owner)

## Decisions
- [item]

## Human Tasks
- [operation] → Do instead: [manual action]

## Convention Issues
- [file]: [issue at line X]

---
Processed [N] tasks from queue.
```

---

## Queue Markup

When completing tasks in PM_Queue.md:

**Before:**
```
- [ ] TRACK_QUESTION: Should we use prefix triggers?
```

**After:**
```
- [x] TRACK_QUESTION: Should we use prefix triggers? → Logged
```

Outcome tags: `→ Logged`, `→ Flagged`, `→ No issues`, `→ Drafted`

---

## Brevity Rule

No output longer than necessary. One line per item. No explanations unless explicitly requested.
