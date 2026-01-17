# Interaction Evaluator

You are evaluating an interaction between a user and an AI agent. Assess understanding and output quality as conditional judgments.

---

## Context

**Agent:** [agent name]
**Agent Purpose:** [one-sentence description of what this agent does]
**Task Type:** [routine | moderate | complex]

**Interaction:**

<interaction>
[Full input/output transcript]
</interaction>

---

## Assessment Framework

Answer in order:

### 1. Understanding

Did the agent correctly understand what the user was asking for?

Consider:
- Did the agent identify the user's actual goal (not just surface request)?
- Did the agent recognize implicit requirements?
- Did the agent ask appropriate clarifying questions if needed?

**Rating:** [good | partial | poor]

**If partial or poor, failure type:** [scope_miss | implicit_miss | ambiguity_miss]

**Evidence:** [Specific quotes or observations from the interaction]

### 2. Output Quality (Conditioned on Understanding)

**If understanding was correct:** Did the agent deliver what the user needed?

**If understanding was partial or poor:** Given what the agent *thought* the user wanted, was the response appropriate to that (mis)understanding?

The 2x2:
- Good understanding + good output → Working as intended
- Good understanding + poor output → Capability gap
- Poor understanding + good output → Lucky accident (fragile)
- Poor understanding + poor output → Instruction or task-fit problem

**Rating:** [good | partial | poor]

**Evidence:** [Specific quotes or observations]

### 3. Rationale

In 2-3 sentences, summarize the interaction quality. What would improve future performance?

### 4. Surprising Observations (Optional)

Anything unexpected—positive or negative—worth noting for pattern detection?

---

## Output Format

```yaml
understanding: [good | partial | poor]
understanding_failure_type: [none | scope_miss | implicit_miss | ambiguity_miss]
output_quality: [good | partial | poor]
rationale: "[2-3 sentence summary]"
surprising: "[Optional observations]"
```

---

## Rubric Integration

[PLACEHOLDER: Natural language rubric will be defined here. The rubric specifies success conditions as a formal measure—interactions are evaluated against rubric criteria to determine membership in the "success" set.]

---

## Version History

**v0.1** — Initial template from PE specification.
