# Master Plan

Strategic overview for the Claude Agents project.

---

## Project Team

| Agent | Model | Role |
|-------|-------|------|
| Claude Code | Opus | Master implementation, file management, git |
| Project Manager (PM) | Flexible | Strategy, decisions, user sounding board |
| Prompt Engineer (PE) | Opus | Agent design, framework maintenance |
| Project Coordinator (PC) | Haiku | Async queue processing, status tracking |

---

## Workflow

```
User ←→ PM (strategy) ←→ PE (design) → Output/
                                           ↓
                                    Claude Code (implement)
                                           ↓
                                    Specialists/ → GitHub

PC processes queue async ←── All agents write tasks
```

### Design → Implementation
1. PE designs agent prompts, deposits in `PE/Output/`
2. Claude Code picks up designs, scaffolds agent folders in `Specialists/`
3. Claude Code moves processed Output files to `PE/Archive/`
4. PM coordinates priorities and decisions
5. PC tracks status via queue

### Development Principle
**Build functions separately before integration.** Design and test new capabilities as standalone functions first, then integrate into agents once proven. This allows adaptation to other agents later.

---

## Agent Template

Standard files for every agent in `Agent Files/`:

| File | Purpose |
|------|---------|
| Instructions.md | Core identity and behavior |
| Domain_Knowledge.md | Facts and patterns |
| Conventions.md | Procedures and formats |
| Evaluation_Framework.md | Test cases and feedback |
| Roadmap.md | Planned features |
| Bug_Report.md | Known issues |
| Version_History.md | Changelog |

---

## Personality Stems

Base prompts in `PE/Agent Files/Templates/Personality Stems/`:

| Stem | Model | Use Case |
|------|-------|----------|
| Helper | Haiku | Constrained tasks, queue processing |
| Assistant | Sonnet | General purpose, scheduling |
| Coach (v1.2) | Sonnet/Opus | Teaching, feedback, motivation |
| SME | Opus | Deep expertise, research |

---

## Functions

Reusable modules in `PE/Agent Files/Templates/Functions/`:

| Function | Version | Purpose |
|----------|---------|---------|
| Interview Function | v0.2 | Structured info gathering when context insufficient |
| Lazy File Loading | v0.1 | Load support files on demand, not at startup |
| Language Coach Interview Extensions | v1.0 | Domain-specific interview patterns for language coaches |

---

## Coach Templates

Templates for language coach operations in `PE/Agent Files/Templates/`:

| Template | Purpose |
|----------|---------|
| Coach_Conversation_Log_Template.md | Session logging for coaching interactions |
| Coach_Student_Status_Template.md | Tracking individual student progress |

---

## Evaluation System

Bayesian hypothesis framework for agent assessment:

| Component | Location | Purpose |
|-----------|----------|---------|
| Evaluation_Framework.md | Each agent's `Agent Files/` | YAML hypotheses + test cases |
| Interactions/ | Each agent's `Agent Files/` | Transcript storage |
| Evaluator_Prompt.md | Team Files/ | Opus evaluation template |
| Global_Evaluation.md | Team Files/ | System-wide summary |

**Hypothesis status flow:** untested → testing (1+ obs) → supported (5+ obs, >80% consistent) or refuted (5+ obs, >40% inconsistent)

---

## Task Hierarchy (Speculative)

| Model | Cognitive Style | Task Types |
|-------|-----------------|------------|
| Opus | Discover & Create | Novel design, complex reasoning, research synthesis |
| Sonnet | Find & Explain | Pattern matching, clear explanation, moderate complexity |
| Haiku | Go Get & Replace | Routine lookup, simple substitution, queue processing |

---

## To Do

| Task | Owner | Priority |
|------|-------|----------|
| Deployment guide for local Rust setup with GitHub | Claude Code | Medium |
| Test Spanish Coach with external tester | Designer | P0 |
| Test Japanese and French Coaches | Designer | P1 |
| Decide: Interview function in Instructions.md or Conventions.md? | PM | Medium |
| Prepare documentation for instruction pattern testing | PE | Medium |
| Refine Weightlifting Programmer from draft | PE | Low |
| Refine Personal Assistant from draft | PE | Low |

---

## Deployed Specialists

Agents in `Specialists/` with full 7-file structure:

| Agent | Model | Status |
|-------|-------|--------|
| Spanish Coach (CR) | Sonnet | Ready for testing |
| Japanese Coach (Kyoto) | Sonnet | Ready for testing |
| French Coach (Quebec) | Sonnet | Ready for testing |

---

## Prospective Agents

### Helpers (Haiku)
- Tex/Lean Text Helper
- Research Coordinator
- Grader — evaluation framework management
- Reading Notes Helper

### Assistants (Sonnet)
- Personal Assistant/Scheduler — draft in PE Output
- Proof Assistant
- Data Analyst
- Scientific Literature Researcher

### Coaches (Sonnet/Opus)
- Language Coach (additional dialects)
  - Finnish
  - Chinese (Cantonese)
- Weightlifting Programmer — draft in PE Output (Sonnet, may escalate to Opus)

### SMEs (Opus)
- Research Assistant
- SME Researcher
- Copy Writer
- Web Designer

---

## Directory Structure

```
Claude-Agents/
├── Project Team (PT)/          # Development infrastructure
│   ├── src/                    # Rust CLI
│   ├── Team Files/             # Shared docs (this file, queue, workflow)
│   ├── Project Manager (PM)/
│   ├── Prompt Engineer (PE)/
│   │   ├── Agent Files/
│   │   │   └── Templates/      # Stems, Functions
│   │   ├── Output/             # Active designs
│   │   └── Archive/            # Processed designs
│   └── Project Coordinator (PC)/
│
└── Specialists/                # Deployed agents
    ├── Spanish Coach (CR)/
    ├── Japanese Coach (Kyoto)/
    └── French Coach (Quebec)/
```

---

## Reference

- [Claude_Code_Workflow.md](Claude_Code_Workflow.md) — Claude Code operating procedures
- [PM_Queue.md](PM_Queue.md) — Async task queue
- [Project_History.md](Project_History.md) — Major milestones
- [Conversation_Log_Template.md](Conversation_Log_Template.md) — Session logging format
- [Evaluator_Prompt.md](Evaluator_Prompt.md) — Interaction evaluation template
- [Global_Evaluation.md](Global_Evaluation.md) — System-wide evaluation summary
