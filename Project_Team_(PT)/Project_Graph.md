# Project Graph

Internal coordination infrastructure for the Project Team. Never distributed with deployed agents.

**Distribution:** All nodes `internal`

---

## Schema

```yaml
node_id: unique-identifier
domain: project-team | deployment | shared-procedures | presentation
category: category-name
distribution: internal
related: []
see_also: []
supersedes: null
source: null
```

---

## DOMAIN: project-team

### CATEGORY: agent-roster

```yaml
node_id: agent-pm
domain: project-team
category: agent-roster
distribution: internal
related: [agent-pe, agent-pc, task-routing]
see_also: []
supersedes: null
source: null
---
NODE: agent-pm

**Project Manager (PM)**
- Model: Flexible (typically Sonnet)
- Role: Strategy, decisions, user sounding board
- Domain graph: pm-domain
- Capabilities: Prioritization, decision frameworks, stakeholder coordination
- Invocation: Strategic planning, priority conflicts, decision logging

RELATED: agent-pe, agent-pc, task-routing
```

---

```yaml
node_id: agent-pe
domain: project-team
category: agent-roster
distribution: internal
related: [agent-pm, agent-pc, task-routing]
see_also: []
supersedes: null
source: null
---
NODE: agent-pe

**Prompt Engineer (PE)**
- Model: Opus
- Role: Agent design, framework maintenance
- Domain graph: pe-domain
- Capabilities: Instruction design, failure analysis, model optimization
- Invocation: New agent design, prompt review, architecture decisions

RELATED: agent-pm, agent-pc, task-routing
```

---

```yaml
node_id: agent-pc
domain: project-team
category: agent-roster
distribution: internal
related: [agent-pm, agent-pe, task-routing]
see_also: []
supersedes: null
source: null
---
NODE: agent-pc

**Project Coordinator (PC)**
- Model: Haiku
- Role: Async queue processing, status tracking
- Domain graph: None (uses project graph only)
- Capabilities: Queue management, status reports, routine lookups
- Invocation: Background tasks, logging, convention checks

RELATED: agent-pm, agent-pe, task-routing
```

---

```yaml
node_id: agent-claude-code
domain: project-team
category: agent-roster
distribution: internal
related: [task-routing, deployment-architecture]
see_also: []
supersedes: null
source: null
---
NODE: agent-claude-code

**Claude Code (Claudio)**
- Model: Opus
- Role: Master implementation, file management, git
- Domain graph: deployment-domain
- Capabilities: Code generation, file operations, scaffolding, deployment
- Invocation: Implementation tasks, file structure changes, Rust development

RELATED: task-routing, deployment-architecture
```

---

```yaml
node_id: agent-sonnet-interface
domain: project-team
category: agent-roster
distribution: internal
related: [presentation-layer-protocol, task-routing]
see_also: []
supersedes: null
source: null
---
NODE: agent-sonnet-interface

**Sonnet Interface Layer**
- Model: Sonnet
- Role: Primary interlocutor, presentation layer, user relationship
- Graph access: Project graph (routing), all domain graphs (query), user graph (exclusive)
- Capabilities: Conversation management, graph queries, specialist routing, tone/diction
- Always active: Entry point for all user interactions

Sonnet does not appear as a "called" agent — it is the caller.

RELATED: presentation-layer-protocol, task-routing
```

---

### CATEGORY: task-routing

```yaml
node_id: task-routing-rules
domain: project-team
category: task-routing
distribution: internal
related: [cognitive-style-hierarchy, agent-roster]
see_also: []
supersedes: null
source: null
---
NODE: task-routing-rules

Route tasks by cognitive demand:

| Signal | Route To | Model |
|--------|----------|-------|
| "design", "architect", "plan" | PE | Opus |
| "implement", "scaffold", "deploy" | Claude Code | Opus |
| "decide", "prioritize", "strategy" | PM | Sonnet |
| "log", "track", "status" | PC | Haiku |
| Routine, low-ambiguity | Handle directly | Sonnet |

When ambiguous: Sonnet handles directly, escalates if complexity emerges.

RELATED: cognitive-style-hierarchy, agent-roster
```

---

