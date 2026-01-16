# Project Manager Domain Knowledge

Facts, patterns, and learnings for project management.

---

## Team Dynamics

### Agent Capabilities

| Agent | Strength | Limitation |
|-------|----------|------------|
| Claude Code | Fast implementation, file access | No strategic judgment |
| PE | Deep prompt design, framework thinking | Slow, expensive tokens |
| PC | Cheap, reliable queue processing | No decision-making |
| PM | Strategic thinking, user alignment | No direct implementation |

### Handoff Patterns

- **Ambiguous goal** → PM clarifies → PE designs → Claude Code implements
- **Quick fix** → Claude Code directly
- **New agent** → PM scopes → PE designs → Claude Code scaffolds
- **Status check** → PC processes queue

---

## Decision Patterns

### Good Decisions
- Clear scope and success criteria
- Trade-offs explicitly stated
- Reversible when possible
- Documented for future reference

### Decision Anti-Patterns
- Deciding without user input on major changes
- Over-engineering simple problems
- Premature optimization
- Scope creep disguised as "improvement"

---

## Project Health Signals

### Healthy
- Clear next steps
- Work flowing to right agents
- User feels heard
- Decisions documented

### Unhealthy
- Spinning without progress
- Work stuck in one agent
- User frustrated or confused
- Same issues recurring

---

## Token Economy

### Expensive Operations
- Large file rewrites
- Extensive codebase exploration
- Multiple agent coordination in one session

### Cheap Operations
- Queue writes for PC
- Direct Claude Code file ops
- Focused, scoped tasks

### Rule of Thumb
> If a human could do it faster with keyboard shortcuts, flag it as heavy op.

---

## Lessons Learned

| Date | Learning | Source |
|------|----------|--------|
| 2025-01-15 | Modular hooks reduce instruction bloat | PE Framework v2.0 |
| 2025-01-15 | Queue location should be team-level, not agent-level | PC restructure |
