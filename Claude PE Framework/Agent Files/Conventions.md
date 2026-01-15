# PE Framework Conventions

Standards for formatting, naming, and structure across all prompt engineering work.

---

## Naming Convention

| Level | Name | Refers to |
|-------|------|-----------|
| Meta | **PE Framework** | These instructions — the Prompt Engineer system prompt |
| Work | **Target Prompt** or **[Agent Name]** | The prompt system being built or reviewed |

Use these labels explicitly when discussing changes to avoid conflation.

---

## Version Numbering

**Single version number** covers the entire system (main prompt + all support files).

| Change Type | Increment | Example | Use When |
|-------------|-----------|---------|----------|
| Patch | v1.0 → v1.0.1 | Bug fixes, typos, clarifications | No behavior change |
| Minor | v1.0 → v1.1 | New sections, refined logic | Additive changes |
| Major | v1.x → v2.0 | Structural overhaul | Breaking changes |

---

## Edit Recommendation Format

```
### Edit [N]: [Specific Issue]

**Current Problem:** [What breaks or confuses agents]

**Suggested Change:** [Concrete replacement text]

**Rationale:** [Why this improves convergence toward goal]
```

Prioritize impact over hitting a number (0-3 edits as warranted).

---

## Lesson Flagging Protocol

During work, the designer may say **"flag this lesson"** (or similar). This means:

1. **Capture the case study:** Summarize what just happened—the problem, the fix, and the outcome
2. **Generalize the principle:** Extract the underlying pattern that applies beyond this specific case
3. **Format for Best_Practices.md (Case Studies section):**

```
### [Principle Name]

**Observation:** [What happened in the specific case]

**Principle:** [The generalized lesson]

**Design Implication:** [How to apply this when building prompts]
```

4. **Propose the addition** to Best_Practices.md (don't add silently—confirm first)

**Trigger phrases:** "flag this lesson," "add this to lessons," "that's a lesson," "note that pattern"

---

## Version Tracking for Target Projects

Each target project maintains its own version history, separate from the PE Framework's history.

### Template for New Projects

```markdown
## Version History

**v1.0** - [Date] - Initial draft
**v1.1** - [Date] - [What changed and why]
**v2.0** - [Date] - [Major restructure: what changed and why]
```

### What to Track

- Structural changes (step reordering, new steps added)
- Logic changes (scoring methods, decision rules)
- Tone/style changes
- Bug fixes or failure mode patches

---

## File Organization

| File | Contains | Update Frequency |
|------|----------|------------------|
| Main prompt (.md) | Agent instructions | Per iteration |
| Best_Practices.md | Design patterns, optimization guidance, case studies | When patterns emerge |
| Conventions.md | Formatting standards, templates | When standards solidify |
| PE_Version_History.md | PE Framework change log | Each PE Framework change |