```yaml
node_id: cognitive-style-hierarchy
domain: project-team
category: task-routing
distribution: internal
related: [task-routing-rules]
see_also: [opus-characteristics, sonnet-characteristics, haiku-characteristics]
supersedes: null
source: null
---
NODE: cognitive-style-hierarchy

Model selection by cognitive style:

| Model | Style | Task Types |
|-------|-------|------------|
| Opus | Discover & Create | Novel design, complex reasoning, ambiguity |
| Sonnet | Find & Explain | Pattern matching, moderate complexity, conversation |
| Haiku | Go Get & Replace | Routine lookup, simple substitution, queue processing |

Principle: Use the smallest model that can handle the task. Escalate when ambiguity or novelty exceeds model capacity.

SEE-ALSO: opus-characteristics, sonnet-characteristics, haiku-characteristics
```

---

```yaml
node_id: multi-prompt-routing
domain: project-team
category: task-routing
distribution: internal
related: [task-routing-rules, presentation-layer-protocol]
see_also: []
supersedes: null
source: null
---
NODE: multi-prompt-routing

One logical "agent" may split into multiple prompt sets called independently.

Example: Language Coach
- Conversation prompt (Sonnet) — primary interaction
- Grammar analysis prompt (Haiku) — parse errors
- Lesson planning prompt (Sonnet) — curriculum sequencing
- Feedback synthesis prompt (Opus) — complex pedagogical decisions

Interface layer routes to appropriate prompt set based on current task, not fixed agent identity.

RELATED: task-routing-rules, presentation-layer-protocol
```

---

### CATEGORY: team-workflow

```yaml
node_id: design-implementation-handoff
domain: project-team
category: team-workflow
distribution: internal
related: [agent-pe, agent-claude-code]
see_also: []
supersedes: null
source: null
---
NODE: design-implementation-handoff

PE → Claude Code handoff:

1. PE designs agent prompts
2. PE deposits artifacts in `PE/Output/`
3. Claude Code picks up designs
4. Claude Code scaffolds agent folders in `Specialists/`
5. Claude Code moves processed files to `PE/Archive/`
6. PM coordinates priorities if conflicts arise

Artifacts flow: Output/ → Specialists/ → Archive/

RELATED: agent-pe, agent-claude-code
```

---

```yaml
node_id: decision-logging-workflow
domain: project-team
category: team-workflow
distribution: internal
related: [agent-pm, agent-pc]
see_also: [lesson-capture]
supersedes: null
source: null
---
NODE: decision-logging-workflow

When decisions are made:

1. PM (or active agent) notes decision
2. Format: Decision, rationale, alternatives considered
3. Log to PM_Queue if async, or Decision_Log if immediate
4. PC processes queue during downtime
5. Significant decisions propagate to relevant graphs

Trigger phrases: "let's decide", "decision made", "we're going with"

RELATED: agent-pm, agent-pc
SEE-ALSO: lesson-capture
```

---

```yaml
node_id: sprint-structure
domain: project-team
category: team-workflow
distribution: internal
related: []
see_also: []
supersedes: null
source: Master_Plan.md
---
NODE: sprint-structure

Sprint cadence: ~3-4 days

Current sprint: 2025-01-21 → 2025-01-24

Sprint structure:
1. Critical path identified in Master_Plan.md
2. Tasks assigned by owner (Designer, PM, PE, Claude Code)
3. Status tracked via PM_Queue and PC
4. End-of-sprint: Update Master_Plan, archive completed items

RELATED: (none — top-level coordination node)
```

---

## DOMAIN: deployment

### CATEGORY: aws-architecture

```yaml
node_id: deployment-architecture
domain: deployment
category: aws-architecture
distribution: internal
related: [rust-agent-wrapper, api-endpoint-pattern]
see_also: []
supersedes: null
source: AWS_MIGRATION_GUIDE.md
---
NODE: deployment-architecture

Target AWS architecture:

- EC2 Linux server
- Rust-wrapped Claude agents
- Server-side API proxy (centralized API key)
- S3 for session storage
- Interface layer for routing

Traffic flow:
User → Website → API endpoint → Rust interface → Claude API → Response

RELATED: rust-agent-wrapper, api-endpoint-pattern
```

---

```yaml
node_id: api-endpoint-pattern
domain: deployment
category: aws-architecture
distribution: internal
related: [deployment-architecture, rust-agent-wrapper]
see_also: []
supersedes: null
source: null
---
NODE: api-endpoint-pattern

API endpoints for website ↔ AWS communication:

```
POST /agent/{agent_id}/message
  body: { user_id, message, session_id }
  returns: { response, session_id }

