# PE Knowledge Graph

Domain expertise for prompt engineering. Hybrid vector + graph retrieval format.

**Distribution:** All nodes `internal`

---

## Schema

Each node is one chunk. Structure:

```yaml
node_id: unique-identifier
domain: prompt-engineering
category: category-name
distribution: internal
related: [direct relationships]
see_also: [tangential references]
supersedes: [deprecated node, if any]
source: [external citation, if any]
---
NODE: node-id
[content]
RELATED: node-a, node-b
SEE-ALSO: node-c
```

---

## DOMAIN: prompt-engineering

### CATEGORY: instruction-positioning

```yaml
node_id: primacy-effect
domain: prompt-engineering
category: instruction-positioning
distribution: internal
related: [recency-effect, middle-weakness]
see_also: [instruction-clarity]
supersedes: null
source: Murdock (1962), serial position effect
---
NODE: primacy-effect

Early instructions anchor interpretation of later content. What you establish first becomes the default frame.

Design implication: Open with role definition, core constraints, and non-negotiable rules.

RELATED: recency-effect, middle-weakness
```

---

```yaml
node_id: recency-effect
domain: prompt-engineering
category: instruction-positioning
distribution: internal
related: [primacy-effect, middle-weakness]
see_also: [instruction-clarity]
supersedes: null
source: Murdock (1962), serial position effect
---
NODE: recency-effect

Final instructions feel most immediately actionable. Closing with critical constraints increases compliance.

Design implication: End with response format, quality checks, or "remember to..." reminders.

RELATED: primacy-effect, middle-weakness
```

---

```yaml
node_id: middle-weakness
domain: prompt-engineering
category: instruction-positioning
distribution: internal
related: [primacy-effect, recency-effect]
see_also: [liu-2023-lost-middle]
supersedes: null
source: Liu et al. (2023), "Lost in the Middle"
---
NODE: middle-weakness

Content in the middle of long prompts is deprioritized under context pressure. Retrieval accuracy drops significantly for mid-positioned information.

Design implication: Place critical constraints at boundaries (beginning/end). Bury optional guidance, examples, and edge cases in the middle.

Analogy: Boundary conditions in a PDE — they constrain the solution space; the interior is solved given those constraints.

RELATED: primacy-effect, recency-effect
SEE-ALSO: liu-2023-lost-middle
```

---

### CATEGORY: model-optimization

```yaml
node_id: opus-characteristics
domain: prompt-engineering
category: model-optimization
distribution: internal
related: [sonnet-characteristics, haiku-characteristics, model-selection]
see_also: []
supersedes: null
source: null
---
NODE: opus-characteristics

Opus tolerates dense, nested instructions. Strong coherence across long conversations. Handles judgment under ambiguity well.

Instruction style: Can use complex conditional logic, nested structures, nuanced decision rules. Tolerates implicit context better than smaller models.

Best for: Multi-domain synthesis, complex multi-step reasoning, nuanced judgment calls.

RELATED: sonnet-characteristics, haiku-characteristics, model-selection
```

---

```yaml
node_id: sonnet-characteristics
domain: prompt-engineering
category: model-optimization
distribution: internal
related: [opus-characteristics, haiku-characteristics, model-selection]
see_also: []
supersedes: null
source: null
---
NODE: sonnet-characteristics

Sonnet needs balanced instruction density with clear section breaks. Build in reference mechanisms for long conversations.

Instruction style: Moderate complexity. Use explicit structure (headers, numbered steps). Avoid deeply nested conditionals.

Best for: Balanced workflows, most production use cases where cost/capability tradeoff matters.

RELATED: opus-characteristics, haiku-characteristics, model-selection
```

---

```yaml
node_id: haiku-characteristics
domain: prompt-engineering
category: model-optimization
distribution: internal
related: [opus-characteristics, sonnet-characteristics, model-selection]
see_also: []
supersedes: null
source: null
---
NODE: haiku-characteristics

Haiku requires sparse, direct instructions. One clear task per prompt. Keep context windows focused. Minimal ambiguity tolerance.

Instruction style: Short sentences. Explicit decision rules. No implicit context — state everything needed. Avoid conditional branching.

Best for: High-volume tasks, well-defined operations, speed over nuance.

RELATED: opus-characteristics, sonnet-characteristics, model-selection
```

---

