# Knowledge Graph Template

Template for agent domain knowledge. Hybrid vector + graph retrieval format.

---

## Schema

Each node is one chunk (~100-300 tokens). Self-contained with visible relationships.

### Metadata Block (YAML)

```yaml
node_id: unique-kebab-case-identifier
domain: top-level-domain
category: grouping-within-domain
distribution: internal | deployed | user-portable
related: [direct-relationships]
see_also: [tangential-references]
supersedes: [deprecated-node-if-any]
source: [external-citation-if-any]
```

### Distribution Values

| Value | Meaning | Example Use |
|-------|---------|-------------|
| `internal` | Project Team only, never ships | Routing logic, team coordination |
| `deployed` | Ships with agent | Domain expertise, procedures |
| `user-portable` | Persists with user across sessions | User profile, preferences |

### Content Block

```markdown
---
NODE: node-id

[Core content — what this node teaches. 2-5 sentences.]

[Design implication or action guidance, if applicable.]

RELATED: node-a, node-b
SEE-ALSO: node-c
```

---

## Structure

```
DOMAIN: [agent-domain]
├── CATEGORY: [grouping-1]
│   ├── NODE: [concept]
│   ├── NODE: [concept]
│   └── NODE: [concept]
├── CATEGORY: [grouping-2]
│   └── ...
├── CATEGORY: failure-modes
│   └── NODE: [documented failures]
├── CATEGORY: procedures
│   └── NODE: [standard procedures]
└── CATEGORY: external-sources
    └── NODE: [citations]
```

---

## Node Template

Copy and fill:

```yaml
node_id: [unique-identifier]
domain: [domain]
category: [category]
distribution: [internal | deployed | user-portable]
related: []
see_also: []
supersedes: null
source: null
---
NODE: [node-id]

[Content]

RELATED: [if any]
SEE-ALSO: [if any]
```

---

## Standard Categories

Include these in every graph:

| Category | Contains |
|----------|----------|
| [domain-specific] | Core domain knowledge — varies by agent |
| failure-modes | Documented failures: symptom, cause, fix |
| procedures | Standard procedures: trigger, steps |
| external-sources | Citations with URLs |

---

## Authoring Guidelines

### Chunk Size
- Target 100-300 tokens per node
- Self-contained: node should make sense without neighbors
- Include parent context (domain/category) in each chunk

### Distribution Assignment
- `internal`: Coordination, routing, infrastructure — never leaves project
- `deployed`: Domain expertise that ships with agent
- `user-portable`: User-specific data that persists across sessions

### Relationships
- RELATED: Direct conceptual links (traverse for context)
- SEE-ALSO: Tangential links (traverse for breadth)
- SUPERSEDES: Points to deprecated node this replaces

### Failure Modes
Document failures as they occur:

```yaml
node_id: [failure-name]
domain: [domain]
category: failure-modes
distribution: [typically deployed]
---
NODE: [failure-name]

Failure: [What went wrong]

Symptom: [How you notice it]

Cause: [Why it happens]

Fix: [How to prevent or resolve]

RELATED: [relevant-nodes]
```

### Procedures
Document repeatable processes:

```yaml
node_id: [procedure-name]
domain: [domain]
category: procedures
distribution: [typically deployed]
---
NODE: [procedure-name]

Trigger: [When to run this procedure]

Steps:
1. [Step one]
2. [Step two]
3. [Step three]

RELATED: [relevant-nodes]
```

### Live References
External sources that may update:

```yaml
node_id: [source-name]
domain: [domain]
category: external-sources
distribution: [typically deployed]
type: live-reference
url: [https://...]
---
NODE: [source-name]

[Description of source]

LIVE: [url]

[When to consult]

RELATED: [relevant-nodes]
```

---

## Navigation Protocol

### Query Patterns

| Intent | Approach |
|--------|----------|
| Find guidance | Natural language: "How do I [task]?" |
| Find failure | Natural language: "What causes [symptom]?" |
| Find procedure | "How do I [action]?" [filter: procedures] |
| Specific node | [node: node-id] |
| Category filter | [filter: category-name] |

### Traversal Rules

1. Semantic query → vector retrieval returns top-k nodes
2. Follow RELATED edges for direct context
3. Follow SEE-ALSO edges for tangential relevance
4. Check SUPERSEDES — use newer version if exists
5. Cite source node when applying knowledge

### Not Found Protocol

If graph lacks relevant knowledge:
1. Proceed with general reasoning
2. Flag as candidate lesson if gap seems significant
3. Note: "Graph did not contain [topic] — reasoning from first principles"

---

## Maintenance

### Adding Nodes
1. Identify gap or new pattern
2. Create node using template
3. Assign appropriate distribution
4. Add RELATED/SEE-ALSO edges to existing nodes
5. Update existing nodes to link back

### Deprecating Nodes
1. Create new node with corrected content
2. Add `supersedes: old-node-id` to new node
3. Keep old node (for reference) but mark deprecated
4. Update edges pointing to old node

### Synthesis Cycles
Run periodically to consolidate accumulated knowledge:
1. Review pending lessons / new nodes
2. Merge redundant nodes
3. Strengthen cross-references
4. Prune orphaned nodes

---

## Graph Types by Distribution

| Graph Type | Default Distribution | Access |
|------------|---------------------|--------|
| Project Graph | `internal` | Project Team only |
| Domain Graph (PE, PM, etc.) | `internal` | Project Team (development) |
| Domain Graph (Coaches, etc.) | `deployed` | Ships with agent |
| User Graph | `user-portable` | Per-user, per-agent instance |
