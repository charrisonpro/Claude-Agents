# Evaluation System Specification

*For Claudio (Claude Code). From PM, reviewed by PE.*

---

## Overview

Two-component evaluation system:

1. **Evaluation_Framework.md** — Per-agent template storing hypotheses and observations
2. **Opus Evaluator** — Prompt that assesses interactions using conditional reasoning

Bayesian framing provides conceptual discipline, not statistical machinery. Conditional probability language structures thinking about success and failure.

---

## Component 1: Evaluation_Framework.md Template

### Purpose

Each agent gets an Evaluation_Framework.md that:
- Tracks hypotheses about agent performance (max 5)
- References stored interactions
- Records Opus evaluator judgments
- Supports external tester feedback via GitHub

### File Structure

```yaml
---
# MACHINE-OWNED: Claudio reads/writes this block
agent: [agent name]
version: [agent version]
last_updated: [timestamp, set by Rust]

hypotheses:
  - id: H1
    condition: "[conditioning variable]"
    outcome: "[outcome variable]"
    statement: "P(outcome | condition) > 0.8"
    status: untested  # untested | testing | supported | refuted | inconclusive
    observation_count: 0
    consistent_count: 0
    inconsistent_count: 0
    observations: []  # list of observation IDs

observations:
  - id: OBS-001
    timestamp: [set by Rust]
    task_type: routine  # routine | moderate | complex
    interaction_ref: [filepath to stored interaction]
    evaluator_judgment:
      understanding: good  # good | partial | poor
      understanding_failure_type: none  # none | scope_miss | implicit_miss | ambiguity_miss
      output_quality: good  # good | partial | poor
      rationale: "[free text from Opus evaluator]"
      surprising: "[optional: unexpected observations]"
---

# Evaluation Framework — [Agent Name]

## Hypotheses

Human-readable explanation of what we're testing and why.

### H1: [Short name]

**Statement:** P(outcome | condition) > threshold

**Rationale:** [Why this hypothesis matters]

**Status:** [Current status and summary of evidence]

---

## Radiation Log

Observations that should propagate to other components:

| Date | Observation | Target | Status |
|------|-------------|--------|--------|
| | | | |

---

## Notes

Space for PM synthesis, patterns observed, open questions.

---

## Version History

**v0.1** — Initial template.
```

---

## Enums

```rust
enum TaskType { Routine, Moderate, Complex }
enum Understanding { Good, Partial, Poor }
enum UnderstandingFailureType { None, ScopeMiss, ImplicitMiss, AmbiguityMiss }
enum OutputQuality { Good, Partial, Poor }
enum HypothesisStatus { Untested, Testing, Supported, Refuted, Inconclusive }
```

### Task Type Definitions

| Task Type | Characteristics | Example |
|-----------|-----------------|---------|
| **Routine** | Clear input, clear output, single-step, no judgment | "Conjugate this verb" |
| **Moderate** | Multiple steps OR domain knowledge OR some ambiguity | "Explain when to use voseo vs. usted" |
| **Complex** | Ambiguous goals, synthesis required, judgment-heavy | "Help me sound more natural" |

### Understanding Failure Types

| Type | Description |
|------|-------------|
| **none** | Understanding was good |
| **scope_miss** | Agent addressed wrong scope (too narrow or broad) |
| **implicit_miss** | Agent missed unstated requirements |
| **ambiguity_miss** | Agent picked wrong interpretation without clarifying |

---

## Rust Functions

| Function | Input | Output | Notes |
|----------|-------|--------|-------|
| `read_evaluation(filepath)` | filepath | Parsed YAML struct | Extract front matter |
| `add_observation(filepath, observation)` | filepath, observation struct | Updated file | Rust sets timestamp |
| `update_hypothesis_status(filepath, id, status)` | filepath, hypothesis ID, new status | Updated file | |
| `update_hypothesis_counts(filepath, id, consistent)` | filepath, hypothesis ID, bool | Updated file | Increments appropriate counter |
| `get_observations_for_hypothesis(filepath, id)` | filepath, hypothesis ID | List of observations | Filter by hypothesis |
| `check_status_thresholds(filepath, id)` | filepath, hypothesis ID | Suggested status or None | See thresholds below |
| `export_summary(filepath)` | filepath | Summary struct | For Global_Evaluation rollup |