```yaml
node_id: model-selection
domain: prompt-engineering
category: model-optimization
distribution: internal
related: [opus-characteristics, sonnet-characteristics, haiku-characteristics]
see_also: []
supersedes: null
source: null
---
NODE: model-selection

Choose model based on task requirements:

- Opus: Judgment under ambiguity, multi-domain synthesis, complex reasoning
- Sonnet: Default for most systems, balanced capability and efficiency
- Haiku: Speed over nuance, constrained tasks, high volume

When building on Opus for deployment on smaller models: Use Opus reasoning to *model* how the target model will interpret instructions. Simpler models need sparser, more direct prompts.

RELATED: opus-characteristics, sonnet-characteristics, haiku-characteristics
```

---

### CATEGORY: design-patterns

```yaml
node_id: modular-system-design
domain: prompt-engineering
category: design-patterns
distribution: internal
related: [separation-of-concerns, instruction-clarity]
see_also: []
supersedes: null
source: null
---
NODE: modular-system-design

Multi-step agent systems should be modular:

- Each step produces an artifact for the next
- Clear input/output boundaries
- Validation gates between steps
- Context-carrying mechanisms when needed

Design implication: Define what each step consumes and produces. Make handoffs explicit.

RELATED: separation-of-concerns, instruction-clarity
```

---

```yaml
node_id: instruction-clarity
domain: prompt-engineering
category: design-patterns
distribution: internal
related: [modular-system-design]
see_also: [primacy-effect, recency-effect]
supersedes: null
source: null
---
NODE: instruction-clarity

Clear instructions reduce agent drift:

- Use //inline comments// to guide without cluttering output
- Provide explicit decision rules and scoring methods
- Include examples when logic is complex
- Specify failure handling behavior

Avoid: Ambiguous decision rules, implicit context assumptions, unbounded output length.

RELATED: modular-system-design
SEE-ALSO: primacy-effect, recency-effect
```

---

```yaml
node_id: mutable-state-consolidation
domain: prompt-engineering
category: design-patterns
distribution: internal
related: [separation-of-concerns]
see_also: []
supersedes: null
source: Case study, Japanese Coach project (2025-01)
---
NODE: mutable-state-consolidation

Mutable state should have a single source of truth. Scatter references create sync failures.

Failure pattern: Learner proficiency referenced in three places (intro, challenge calibration, feedback). When updating skill level, only one reference was changed, creating contradictory instructions.

Design implication: Consolidate all references to changeable data (user level, preferences, session context) into one clearly marked section. Other sections reference that section, never duplicate the value.

RELATED: separation-of-concerns
```

---

### CATEGORY: failure-modes

```yaml
node_id: separation-of-concerns
domain: prompt-engineering
category: failure-modes
distribution: internal
related: [modular-system-design, mutable-state-consolidation]
see_also: []
supersedes: null
source: Case study, PE Framework v1.2
---
NODE: separation-of-concerns

Failure: Mixing agent behavior, formatting conventions, and domain knowledge in one file.

Symptom: Updates to one concern destabilize others. Version history becomes tangled. Hard to reuse components.

Fix: Distinct files for distinct concerns:
- Instructions.md — what the agent does
- Conventions.md — how we format things
- Domain_Knowledge.md — what the agent knows

RELATED: modular-system-design, mutable-state-consolidation
```

---

```yaml
node_id: chat-context-staleness
domain: prompt-engineering
category: failure-modes
distribution: internal
related: [source-precedence]
see_also: []
supersedes: null
source: Case study, PE Framework v1.5
---
NODE: chat-context-staleness

Failure: Agent trusts chat artifacts over project files.

Symptom: False inconsistency reports. Agent claims files are out of sync when they aren't. References outdated drafts from earlier in conversation.

Cause: Stale context accumulates over long conversations and can override accurate source data.

Fix: Explicit source precedence rules in agent instructions.

RELATED: source-precedence
```

---

```yaml
node_id: source-precedence
domain: prompt-engineering
category: failure-modes
distribution: internal
related: [chat-context-staleness]
see_also: []
supersedes: null
source: null
---
NODE: source-precedence

Rule for agents working with external files:

- Project files are canonical — always re-read before assessing
- Chat-generated content is draft — ephemeral until written to file
- Do not trust earlier chat context over current file contents

Implementation: Add Source Precedence section near top of agent instructions (leverage primacy effect).

RELATED: chat-context-staleness
```

---

### CATEGORY: procedures

```yaml
node_id: lesson-capture
domain: prompt-engineering
category: procedures
distribution: internal
related: [synthesis-protocol]
see_also: []
supersedes: null
source: null
---
NODE: lesson-capture

Trigger phrases: "flag this lesson", "add this to lessons", "that's a lesson", "note that pattern"

Steps:
1. Format as lesson card:
   - Observation: What happened (specific, concrete)
   - Principle: The generalized rule
   - Design Implication: How to apply when building prompts
   - Related: Existing node this extends or contradicts
2. Append to Domain_Knowledge.md under Pending Lessons (or directly to graph)
3. Confirm with designer before proceeding

RELATED: synthesis-protocol
```

