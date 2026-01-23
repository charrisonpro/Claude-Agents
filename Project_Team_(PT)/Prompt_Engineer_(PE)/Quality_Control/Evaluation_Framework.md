# Evaluation Framework

Systematic testing and feedback tracking for this agent. Maintained by PE Framework and Claude Code.

---

## Evaluation Dimensions

| Dimension | Description | Weight |
|-----------|-------------|--------|
| [e.g., Tone Accuracy] | [What we're measuring] | [High/Medium/Low] |
| [e.g., Task Completion] | [What we're measuring] | [High/Medium/Low] |
| [e.g., Challenge Calibration] | [What we're measuring] | [High/Medium/Low] |

---

## Test Cases

### Case 1: [Scenario Name]

**Input:** [What the user says or does]

**Expected Behavior:** [What the agent should do]

**Actual Behavior:** [Fill during testing]

**Result:** Pass / Fail / Partial

**Notes:** [Observations]

---

### Case 2: [Scenario Name]

**Input:**

**Expected Behavior:**

**Actual Behavior:**

**Result:**

**Notes:**

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

- [Observation] → [Why it's local]

### Received from Stem

Updates inherited from the base framework.

- [Date]: [What changed]

---

## Version Correlation

Track which agent version was tested.

| Eval Date | Agent Version | Notes |
|-----------|---------------|-------|
| YYYY-MM-DD | v0.x | [What was tested] |
