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
                                      Local repo → GitHub

PC processes queue async ←── All agents write tasks
```

### Design → Implementation
1. PE designs agent prompts, deposits in `PE/Output/`
2. Claude Code picks up designs, scaffolds agent folders
3. PM coordinates priorities and decisions
4. PC tracks status via queue

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
| Coach | Sonnet/Opus | Teaching, feedback, motivation |
| SME | Opus | Deep expertise, research |

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
| | | |

---

## Prospective Agents

### Helpers (Haiku)
- Tex/Lean Text Helper
- Research Coordinator
- Grader — evaluation framework management
- Reading Notes Helper

### Assistants (Sonnet)
- Personal Assistant/Scheduler — scheduling, reminders, task management
- Proof Assistant
- Data Analyst
- Scientific Literature Researcher

### Coaches (Sonnet/Opus)
- Language Coach
  - Japanese (standard + Kyoto-ben)
  - French (standard + Quebec)
  - Finnish
  - Chinese (Cantonese)
  - Spanish (Costa Rican dialect) — **priority for deployment**
- Weightlifting Programmer (Sonnet, may escalate to Opus)

### SMEs (Opus)
- Research Assistant
- SME Researcher
- Copy Writer
- Web Designer

---

## Reference

- [Claude_Code_Workflow.md](Claude_Code_Workflow.md) — Claude Code operating procedures
- [PM_Queue.md](PM_Queue.md) — Async task queue
- [Project_History.md](Project_History.md) — Major milestones