---

## Hypothesis Status Thresholds

| Transition | Criteria |
|------------|----------|
| untested → testing | First observation recorded |
| testing → supported | 5+ observations, >80% consistent |
| testing → refuted | 5+ observations, >40% inconsistent |
| testing → inconclusive | 10+ observations, neither threshold met |

`check_status_thresholds` returns suggested status when threshold met; PM confirms transition.

---

## Hypothesis Design Criteria

A hypothesis must be:

1. **Falsifiable** — An observation could refute it
2. **Actionable** — If refuted, we know what to change
3. **Scoped** — Tests one thing, not a bundle

### Hypothesis Templates by Agent Type

| Agent Type | Hypothesis | Rationale |
|------------|------------|-----------|
| **Coach (Language)** | P(learner progresses \| level correctly assessed) > 0.8 | Correct level → learning follows |
| **Coach (Language)** | P(error addressed effectively \| error detected) > 0.9 | Detection without correction is useless |
| **Coach (Language)** | P(cultural context included \| dialect feature introduced) > 0.85 | Language without culture is skeleton |
| **Coach (Language)** | P(learner continues engagement \| trust behaviors present) > 0.8 | Warmth predicts retention |
| **Coach (Weightlifting)** | P(program executable \| constraints gathered) > 0.9 | Can't program what you don't know |
| **Coach (Weightlifting)** | P(progressive overload present \| program delivered) > 0.95 | Non-negotiable principle |
| **Assistant** | P(task captured \| user mentions task) > 0.95 | Capture is the core job |
| **Assistant** | P(conflict surfaced \| scheduling conflict exists) > 0.9 | Silent conflicts become crises |
| **Assistant** | P(confirmation obtained \| commitment made) > 0.95 | Never commit without confirmation |
| **SME Researcher** | P(accurate information \| source cited) > 0.9 | Citations must be trustworthy |
| **SME Researcher** | P(scope appropriate \| research question clarified) > 0.85 | Garbage in, garbage out |
| **Data Analyst** | P(correct method \| problem type identified) > 0.85 | Method follows problem |
| **Data Analyst** | P(interpretation valid \| analysis complete) > 0.8 | Numbers without meaning are noise |
| **PM** | P(correct delegation \| task complexity assessed) > 0.85 | Right task to right agent |
| **PM** | P(goal clarified \| ambiguity detected) > 0.9 | Surface ambiguity, don't bury it |
| **PM Helper** | P(task processed correctly \| task type identified) > 0.95 | Haiku needs clear categories |
| **Interview Function** | P(sufficient context gathered \| interview completed) > 0.85 | Interview should produce usable input |
| **Interview Function** | P(user feels heard \| trust behaviors present) > 0.8 | Process matters, not just output |

---

## Component 2: Opus Evaluator Prompt

Store as `Team_Files/Evaluator_Prompt.md`.

```markdown
# Interaction Evaluator

You are evaluating an interaction between a user and an AI agent. Assess understanding and output quality as conditional judgments.

## Context

**Agent:** [agent name]
**Agent Purpose:** [one-sentence description of what this agent does]
**Task Type:** [routine | moderate | complex]

**Interaction:**

<interaction>
[Full input/output transcript]
</interaction>

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

## Output Format

```yaml
understanding: [good | partial | poor]
understanding_failure_type: [none | scope_miss | implicit_miss | ambiguity_miss]
output_quality: [good | partial | poor]
rationale: "[2-3 sentence summary]"
surprising: "[Optional observations]"
```
```

---

## Component 3: Interaction Storage

### Structure

```
agents/
  coaches/
    Spanish_Coach_CR_v1.0.md
    Spanish_Coach_CR_Evaluation.md
    Spanish_Coach_CR_Interactions/
      INT-001.md
      INT-002.md
```

