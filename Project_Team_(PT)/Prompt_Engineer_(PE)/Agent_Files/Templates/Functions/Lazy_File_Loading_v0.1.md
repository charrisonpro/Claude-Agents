# Lazy File Loading Function (v0.1)

*Standalone module. Test in isolation before integration.*

---

## Purpose

Agents load support files only when tasks require them, not at initialization. This conserves context, reduces noise, and ensures agents reference current file state rather than stale initialization snapshots.

---

## Principle

**Read on demand, not on startup.**

At initialization, agents receive their core instructions only. Support files (Domain Knowledge, Conventions, Toolkit, etc.) are loaded when a task requires that specific knowledge—not before.

This is analogous to lazy evaluation: defer computation (file reading) until the value (file contents) is actually needed.

---

## When to Load Files

Load a file when:

- A task explicitly requires information in that file
- You need to verify current state before modifying
- The user references content you don't have in context

Do NOT load a file when:

- You can proceed with instructions already in context
- The task doesn't require that file's domain
- You're "just checking" without specific need

---

## File Categories and Triggers

| File Type | Load When... |
|-----------|--------------|
| Domain_Knowledge.md | Task requires domain-specific patterns, model optimization guidance, or case study reference |
| Conventions.md | Producing formatted output, versioning decisions, naming new components |
| Toolkit.md | Using tools, especially unfamiliar operations or scaffolding |
| Version_History.md | Logging changes, checking what changed when |
| Evaluation.md | Running tests, reviewing evaluation cases, logging feedback |

---

## Loading Protocol

When you determine a file is needed:

1. **State the need:** "This task requires [X], which is in [File]."
2. **Load the file:** Read the relevant file.
3. **Extract what's needed:** Don't summarize the whole file—pull the specific information relevant to the task.
4. **Proceed:** Use the information, complete the task.

### After Loading

- The file contents are now in context for this session
- No need to re-read unless you suspect the file has changed
- If you modify a file, your context reflects the old state until you re-read

---

## Anti-Patterns

### Startup Loading

❌ "Let me read all my support files to get oriented."

✅ "I have my core instructions. I'll load support files as tasks require them."

### Speculative Loading

❌ "I might need Domain Knowledge, so let me read it just in case."

✅ "This task is about [X]. Do I need Domain Knowledge for that? [If yes, load. If no, proceed.]"

### Re-Reading Without Cause

❌ "Let me re-read Conventions to make sure I remember..."

✅ "I loaded Conventions earlier this session. Unless it's changed or I need something I didn't extract, I'll use what I have."

---

## Integration Notes

### Instruction Placement

In agent instructions, include a brief note:

```markdown
## Support Files

Load these files when tasks require them—not at startup:

- **Domain_Knowledge.md** — Design patterns, model optimization, case studies
- **Conventions.md** — Formatting standards, version numbering, templates
- **Toolkit.md** — Tool documentation, scaffolding reference

Read on demand. Conserve context.
```

### Trigger Phrase Option

If testing trigger-phrase architecture, the agent might say:

> "This requires conventions guidance. Loading Conventions.md § [relevant section]."

Then load and extract the specific section needed.

---

## Example Interaction

**User:** "Create a new version history entry for the changes we just made."

**Agent thinking:** This requires version numbering conventions. I need Conventions.md.

**Agent:** "I'll check the version numbering conventions to format this correctly."

*[Loads Conventions.md, extracts version numbering rules]*

**Agent:** "Based on conventions: this is a minor change (added section, no structural overhaul), so we increment v1.0 → v1.1. Here's the entry:

**v1.1** — [Date] — Added [feature]; refined [section]."

---

## Version History

**v0.1** — Initial draft. Core principle, loading triggers, anti-patterns.
