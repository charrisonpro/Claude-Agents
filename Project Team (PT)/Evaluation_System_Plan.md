# Evaluation System Implementation Plan

For Claudio (Claude Code). From PM, approved by Designer.

---

## Overview

We're building an evaluation system with two components:

1. **Evaluation_Framework.md** — A template for each agent that stores observations and hypothesis status
2. **Opus Evaluator** — A prompt that assesses interactions using Bayesian-framed "sentiment analysis"

The system uses Bayesian reasoning as a **conceptual discipline**, not statistical machinery. Conditional probability language structures how we think about success and failure.

---

## Component 1: Evaluation_Framework.md Template

### Purpose

Each agent gets an Evaluation_Framework.md that:
- Tracks hypotheses about agent performance (max 5 per agent)
- References stored interactions
- Records Opus evaluator judgments
- Supports external testers uploading feedback via GitHub

### File Structure

```yaml
---
# MACHINE-OWNED: Claudio reads/writes this block
agent: [agent name]
version: [agent version]
last_updated: [timestamp, set by Rust]

hypotheses:
  - id: H1
    condition: "[conditioning variable, e.g., 'understanding correct']"
    outcome: "[outcome variable, e.g., 'user need met']"
    statement: "P(outcome | condition) > 0.8"
    status: untested  # untested | testing | supported | refuted | inconclusive
    observations: []  # list of observation IDs relevant to this hypothesis

observations:
  - id: OBS-001
    timestamp: [set by Rust]
    task_type: [enum: routine | moderate | complex]
    interaction_ref: [filepath or ID to stored interaction]
    evaluator_judgment:
      understanding: [good | partial | poor]
      output_quality: [good | partial | poor]
      rationale: "[free text from Opus evaluator]"
---

# Evaluation Framework — [Agent Name]

## Hypotheses

Human-readable explanation of what we're testing and why.

### H1: [Short name]

**Statement:** P(user need met | understanding correct) > 0.8

**Rationale:** [Why this hypothesis matters]

**Status:** [Current status and summary of evidence]

---

## Notes

Space for PM synthesis, patterns observed, open questions.

---

## Version History

**v0.1** — Initial template.
```

### Rust Functions Needed

| Function | Input | Output | Notes |
|----------|-------|--------|-------|
| `read_evaluation(filepath)` | filepath | Parsed YAML struct | Extract front matter |
| `add_observation(filepath, observation)` | filepath, observation struct | Updated file | Rust sets timestamp |
| `update_hypothesis_status(filepath, id, status)` | filepath, hypothesis ID, new status | Updated file | |
| `get_observations_for_hypothesis(filepath, id)` | filepath, hypothesis ID | List of observations | Filter by hypothesis |
| `export_summary(filepath)` | filepath | Summary struct | For Global_Evaluation rollup |

### Enums

```rust
enum TaskType { Routine, Moderate, Complex }
enum Understanding { Good, Partial, Poor }
enum OutputQuality { Good, Partial, Poor }
enum HypothesisStatus { Untested, Testing, Supported, Refuted, Inconclusive }
```

### Constraints

- Max 5 hypotheses per file
- Observations append-only (no deletion)
- YAML front matter is machine-owned; markdown body is human-owned
- Claudio preserves markdown body when updating YAML

---

## Component 2: Opus Evaluator Prompt

### Purpose

An Opus-level prompt that reads a stored interaction and returns a structured judgment. Uses Bayesian conditional framing to discipline the assessment.

### Evaluator Prompt (Draft)

