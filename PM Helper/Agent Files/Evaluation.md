# Evaluation Framework

Systematic testing and feedback tracking for PM Helper. Maintained by PE Framework and Claude Code.

---

## Evaluation Dimensions

| Dimension | Description | Weight |
|-----------|-------------|--------|
| Task Processing | Correctly identifies and processes queue items | High |
| Output Accuracy | Status reports contain correct information | High |
| Queue Markup | Properly marks completed items with outcomes | Medium |
| Brevity | Keeps responses concise per Haiku optimization | Medium |
| Heavy Op Detection | Flags expensive operations with alternatives | Low |

---

## Test Cases

### Case 1: Empty Queue Processing

**Input:** "Process all pending tasks in the queue."

**Expected Behavior:** Reads queue, reports no pending tasks, does not create empty status report

**Actual Behavior:** [Fill during testing]

**Result:** Pass / Fail / Partial

**Notes:** [Observations]

---

### Case 2: Mixed Task Types

**Input:** Queue with TRACK_QUESTION, LOG_DECISION, and FLAG_HEAVY_OP items

**Expected Behavior:** Processes each correctly, marks complete with appropriate outcome tags

**Actual Behavior:** [Fill during testing]

**Result:** Pass / Fail / Partial

**Notes:** [Observations]

---

### Case 3: Status Report Generation

**Input:** "Generate a full status report."

**Expected Behavior:** Creates PM_Status_[Date].md in Output/ with all sections

**Actual Behavior:** [Fill during testing]

**Result:** Pass / Fail / Partial

**Notes:** [Observations]

---

## Feedback Log

Running observations from testing. Date-stamped, newest first.

| Date | Observer | Observation | Severity | Status |
|------|----------|-------------|----------|--------|
| YYYY-MM-DD | [PE/Claude Code] | [What happened] | [Critical/Minor/Note] | [Open/Resolved] |

---

## Radiation Notes

Learnings that may propagate beyond this agent.

### Propagate to Stem (Generalize)

Observations that apply to the base framework or sibling agents.

- [Observation] → [Suggested change to stem]

### Keep Local (Domain-Specific)

Observations that belong only in this specialization.

- Haiku token limits require shorter context windows
- Queue processing is single-purpose, doesn't need complex reasoning

### Received from Stem

Updates inherited from the base framework.

- [Date]: [What changed]

---

## Version Correlation

Track which agent version was tested.

| Eval Date | Agent Version | Notes |
|-----------|---------------|-------|
| YYYY-MM-DD | v1.0 | Initial release |
