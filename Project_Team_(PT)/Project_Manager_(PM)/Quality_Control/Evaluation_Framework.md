# Project Manager Evaluation Framework

Systematic testing and feedback tracking.

---

## Evaluation Dimensions

| Dimension | Description | Weight |
|-----------|-------------|--------|
| Decision Quality | Makes clear, well-reasoned recommendations | High |
| User Alignment | Understands and reflects user's goals | High |
| Delegation Accuracy | Routes work to correct agent | Medium |
| Communication Clarity | Concise, actionable responses | Medium |
| Queue Discipline | Appropriately uses async queue | Low |

---

## Test Cases

### Case 1: Ambiguous Goal

**Input:** "I want to make the agents better"

**Expected Behavior:**
- Ask clarifying questions (which agents? what aspect?)
- Propose 2-3 specific improvement paths
- Recommend one with rationale

**Actual Behavior:** [Fill during testing]

**Result:** Pass / Fail / Partial

**Notes:** [Observations]

---

### Case 2: Multi-Agent Coordination

**Input:** "Add a new language coach agent"

**Expected Behavior:**
- Break into steps (scope → design → implement)
- Identify PE for design, Claude Code for implementation
- Propose timeline/sequence

**Actual Behavior:** [Fill during testing]

**Result:** Pass / Fail / Partial

**Notes:** [Observations]

---

### Case 3: Decision Under Uncertainty

**Input:** "Should we use Haiku or Sonnet for the research assistant?"

**Expected Behavior:**
- Present trade-offs (cost vs capability)
- Ask about use case specifics
- Make recommendation with reasoning

**Actual Behavior:** [Fill during testing]

**Result:** Pass / Fail / Partial

**Notes:** [Observations]

---

## Feedback Log

| Date | Observer | Observation | Severity | Status |
|------|----------|-------------|----------|--------|
| YYYY-MM-DD | [Tester] | [What happened] | [Critical/Minor/Note] | [Open/Resolved] |

---

## Radiation Notes

### Propagate to Stem
- [Observation] → [Suggested change]

### Keep Local
- Strategic decision-making is PM-specific
- User sounding board role doesn't transfer to other agents

### Received from Stem
- [Date]: [What changed]

---

## Version Correlation

| Eval Date | Agent Version | Notes |
|-----------|---------------|-------|
| YYYY-MM-DD | v1.0 | Initial release |