---

```yaml
node_id: synthesis-protocol
domain: prompt-engineering
category: procedures
distribution: internal
related: [lesson-capture]
see_also: []
supersedes: null
source: null
---
NODE: synthesis-protocol

Trigger: 5+ lesson cards accumulated, or major project concludes, or patterns across cards become obvious.

Steps:
1. Read all pending cards
2. Group by theme — identify cards addressing the same issue
3. Compress related cards into ONE node (synthesize observations, distill principle, combine implications)
4. Integrate: update existing nodes or add new ones
5. Promote to higher category if principle appears in 3+ case studies
6. Delete processed cards from Pending section
7. Increment version (minor bump)

Quality check: After synthesis, no redundant nodes; clear cross-references; reads as coherent knowledge, not fragments.

RELATED: lesson-capture
```

---

```yaml
node_id: version-archive
domain: prompt-engineering
category: procedures
distribution: internal
related: []
see_also: []
supersedes: null
source: null
---
NODE: version-archive

Trigger: Major change to agent instructions.

Steps:
1. Copy current Instructions.md to Arch/[agent]_v[X.Y].md
2. Update Version_History.md with change summary
3. Increment version number in Instructions.md header

Version increments:
- Patch (v1.0 → v1.0.1): Bug fixes, typos, clarifications
- Minor (v1.0 → v1.1): New sections, refined logic
- Major (v1.x → v2.0): Structural overhaul
```

---

### CATEGORY: external-sources

```yaml
node_id: anthropic-best-practices
domain: prompt-engineering
category: external-sources
distribution: internal
type: live-reference
url: https://platform.claude.com/docs/en/build-with-claude/prompt-engineering/claude-4-best-practices
related: []
see_also: [instruction-clarity, model-selection]
supersedes: null
source: null
---
NODE: anthropic-best-practices

Official Anthropic prompting guidance for Claude 4 models.

LIVE: https://platform.claude.com/docs/en/build-with-claude/prompt-engineering/claude-4-best-practices

Covers: Instruction structure, clarity patterns, model-specific considerations.

When current guidance needed, consult live URL.

SEE-ALSO: instruction-clarity, model-selection
```

---

```yaml
node_id: liu-2023-lost-middle
domain: prompt-engineering
category: external-sources
distribution: internal
type: citation
url: https://arxiv.org/abs/2307.03172
related: [middle-weakness]
see_also: []
supersedes: null
source: null
---
NODE: liu-2023-lost-middle

Liu et al. (2023), "Lost in the Middle: How Language Models Use Long Contexts"

Finding: LLMs retrieve information less reliably from the middle of long contexts. Performance is highest for information at the beginning and end.

URL: https://arxiv.org/abs/2307.03172

RELATED: middle-weakness
```

---

```yaml
node_id: murdock-1962-serial-position
domain: prompt-engineering
category: external-sources
distribution: internal
type: citation
related: [primacy-effect, recency-effect]
see_also: []
supersedes: null
source: null
---
NODE: murdock-1962-serial-position

Murdock (1962), "The serial position effect of free recall"

Finding: Items at the beginning (primacy) and end (recency) of a sequence are better remembered than items in the middle. Cognitive basis for instruction positioning principles.

RELATED: primacy-effect, recency-effect
```

---

## Navigation Protocol

### Query Patterns

Natural language primary, structured filters optional:

| Intent | Example Query |
|--------|---------------|
| Design guidance | "How should I structure instructions for Haiku?" |
| Failure diagnosis | "What causes agents to reference outdated state?" |
| Procedure | "How do I flag a lesson?" [filter: procedures] |
| Specific node | [node: mutable-state-consolidation] |

### Traversal Rules

1. Start with semantic search for relevant nodes
2. Follow RELATED edges for direct context
3. Follow SEE-ALSO edges for tangential relevance
4. Check SUPERSEDES before applying — use newer version if exists
5. Cite source node when applying knowledge

### Not Found Protocol

If the graph lacks relevant knowledge:
1. Proceed with general reasoning
2. Flag as candidate lesson if the gap seems significant
3. Note: "Graph did not contain [topic] — reasoning from first principles"

---

## Pending Lessons

Lesson cards awaiting synthesis. Run Synthesis Protocol when 5+ cards accumulate.

<!-- Append new lesson cards below this line -->
