# Prompt Engineering Best Practices

Actionable guidance for building Claude agent systems. Consult during design and review.

---

## General Best Practices

### Instruction Positioning

Claude processes prompts sequentially; position affects weight. This parallels the *serial position effect* in human cognition, though the mechanism differs.

**Primacy effect:** Early instructions anchor interpretation of later content. What you establish first becomes the default frame.

**Recency effect:** Final instructions feel most immediately actionable. Closing with your most critical constraint increases compliance.

**Middle position weakness:** Content in the middle of long prompts is most likely to be deprioritized under context pressure. (Liu et al., 2023)

**Design implication:** Place critical constraints at the beginning (to anchor) and end (to activate). Bury optional guidance in the middle.

*Analogy:* Boundary conditions in a PDE—they constrain the solution space; the interior is solved given those constraints.

### Instruction Clarity

- Use *//inline comments//* to guide agents without cluttering output
- Provide explicit decision rules and scoring methods
- Include examples when logic is complex
- Specify failure handling behavior

### Modular System Design

- Each step produces an artifact for the next
- Clear input/output boundaries
- Validation gates between steps
- Context-carrying mechanisms when needed

### Claude Model Optimization

**Leverage:**
- Multi-turn conversation management across long interactions
- Complex instruction following with clear structure
- Pattern recognition across disparate information
- Natural language generation and authentic voice
- Artifact creation and incremental iteration
- Code generation and analysis capabilities

**Avoid:**
- Ambiguous decision rules (provide explicit logic/scoring)
- Over-reliance on implicit context (make dependencies explicit)
- Defaulting to generic/corporate language (specify tone explicitly)
- Unbounded artifact length (set clear limits)

---

## Domain-Specific Practices

### Conversational Agents

- Natural dialogue flow with validation gates
- User trust-building through active listening signals
- Tone specification (professional/casual/authoritative)
- One question at a time to avoid overwhelming

### Analysis Pipelines

- Clear input format specifications (CSV, JSON, unstructured text)
- Explicit handling of edge cases (missing data, malformed input)
- Structured output with validation criteria
- Intermediate artifact visibility for debugging

### Coding Tools

- Language and framework constraints upfront
- Error handling and fallback behaviors
- Code style and convention specifications
- Testing/validation steps built into workflow
- Clear separation between generation and execution

### Model-Specific Adjustments

| Consideration | Opus 4.5 | Sonnet 4.5 | Haiku 4.5 |
|---------------|----------|------------|-----------|
| **Parallel complexity** | Multiple dimensions simultaneously | Sequential steps | One task per prompt |
| **Context management** | Strong long-thread coherence | Build in reference mechanisms | Keep context focused |
| **Instruction density** | Dense, nested OK | Moderate; clear section breaks | Sparse, direct |
| **Best for** | Complex reasoning, nuanced judgment | Balanced workflows | High-volume, constrained tasks |

**Choosing Your Target:**
- **Opus** — judgment under ambiguity, multi-domain synthesis
- **Sonnet** — default for most systems (balance of capability and efficiency)
- **Haiku** — speed over nuance, highly constrained tasks

---

## Case Studies

Observed patterns from testing. Each documents a specific failure or success and extracts a generalizable principle.

### Separation of Concerns

**Observation:** PE Framework v1.2 mixed agent behavior instructions with formatting conventions and domain knowledge in a single file. During testing, updates to one concern risked destabilizing others.

**Principle:** Separate *what the agent does* (behavior) from *how we format things* (conventions) from *what the agent knows* (best practices).

**Design Implication:** Use distinct files for distinct concerns. The agent's core prompt should define behavior and point to external files for knowledge and standards.

### Chat Context vs. Project Files

**Observation:** During framework review, the agent referenced draft artifacts from earlier in the chat instead of re-reading current project files. This produced false inconsistency reports—the agent claimed files were out of sync when they weren't.

**Principle:** Chat-generated content is ephemeral; project files are canonical. Stale context accumulates over long conversations and can override accurate source data.

**Design Implication:** When agents work with versioned external documents, include explicit source precedence rules. Instruct agents to re-read project files before evaluating current state rather than trusting chat context.

---

## External Sources

- **Anthropic Claude 4 Best Practices** — Official prompting guidance. [https://platform.claude.com/docs/en/build-with-claude/prompt-engineering/claude-4-best-practices](https://platform.claude.com/docs/en/build-with-claude/prompt-engineering/claude-4-best-practices)

- **Liu et al. (2023), "Lost in the Middle: How Language Models Use Long Contexts"** — LLMs retrieve information less reliably from the middle of long contexts. [https://arxiv.org/abs/2307.03172](https://arxiv.org/abs/2307.03172)

- **Murdock (1962), "The serial position effect of free recall"** — Cognitive basis for primacy/recency effects in sequential processing.

---

## Pending Lessons

Lesson cards awaiting synthesis. Run Synthesis Protocol when 5+ cards accumulate. Use the template below


---
### LESSON CARD: [Short Title]
**Source:** [Project/session where observed]
**Date:** [YYYY-MM-DD]

**Observation:** [What happened — specific, concrete]

**Principle:** [The generalized rule]

**Design Implication:** [How to apply when building prompts]

**Related:** [Optional — existing case study this extends or contradicts]
---

---
### LESSON CARD: Mutable State Consolidation
**Source:** Japanese Coach project
**Date:** 2025-01-15

**Observation:** Learner proficiency was referenced in three places (intro, challenge calibration, feedback). When updating skill level, only one reference was changed, creating contradictory instructions.

**Principle:** Mutable state should have a single source of truth. Scatter references create sync failures.

**Design Implication:** Consolidate all references to changeable data (user level, preferences, context) into one clearly marked section. Other sections reference that section, never duplicate the value.

**Related:** Extends "Separation of Concerns"
---
<!-- Append new lesson cards below this line -->