# Domain Knowledge Retrieval Rubric (v0.2)

*Evaluates whether the system successfully delivers useful knowledge to agents via vectorized graph queries and subgraph handoff.*

---

## Architecture Context

Knowledge retrieval now operates in a three-layer model:

```
┌─────────────────────────────────────────────────────────┐
│                    PRESENTATION LAYER                    │
│                      (Sonnet)                            │
│  • Queries vectorized graphs                            │
│  • Selects relevant subgraph                            │
│  • Passes context to specialists                        │
└─────────────────────────────────────────────────────────┘
                          ↓
                   subgraph handoff
                          ↓
┌─────────────────────────────────────────────────────────┐
│                    REASONING LAYER                       │
│                (Opus specialists, Haiku helpers)         │
│  • Receives pre-filtered subgraph                       │
│  • Trusts caller's selection                            │
│  • Applies knowledge to task                            │
└─────────────────────────────────────────────────────────┘
                          ↑
                   vector query
                          ↑
┌─────────────────────────────────────────────────────────┐
│                    KNOWLEDGE LAYER                       │
│              (Vectorized graphs + Rust functions)        │
│  • Project graph (internal)                             │
│  • Domain graphs (per-specialist)                       │
│  • User graph (per-user, Sonnet-only)                   │
└─────────────────────────────────────────────────────────┘
```

**Key shift:** Agents don't query knowledge themselves. Sonnet queries, selects, and hands off. This rubric evaluates the full chain.

---

## What We're Evaluating

The rubric assesses three handoff points:

| Point | Actor | Question |
|-------|-------|----------|
| **Query** | Sonnet / Rust | Did we query the right graph with the right terms? |
| **Selection** | Sonnet | Did we select the right nodes from results? |
| **Application** | Specialist | Did the specialist apply the knowledge correctly? |

Failures at different points require different fixes:
- Query failures → Rust function or Sonnet routing logic
- Selection failures → Subgraph assembly protocol
- Application failures → Specialist prompt or node content

---

## Three-Level Assessment

### Level 3: Effective Retrieval

**Definition:** Query returned relevant nodes, selection assembled an appropriate subgraph, and specialist applied the knowledge correctly.

**Indicators:**
- Query matched task semantics to graph content
- Selected nodes were sufficient and not over-broad
- Edge traversal (RELATED, SEE-ALSO) added useful context without bloat
- Token budget respected
- Specialist output demonstrates correct application

**Evidence pattern:**
> Sonnet: "This requires design pattern guidance. Querying PE domain graph for [instruction positioning]..."
> [Returns: primacy-effect, middle-position-weakness, instruction-clarity]
> "Passing subgraph to PE specialist with conversation context."
> Specialist: "Based on the primacy effect principle, the main constraint should move to the opening..."

**Diagnostic:** Full chain working. Query → Selection → Application all succeeded.

---

### Level 2: Partial Retrieval

**Definition:** One or more handoff points degraded, but system still produced useful output.

**Subtypes:**

| Subtype | Failure Point | Description | Fix Target |
|---------|---------------|-------------|------------|
| **Query miss** | Query | Retrieved tangential nodes, missed most relevant | Query terms, graph indexing |
| **Over-selection** | Selection | Passed too many nodes, wasted tokens | Selection criteria |
| **Under-selection** | Selection | Missed relevant nodes that were retrieved | Edge traversal logic |
| **Misapplication** | Application | Specialist had right nodes, used them wrong | Specialist prompt |
| **Late query** | Query | Queried after initial attempt failed | Routing triggers |

**Evidence pattern (over-selection):**
> Sonnet: "Querying for model selection guidance..."
> [Returns 12 nodes including unrelated failure modes]
> "Passing full result set to specialist."
> Specialist: [Wades through irrelevant content, finds answer]

**Diagnostic:** Query worked, selection too broad. Tighten selection criteria or token budget.

---

### Level 1: Failed Retrieval

**Definition:** Knowledge chain failed to deliver useful information to the specialist.

**Subtypes:**

| Subtype | Failure Point | Description | Fix Target |
|---------|---------------|-------------|------------|
| **No query** | Query | Task required knowledge, no query issued | Routing rules |
| **Empty result** | Query | Query returned no relevant nodes | Graph coverage, query terms |
| **Wrong graph** | Query | Queried wrong domain | Graph routing logic |
| **Selection dropped** | Selection | Retrieved nodes not passed to specialist | Handoff protocol |
| **Knowledge ignored** | Application | Specialist received subgraph, didn't use it | Specialist prompt |
| **Hallucinated knowledge** | Application | Specialist cited nodes that weren't in subgraph | Grounding failure |

**Evidence pattern (no query):**
> User: "Design an analysis pipeline agent."
> Sonnet: [Routes directly to specialist without querying domain graph]
> Specialist: [Produces generic output, misses documented patterns]

**Diagnostic:** Routing rules didn't trigger query for "design" tasks. Update task-routing-rules node.

---

## Evaluation Protocol

### Step 1: Establish Ground Truth

Before evaluating:

1. **Did this task require domain knowledge?**
   - If no → Retrieval N/A
   - If yes → Which graph(s)? Which nodes would help?

2. **Map the ideal chain:**
   - Query: What terms should match?
   - Selection: Which nodes (with edge traversal)?
   - Application: How should knowledge appear in output?

### Step 2: Observe Each Handoff

Document:

| Handoff | Observation |
|---------|-------------|
| **Query** | Issued? Terms? Graph? Results? |
| **Selection** | Nodes selected? Edges traversed? Token budget? |
| **Application** | Knowledge visible in output? Correctly applied? |