GET /agent/{agent_id}/session/{session_id}
  returns: { history, user_profile }

POST /agent/{agent_id}/feedback
  body: { session_id, rating, comments }
```

All endpoints proxy through Rust interface layer before reaching Claude API.

RELATED: deployment-architecture, rust-agent-wrapper
```

---

### CATEGORY: rust-functions

```yaml
node_id: rust-agent-wrapper
domain: deployment
category: rust-functions
distribution: internal
related: [deployment-architecture, agent-call-signature]
see_also: []
supersedes: null
source: null
---
NODE: rust-agent-wrapper

Rust wrapper for Claude agents:

Responsibilities:
- Manage API authentication
- Query vectorized graphs
- Compose context (conversation + subgraph)
- Route to appropriate model/prompt set
- Handle response streaming
- Log interactions for evaluation

Location: `Project_Team_(PT)/src/`

RELATED: deployment-architecture, agent-call-signature
```

---

```yaml
node_id: agent-call-signature
domain: deployment
category: rust-functions
distribution: internal
related: [rust-agent-wrapper, subgraph-handoff-protocol]
see_also: []
supersedes: null
source: null
---
NODE: agent-call-signature

Standard function signature for agent calls:

```rust
fn agent_call(
    model: Model,           // Opus | Sonnet | Haiku
    prompt_set: &str,       // e.g., "PE", "SpanishCoach", "NoteTaker"
    context: &str,          // Conversation excerpt
    subgraph: SubGraph,     // Pre-selected knowledge nodes
) -> AgentResponse
```

Variants:
- `planning(Model, prompt_set, context, subgraph)` — deep reasoning tasks
- `helper(prompt_set, context)` — always Haiku, minimal subgraph
- `query_graph(domain, query)` — returns relevant nodes

RELATED: rust-agent-wrapper, subgraph-handoff-protocol
```

---

```yaml
node_id: graph-query-function
domain: deployment
category: rust-functions
distribution: internal
related: [agent-call-signature, subgraph-handoff-protocol]
see_also: []
supersedes: null
source: null
---
NODE: graph-query-function

Graph query interface:

```rust
fn query_graph(
    domains: Vec<&str>,     // Which graphs to search
    query: &str,            // Natural language or keywords
    filters: Option<Filters>, // Category, distribution, etc.
    max_nodes: usize,       // Token budget control
) -> SubGraph
```

Returns nodes ranked by relevance. Caller (Sonnet) decides final selection.

Edge traversal: Query retrieves entry points; caller follows RELATED edges as needed.

RELATED: agent-call-signature, subgraph-handoff-protocol
```

---

## DOMAIN: presentation

### CATEGORY: interface-protocols

```yaml
node_id: presentation-layer-protocol
domain: presentation
category: interface-protocols
distribution: internal
related: [subgraph-handoff-protocol, user-profile-schema]
see_also: [agent-sonnet-interface]
supersedes: null
source: null
---
NODE: presentation-layer-protocol

Sonnet is the presentation layer:

1. Maintains user relationship (profile, history, tone preferences)
2. Queries graphs for relevant knowledge
3. Routes complex tasks to specialists with curated context
4. Receives structured output from specialists
5. Translates to user-appropriate phrasing
6. Maintains conversation continuity

Specialists return structured artifacts, not user-facing prose. Sonnet owns final diction.

User profile graph is Sonnet-only — specialists work task-blind to user context.

RELATED: subgraph-handoff-protocol, user-profile-schema
SEE-ALSO: agent-sonnet-interface
```

---

```yaml
node_id: subgraph-handoff-protocol
domain: presentation
category: interface-protocols
distribution: internal
related: [presentation-layer-protocol, agent-call-signature]
see_also: []
supersedes: null
source: null
---
NODE: subgraph-handoff-protocol

When Sonnet calls a specialist:

1. Select relevant nodes via vector query + 1-hop edge traversal
2. Check token budget: conversation_tokens + subgraph_tokens < model_limit
3. If over budget: summarize older conversation, prune distant nodes
4. Serialize as: { summary: "...", nodes: [...] }
5. Call: agent_call(model, prompt_set, context, subgraph)

Callee trusts caller's selection — no re-query.

Serialization format:
```yaml
subgraph:
  summary: "Context on [topic] for [task]"
  nodes:
    - node_id: ...
      content: ...
    - node_id: ...
      content: ...
