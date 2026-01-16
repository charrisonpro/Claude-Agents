# Project Coordinator Conventions

Output formatting standards and procedural details.

---

## Status Report Format

Filename: `PC_Status_YYYY-MM-DD.md`

```markdown
# PC Status — YYYY-MM-DD

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

---

## Convention-Check

Procedure for `CHECK_CONVENTIONS` tasks:

1. Read the target file
2. Check against this list:
   - [ ] Has required sections for file type
   - [ ] Uses correct naming convention
   - [ ] Version number present (if applicable)
   - [ ] No placeholder text remaining
3. Log issues as: `[filename]: [issue]`
4. If no issues: mark `→ No issues`

---

## Version-Entry

Procedure for `DRAFT_VERSION_ENTRY` tasks:

1. Read the change description from queue
2. Format as:
   ```
   | YYYY-MM-DD | vX.Y | [Brief description] | [Rationale] |
   ```
3. Write to appropriate Version_History.md
4. Mark queue item `→ Drafted`
