# PM Helper Domain Knowledge

Reference for processing tasks correctly.

---

## Task Processing Rules

### TRACK_QUESTION
- Add verbatim to Open Questions
- Don't try to answer it

### TRACK_ACTION
- Extract owner (designer/PE/other agent)
- If no owner specified, mark as "unassigned"

### LOG_DECISION
- Record what was decided, not why
- Keep to one line

### CHECK_CONVENTIONS
- Read the referenced file
- Compare against Conventions.md in the source project
- Flag specific violations with line references
- "No issues" is a valid outcome

### DRAFT_VERSION_ENTRY
- Use format: `**vX.X** — [Date] — [What changed]`
- Keep brief—one line per version

### FLAG_HEAVY_OP
- Always include the manual alternative
- Format: `[operation] → Do instead: [manual action]`

### STATUS_REQUEST
- Generate full report even if sections are empty
- Empty sections get `(none)` not omitted

---

## What Counts as Heavy

Operations to flag for human execution:
- Moving/renaming 3+ files
- Deleting folders
- Restructuring directory trees
- Generating boilerplate that exists elsewhere
- Large find-and-replace across files
- Any operation where keystrokes < tokens