```

RELATED: presentation-layer-protocol, agent-call-signature
```

---

```yaml
node_id: specialist-output-format
domain: presentation
category: interface-protocols
distribution: internal
related: [presentation-layer-protocol]
see_also: []
supersedes: null
source: null
---
NODE: specialist-output-format

Specialists return structured content, not conversational prose:

```yaml
response_type: design_artifact | analysis | decision | note
content:
  artifact: |
    [The actual deliverable]
  notes:
    - [Observation 1]
    - [Observation 2]
  confidence: high | medium | low
  follow_up_needed: true | false
  follow_up_question: "..." # if needed
```

Sonnet interprets this structure and phrases for user.

RELATED: presentation-layer-protocol
```

---

```yaml
node_id: user-profile-schema
domain: presentation
category: interface-protocols
distribution: internal
related: [presentation-layer-protocol]
see_also: []
supersedes: null
source: null
---
NODE: user-profile-schema

User profile structure (Sonnet-only access):

```yaml
node_id: user-profile-{user_id}
domain: user-context
category: preferences
distribution: user-portable
---
communication:
  technical_depth: low | medium | high
  formality: casual | professional | academic
  verbosity: concise | moderate | detailed
  style_notes: "..." # e.g., "prefers E.B. White clarity"

working_patterns:
  decision_style: "..." # e.g., "draft and refine"
  signals: [...] # e.g., "good for now = proceed"

current_context:
  active_project: "..."
  recent_topics: [...]
  session_start: timestamp

history:
  sessions: [...]
  key_decisions: [...]
```

RELATED: presentation-layer-protocol
```

---

## DOMAIN: shared-procedures

### CATEGORY: documentation

```yaml
node_id: lesson-capture
domain: shared-procedures
category: documentation
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
   - Design Implication: How to apply when building
   - Related: Existing node this extends or contradicts
2. Append to relevant domain graph under Pending Lessons
3. Confirm with designer before proceeding

Lesson cards await synthesis into permanent nodes.

RELATED: synthesis-protocol
```

---

```yaml
node_id: synthesis-protocol
domain: shared-procedures
category: documentation
distribution: internal
related: [lesson-capture]
see_also: []
supersedes: null
source: null
---
NODE: synthesis-protocol

Trigger: 5+ lesson cards accumulated, or major project concludes.

Steps:
1. Read all pending cards in target graph
2. Group by theme
3. Compress related cards into ONE node (synthesize, distill, combine)
4. Integrate: update existing nodes or add new ones
5. Promote to higher category if principle appears in 3+ case studies
6. Delete processed cards
7. Increment graph version

Quality check: No redundant nodes; clear cross-references; coherent knowledge.

RELATED: lesson-capture
```

---

```yaml
node_id: version-archive
domain: shared-procedures
category: documentation
distribution: internal
related: []
see_also: []
supersedes: null
source: null
---
NODE: version-archive

Trigger: Major change to agent instructions or graph structure.

Steps:
1. Copy current file to `Arch/[name]_v[X.Y].md`
2. Update Version_History.md with change summary
3. Increment version in file header

Version increments:
- Patch (v1.0 → v1.0.1): Fixes, typos
- Minor (v1.0 → v1.1): New content, refined logic
- Major (v1.x → v2.0): Structural overhaul
```

---

```yaml
node_id: session-logging
domain: shared-procedures
category: documentation
distribution: internal
related: []
see_also: []
supersedes: null
source: null
---
NODE: session-logging

At session end:

1. Read Conversation_Log_Template.md
2. Create summary appropriate to agent:
   - Opus agents: Fuller detail on rationale, trade-offs, decisions
   - Sonnet agents: Key outcomes, next steps
   - Haiku agents: Task list, completion status
3. Save to History/ directory

Format includes: Date, participants, topics, decisions, artifacts produced, open questions.
```

---

## Navigation Protocol

Same as domain graphs — see Knowledge_Graph_Template.md.

Internal-only access: This graph is queried by Sonnet interface layer and Project Team agents. Never exposed to deployed specialists or end users.