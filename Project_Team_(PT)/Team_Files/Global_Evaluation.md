---
# MACHINE-OWNED: Claudio reads/writes this block
last_updated: 2025-01-19
agents:
  - name: Spanish_Coach_CR
    version: v1.0
    status: untested
    observation_count: 0
    hypothesis_summary: "0/4 supported"
  - name: Japanese_Coach_Kyoto
    version: v1.0
    status: untested
    observation_count: 0
    hypothesis_summary: "0/4 supported"
  - name: French_Coach_Quebec
    version: v1.0
    status: untested
    observation_count: 0
    hypothesis_summary: "0/4 supported"
---

# Global Evaluation

System-level view across all agents.

---

## Agent Status

| Agent | Version | Status | Observations | Hypotheses |
|-------|---------|--------|--------------|------------|
| Spanish_Coach_CR | v1.0 | untested | 0 | 0/4 supported |
| Japanese_Coach_Kyoto | v1.0 | untested | 0 | 0/4 supported |
| French_Coach_Quebec | v1.0 | untested | 0 | 0/4 supported |

---

## System-Level Hypotheses

Max 5 hypotheses about team/system performance.

### SH1: Coach Stem Effectiveness

**Statement:** P(warmth behaviors present | Coach Stem v1.2 used) > 0.85

**Rationale:** Coach Stem should reliably produce trust-building behaviors across all coaching agents.

**Status:** Untested

### SH2: Interview Function Sufficiency

**Statement:** P(sufficient context gathered | interview completed) > 0.85

**Rationale:** Interview function should produce usable input for downstream work.

**Status:** Untested

---

## PM Synthesis

Cross-agent patterns, token economy observations, goal velocity notes.

[No observations yet]

---

## Rust Functions

| Function | Purpose |
|----------|---------|
| `rollup_to_global` | Aggregate agent evaluation data into this file |

---

## Version History

**v0.1** — Initial template.