```markdown
# Interaction Evaluator

You are evaluating an interaction between a user and an AI agent. Your task is to assess two conditional questions:

## Context

**Agent:** [agent name]
**Task Type:** [routine | moderate | complex]
**Interaction:**

<interaction>
[Full input/output transcript]
</interaction>

## Assessment Framework

Answer these questions in order:

### 1. Understanding

Did the agent correctly understand what the user was asking for?

Consider:
- Did the agent identify the user's actual goal (not just surface request)?
- Did the agent recognize implicit requirements?
- Did the agent ask appropriate clarifying questions if needed?

**Rating:** [good | partial | poor]

**Evidence:** [Specific quotes or observations from the interaction]

### 2. Output Quality (Conditioned on Understanding)

**If understanding was correct:** Did the agent deliver what the user needed?

**If understanding was partial or poor:** Given what the agent *thought* the user wanted, was the response appropriate to that (mis)understanding?

This separation matters. An agent can:
- Understand correctly and succeed → good understanding, good output
- Understand correctly and fail → good understanding, poor output (capability gap)
- Misunderstand and "succeed" at wrong task → poor understanding, output quality assessed against the misunderstanding
- Misunderstand and fail even at wrong task → poor understanding, poor output

**Rating:** [good | partial | poor]

**Evidence:** [Specific quotes or observations]

### 3. Rationale

In 2-3 sentences, summarize the interaction quality. What would improve future performance?

## Output Format

Return your assessment as:

```yaml
understanding: [good | partial | poor]
output_quality: [good | partial | poor]
rationale: "[Your 2-3 sentence summary]"
```
```

### Integration Flow

1. Agent saves interaction to storage (filepath logged)
2. Claudio calls Opus evaluator with interaction content
3. Opus returns YAML judgment
4. Claudio parses judgment, appends to Evaluation_Framework.md observations
5. Claudio updates timestamp

---

## Component 3: Global_Evaluation.md

### Purpose

Lives in `Team Files/`. Provides system-level view across all agents.

### Structure

```yaml
---
last_updated: [timestamp]
agents:
  - name: PC
    version: v1.1
    status: testing
    observations_count: 0
    summary: "[PM-written blurb]"
  - name: PE
    version: v1.0
    status: untested
    observations_count: 0
    summary: "[PM-written blurb]"
---

# Global Evaluation

System-level synthesis across agents.

## Agent Status

| Agent | Version | Status | Observations | Summary |
|-------|---------|--------|--------------|---------|
| PC | v1.1 | testing | 0 | [blurb] |
| PE | v1.0 | untested | 0 | [blurb] |

## System-Level Hypotheses

Max 5 hypotheses about team/system performance.

### SH1: [Hypothesis]

[Description and status]

## PM Synthesis

[Running observations about cross-agent patterns, token economy, goal velocity]

---

## Version History

**v0.1** — Initial template.
```

### Rust Function

| Function | Input | Output | Notes |
|----------|-------|--------|-------|
| `rollup_to_global(agent_filepaths, global_filepath)` | List of agent eval files, global file path | Updated Global_Evaluation.md | Aggregates counts, preserves PM markdown |

---

## Implementation Order

1. **Define Rust structs** for Evaluation, Observation, Hypothesis
2. **Implement `read_evaluation`** — parse YAML front matter
3. **Implement `add_observation`** — append observation, set timestamp
4. **Create Evaluation_Framework.md template** in each agent's folder
5. **Draft Opus evaluator prompt** as a separate file (e.g., `Team Files/Evaluator_Prompt.md`)
6. **Test end-to-end** — manually save an interaction, run evaluator, record result
7. **Implement Global_Evaluation.md** rollup

---

## Open Questions for Designer

1. **Interaction storage:** Where do raw interaction transcripts live? Separate `Interactions/` folder per agent? Or inline in evaluation file?

2. **Evaluator invocation:** Is the Opus evaluator a tool Claudio calls via API? Or a manual step where Designer/PE runs the prompt?

3. **Hypothesis lifecycle:** Who decides when a hypothesis moves from "testing" to "supported/refuted"? PM? Designer? Threshold logic?

---

## Reference

- Bayesian framing provides discipline, not computation
- Conditional assessment: "given understanding, assess output"
- Rust handles timestamps, storage, parsing
- Opus handles judgment
- PM synthesizes patterns across agents
