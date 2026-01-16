# Agent Build Plan (v0.3)

*Master planning document for agent development sprint.*

---

## P1: Project Goal

**Success:** A coordinated suite of agents with clear mandates, clean interfaces, and documented evaluation criteria. Each agent can be deployed independently but composes predictably when orchestrated.

**Failure conditions:**
- Agents with overlapping or ambiguous scope
- Interfaces that require translation or manual bridging
- No way to evaluate whether an agent is performing well
- Docs read unnecessarily—at startup or mid-task

**Governing principle:** Build functions separately before integration. Test in isolation, then compose.

---

## Current Status

### Functions (Shared Infrastructure)

| Function | Version | Status | File |
|----------|---------|--------|------|
| Interview Function | v0.2 | ✅ Complete | `functions/Interview_Function_v0.2.md` |
| Lazy File Loading | v0.1 | ✅ Complete | `functions/Lazy_File_Loading_v0.1.md` |
| Language Coach Interview Extensions | v1.0 | ✅ Complete | `functions/Language_Coach_Interview_Extensions_v1.0.md` |
| PM Dual-Mode | — | 🔲 Not started | — |

### Team Agents

| Agent | Version | Status | File |
|-------|---------|--------|------|
| PM Kernel | v0.1 | ✅ Complete | `agents/team/PM_Kernel_v0.1.md` |
| PM Helper | — | 🔲 Not started | — |
| Prompt Engineer | v2.0-alpha | ✅ Active (in use) | PE Framework |

### Coaches

| Agent | Version | Status | File | Priority |
|-------|---------|--------|------|----------|
| Coach Stem | v1.2 | ✅ Complete | `agents/coaches/Coach_Stem_v1.2.md` | Base |
| Spanish (Costa Rican) | v1.0 | ✅ Complete | `agents/coaches/Spanish_Coach_CR_v1.0.md` | P0 |
| Japanese (Kyoto-ben) | v1.0 | ✅ Complete | `agents/coaches/Japanese_Coach_Kyoto_v1.0.md` | High |
| French (Québécois) | v1.0 | ✅ Complete | `agents/coaches/French_Coach_Quebec_v1.0.md` | High |
| Weightlifting Programmer | v0.1-draft | ⚠️ Draft | `agents/coaches/Weightlifting_Programmer_v0.1-draft.md` | Medium |

### Assistants

| Agent | Version | Status | File |
|-------|---------|--------|------|
| Personal Assistant | v0.1-draft | ⚠️ Draft | `agents/assistants/Personal_Assistant_v0.1-draft.md` |

### SMEs (Not Started)

| Agent | Status | Notes |
|-------|--------|-------|
| SME Researcher | 🔲 Scoping | Foundational for other SMEs |
| Scientific Lit Researcher | 🔲 Scoping | Depends on SME Researcher patterns |
| Data Analyst | 🔲 Scoping | Needs input/output spec |

### Helpers (Not Started)

| Agent | Status | Notes |
|-------|--------|-------|
| PM Helper | 🔲 Not started | Needs queue protocol definition |
| Reading Notes Helper | 🔲 Not started | Low priority |

---

## Build Dependencies

```
Interview Function (v0.2)
├── Language Coach Interview Extensions (v1.0)
│   ├── Spanish Coach CR ✅
│   ├── Japanese Coach Kyoto ✅
│   └── French Coach Quebec ✅
├── Personal Assistant (uses base interview)
└── SME Researchers (will need extensions)

Coach Stem (v1.2)
├── Spanish Coach CR ✅
├── Japanese Coach Kyoto ✅
├── French Coach Quebec ✅
└── Weightlifting Programmer ⚠️ (different domain, may fork)

Lazy File Loading (v0.1)
└── All agents (integration pending)

PM Kernel (v0.1)
└── PM Helper (processes PM's queue)
```

---

## Next Actions

### Immediate (P0)

- [ ] **Deploy Spanish Coach CR** for Sunday testing
- [ ] Create evaluation cases document for Spanish Coach
- [ ] Test Spanish Coach with real user interaction

### Short-term (This Week)

- [ ] Test Japanese Coach Kyoto-ben
- [ ] Test French Coach Québécois  
- [ ] Refine based on testing feedback
- [ ] Draft PM Helper instructions

### Medium-term

- [ ] Integrate Lazy File Loading across agents
- [ ] Test trigger-phrase vs. inline architecture
- [ ] Scope SME Researcher
- [ ] Refine Weightlifting Programmer from draft
- [ ] Refine Personal Assistant from draft

---

## Open Questions

| Question | Impact | Status |
|----------|--------|--------|
| Interview function: Instructions.md or Conventions.md? | All agents | Testing needed |
| Trigger phrase vs. inline architecture | All agents | Experiment pending |
| Evaluation framework ownership (PE vs. Claude Code) | All agents | PM decision needed |
| Personal Assistant calendar integration | PA only | Needs spec |

---

## Decisions Log

| Decision | Rationale | Date |
|----------|-----------|------|
| Functions built separately before integration | Allows isolated testing, prevents coupling failures | — |
| PM model designation flexible (Opus/Sonnet) | Different modes need different depth | — |
| Language coaches trimmed to 3 (ES-CR, JP-Kyoto, FR-QC) | Focus quality over quantity | — |
| Weightlifter and PA drafted quickly | Token economy, refine later | — |

---

## Version History

**v0.1** — Initial plan structure
**v0.2** — Integrated PM queue tasks, added function development queue
**v0.3** — Updated with completed artifacts, current status tracking