### Interaction File Template

```markdown
# Interaction INT-[number]

**Submitted by:** [username or "internal"]
**Timestamp:** [set by Rust or submitter]
**Task Type:** [routine | moderate | complex]
**User Goal:** [one-sentence summary]

---

## Transcript

**User:** [input]

**Agent:** [output]

**User:** [follow-up if any]

**Agent:** [response if any]

---

## Tester Notes

[Optional: What worked? What didn't? What surprised you?]

---

## Evaluator Judgment

[Leave blank until evaluator runs — then paste YAML output]
```

---

## Component 4: Global_Evaluation.md

Lives in `Team_Files/`. System-level view across agents.

```yaml
---
last_updated: [timestamp]
agents:
  - name: Spanish_Coach_CR
    version: v1.0
    status: testing
    observation_count: 0
    hypothesis_summary: "0/3 supported"
  - name: Japanese_Coach_Kyoto
    version: v1.0
    status: untested
    observation_count: 0
    hypothesis_summary: "0/3 supported"
---

# Global Evaluation

## Agent Status

| Agent | Version | Status | Observations | Hypotheses |
|-------|---------|--------|--------------|------------|
| Spanish_Coach_CR | v1.0 | testing | 0 | 0/3 supported |

## System-Level Hypotheses

Max 5 hypotheses about team/system performance.

### SH1: [Hypothesis]

[Description and status]

## PM Synthesis

[Cross-agent patterns, token economy observations, goal velocity]

---

## Version History

**v0.1** — Initial template.
```

### Rollup Function

| Function | Input | Output | Notes |
|----------|-------|--------|-------|
| `rollup_to_global(agent_filepaths, global_filepath)` | List of agent eval files, global file path | Updated Global_Evaluation.md | Aggregates counts, preserves PM markdown |

---

## External Tester Protocol

### Workflow

1. Fork repo or create branch
2. Create interaction file in agent's `Interactions/` folder using template
3. Add observation stub to `Evaluation_Framework.md`:
   ```yaml
   - id: OBS-[next]
     timestamp: [leave blank]
     task_type: [your assessment]
     interaction_ref: Interactions/INT-[number].md
     evaluator_judgment: pending
   ```
4. Submit PR
5. Claudio or PE runs Opus evaluator, fills judgment
6. Rust sets timestamp
7. PR merged with complete observation

---

## Integration Flow

1. Interaction occurs (testing or production)
2. Interaction saved to `Interactions/` folder
3. Claudio calls Opus evaluator with interaction + agent context
4. Opus returns YAML judgment
5. Claudio parses judgment, appends observation to `Evaluation_Framework.md`
6. Claudio updates timestamp, increments hypothesis counters
7. Claudio checks status thresholds, flags PM if transition suggested
8. Periodically: Claudio runs `rollup_to_global` to update system view

---

## Implementation Order

1. Define Rust structs for Evaluation, Observation, Hypothesis
2. Implement `read_evaluation` — parse YAML front matter
3. Implement `add_observation` — append observation, set timestamp
4. Implement `update_hypothesis_counts` — increment counters
5. Implement `check_status_thresholds` — return suggested transitions
6. Create `Evaluation_Framework.md` template in each agent's folder
7. Create `Evaluator_Prompt.md` in Team_Files
8. Test end-to-end — manually save interaction, run evaluator, record result
9. Implement `rollup_to_global` for Global_Evaluation.md
10. Document external tester workflow

---

## Constraints

- Max 5 hypotheses per agent evaluation file
- Observations append-only (no deletion)
- YAML front matter is machine-owned; markdown body is human-owned
- Claudio preserves markdown body when updating YAML
- Timestamps always set by Rust, never manually

---

## Version History

**v0.1** — Initial specification from PM.
**v0.2** — PE additions: understanding failure types, task type calibration, hypothesis templates, status thresholds, radiation log, interaction storage architecture, external tester protocol.