### Step 3: Assign Level

| Query | Selection | Application | Level |
|-------|-----------|-------------|-------|
| Correct | Appropriate | Correct | **3** |
| Correct | Appropriate | Incorrect | **2** (misapplication) |
| Correct | Over-broad | Any | **2** (over-selection) |
| Correct | Too narrow | Any | **2** (under-selection) |
| Partial | Any | Any | **2** (query miss) |
| None | — | — | **1** (no query) |
| Correct | Dropped | — | **1** (selection dropped) |
| Any | Any | Ignored | **1** (knowledge ignored) |

### Step 4: Diagnose Fix Target

| Level | Subtype | Fix Target |
|-------|---------|------------|
| 2 | Query miss | Rust query function, graph indexing |
| 2 | Over-selection | Sonnet selection criteria, token budget |
| 2 | Under-selection | Edge traversal rules |
| 2 | Misapplication | Specialist prompt |
| 1 | No query | Task routing rules |
| 1 | Empty result | Graph coverage |
| 1 | Wrong graph | Graph routing logic |
| 1 | Selection dropped | Handoff protocol |
| 1 | Knowledge ignored | Specialist prompt (grounding) |
| 1 | Hallucinated | Specialist prompt (citation rules) |

---

## Integration with Evaluation_Framework.md

### Hypothesis Templates

```yaml
# Query effectiveness
- id: H-query
  condition: "task requires domain knowledge"
  outcome: "relevant nodes retrieved"
  statement: "P(relevant nodes retrieved | domain knowledge required) > 0.9"

# Selection effectiveness  
- id: H-selection
  condition: "relevant nodes retrieved"
  outcome: "appropriate subgraph assembled"
  statement: "P(appropriate subgraph | relevant nodes retrieved) > 0.85"

# Application effectiveness
- id: H-application
  condition: "appropriate subgraph provided"
  outcome: "knowledge correctly applied"
  statement: "P(correct application | appropriate subgraph) > 0.9"

# Full chain
- id: H-chain
  condition: "task requires domain knowledge"
  outcome: "Level 3 retrieval"
  statement: "P(Level 3 | domain knowledge required) > 0.8"
```

### Observation Schema

```yaml
evaluator_judgment:
  understanding: good
  output_quality: good
  rationale: "..."
  knowledge_retrieval:
    required: true
    level: 3  # 1, 2, or 3
    subtype: null  # If Level < 3
    chain_details:
      query_issued: true
      query_terms: ["instruction positioning", "primacy"]
      graph_queried: "pe-domain"
      nodes_retrieved: ["primacy-effect", "middle-position-weakness", "instruction-clarity"]
      nodes_selected: ["primacy-effect", "instruction-clarity"]
      edges_traversed: ["RELATED"]
      application_visible: true
      application_correct: true
    fix_target: null  # If Level < 3
```

---

## Calibration Examples

### Example A: Level 3 (Full Chain Success)

**Task:** "Review this prompt for instruction positioning issues."

**Chain:**
- Query: Sonnet queries PE domain for "instruction positioning"
- Selection: Returns primacy-effect, middle-position-weakness; selects both + instruction-clarity via RELATED
- Application: Specialist cites primacy effect, identifies constraint buried in middle, recommends move

**Assessment:** All three handoffs succeeded. **Level 3.**

---

### Example B: Level 2 (Over-Selection)

**Task:** "What model should we use for this agent?"

**Chain:**
- Query: Sonnet queries PE domain for "model selection"
- Selection: Returns model-selection, opus-characteristics, sonnet-characteristics, haiku-characteristics + 8 tangentially related nodes
- Application: Specialist wades through, gives correct answer

**Assessment:** Query good, selection too broad (12 nodes when 4 sufficed), application succeeded despite noise. **Level 2 (over-selection).** Fix: Tighten selection criteria or lower max_nodes parameter.

---

### Example C: Level 1 (No Query)

**Task:** "Design an analysis pipeline agent."

**Chain:**
- Query: Sonnet routes directly to specialist, no query
- Selection: N/A
- Application: Specialist produces generic output, misses analysis-pipelines node

**Assessment:** Routing rules didn't trigger query. **Level 1 (no query).** Fix: Add "design" to task-routing-rules query triggers.

---

### Example D: Level 1 (Hallucinated Knowledge)

**Task:** "Explain the 'cascade principle' for prompt design."

**Chain:**
- Query: Sonnet queries, no node named "cascade principle" exists
- Selection: Returns empty or tangential
- Application: Specialist invents plausible-sounding "cascade principle" not in any graph

**Assessment:** Query returned nothing relevant; specialist fabricated rather than acknowledging gap. **Level 1 (hallucinated).** Fix: Strengthen "Not Found Protocol" in specialist prompt.

---

## Aggregation Thresholds

Track retrieval levels across observations:

| Level | Target | Warning | Critical |
|-------|--------|---------|----------|
| 3 | >80% | <75% | <60% |
| 2 | <15% | >20% | >30% |
| 1 | <5% | >10% | >15% |

**If Level 1 exceeds 10%:** Review routing rules, graph coverage, handoff protocol.

**If Level 2 exceeds 20%:** Review selection criteria, token budgets, specialist prompts.

---

## Version History

**v0.1** — Initial rubric for file-based lazy loading (obsolete).
**v0.2** — Rewritten for vectorized graph + subgraph handoff architecture. Three handoff points: Query → Selection → Application.
