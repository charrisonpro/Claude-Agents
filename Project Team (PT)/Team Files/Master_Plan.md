# Master Plan

Strategic overview for the Claude Agents project.

---

## Project Team

| Agent | Model | Role |
|-------|-------|------|
| Claude Code (Claudio) | Opus | Master implementation, file management, git |
| Project Manager (PM) | Flexible | Strategy, decisions, user sounding board |
| Prompt Engineer (PE) | Opus | Agent design, framework maintenance |
| Project Coordinator (PC) | Haiku | Async queue processing, status tracking |

---

## Current Sprint: 2025-01-20 → 2025-01-22

### Critical Path
1. Fix interview verbosity (all coaches) — discovery interview methodology
2. Deploy PA, Reading Notes, Weightlifter by Wednesday
3. Integrate PM/PE domain knowledge
4. AWS cloud migration for durable deployment
5. Design Webmaster/Web Designer for deployment support

### Deployment Targets (by Wednesday 2025-01-22)
| Agent | Status | Owner |
|-------|--------|-------|
| Personal Assistant/Scheduler | Draft exists | PE → Claude Code |
| Reading Notes Helper | To design | PE → Claude Code |
| Weightlifting Programmer | Draft exists | PE → Claude Code |

---

## Infrastructure

### Current: GitHub Pages
- Static hosting at charrisonpro.github.io/Claude-Agents
- Language Lab with 3 coaches (Spanish, Japanese, French)
- localStorage for session data
- User-provided API keys

### Target: AWS Cloud (by Wednesday)
- EC2 Linux server
- Server-side API proxy (centralized API key)
- S3 for session storage
- Terminal-based Claude Code interaction
- Full evaluation system implementation

See: [AWS_MIGRATION_GUIDE.md](../../docs/AWS_MIGRATION_GUIDE.md)

---

## Workflow

```
User ←→ PM (strategy) ←→ PE (design) → Output/
                                           ↓
                                    Claude Code (implement)
                                           ↓
                                    Specialists/ → GitHub → AWS
```

### Design → Implementation
1. PE designs agent prompts, deposits in `PE/Output/`
2. Claude Code picks up designs, scaffolds agent folders in `Specialists/`
3. Claude Code moves processed Output files to `PE/Archive/`
4. PM coordinates priorities and decisions
5. PC tracks status via queue

### Development Principle
**Build functions separately before integration.** Design and test new capabilities as standalone functions first, then integrate into agents once proven.

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

| Function | Version | Status | Purpose |
|----------|---------|--------|---------|
| Interview Function | v0.2 | **NEEDS REVISION** | Structured info gathering — requires discovery interview methodology |
| Lazy File Loading | v0.1 | Active | Load support files on demand |
| Language Coach Interview Extensions | v1.0 | **NEEDS REVISION** | Domain-specific interview — too verbose |
| Outside Lesson Absorption | Planned | Design needed | Extract grammar/vocab from external learning |

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

## Task Hierarchy

| Model | Cognitive Style | Task Types |
|-------|-----------------|------------|
| Opus | Discover & Create | Novel design, complex reasoning, research synthesis |
| Sonnet | Find & Explain | Pattern matching, clear explanation, moderate complexity |
| Haiku | Go Get & Replace | Routine lookup, simple substitution, queue processing |

---

## Critical Issues (from 2025-01-19 Testing)

### 1. Interview Verbosity
- **Problem:** Wall of questions instead of guided discovery
- **Fix:** Redesign interview function with discovery methodology, sequential questions, slower trust-building
- **Owner:** PE

### 2. Dialect Support Infrastructure
- **Problem:** No external dialect resources integrated
- **Fix:** Locate/catalog resources, design resource interaction behavior, reorganize prompt structure
- **Owner:** Designer + PE

### 3. Vocal/Audio Coaching
- **Problem:** Text-only insufficient for pronunciation work
- **Fix:** Research and implement voice capability
- **Owner:** Designer + Claude Code

---

## To Do (Priority Order)

### P0 — This Week
| Task | Owner | Due |
|------|-------|-----|
| Redesign interview function — discovery methodology | PE | ASAP |
| Finalize Personal Assistant design | PE | Tue |
| Design Reading Notes Helper | PE | Tue |
| Finalize Weightlifting Programmer | PE | Tue |
| Deploy PA, Reading Notes, Weightlifter | Claude Code | Wed |
| AWS EC2 setup and migration | Designer + Claude Code | Wed |

### P1 — Next
| Task | Owner |
|------|-------|
| Design Webmaster specialist agent | PE |
| Design Web Designer specialist agent | PE |
| Design "absorb outside lesson" function | PE |
| Locate dialect support resources (videos, text) | Designer |
| Research vocal coaching capability | Designer |

### P2 — Ongoing
| Task | Owner |
|------|-------|
| Integrate PM domain knowledge (agile, project mgmt) | Designer |
| Integrate PE domain knowledge (prompt eng research) | Designer |
| Gather language learning materials per coach | Designer |
| Locate reactive training material for Weightlifter | Designer |

---

## Deployed Specialists

Agents in `Specialists/` with full 7-file structure:

| Agent | Model | Status | Web |
|-------|-------|--------|-----|
| Spanish Coach (CR) | Sonnet | Active | ✓ |
| Japanese Coach (Kyoto) | Sonnet | Active | ✓ |
| French Coach (Quebec) | Sonnet | Active | ✓ |

---

## Prospective Agents

### Deploy by Wednesday
- **Personal Assistant/Scheduler** (Sonnet) — draft exists
- **Reading Notes Helper** (Haiku) — to design
- **Weightlifting Programmer** (Sonnet) — draft exists

### Helpers (Haiku)
- Tex/Lean Text Helper
- Research Coordinator
- Grader — evaluation framework management

### Assistants (Sonnet)
- Proof Assistant
- Data Analyst
- Scientific Literature Researcher

### Coaches (Sonnet/Opus)
- Language Coach (additional dialects): Finnish, Chinese (Cantonese)

### SMEs (Opus)
- Research Assistant
- SME Researcher
- Copy Writer
- **Webmaster** — design/deploy websites (NEW)
- **Web Designer** — UI/UX for browser interfaces (NEW)

---

## Domain Knowledge Integration

Priority agents needing curated background material:

| Agent | Materials Needed | Status |
|-------|------------------|--------|
| PM | Project mgmt, Agile/Scrum, decision frameworks | To gather |
| PE | Prompt engineering research, instruction design | To gather |
| Language Coaches | Standard lang materials + dialect resources | To gather |
| Weightlifting Programmer | Reactive training, programming methodology | To gather |

**Timeline:** Researcher support expected February 2025

---

## Directory Structure

```
Claude-Agents/
├── docs/                       # GitHub Pages / AWS static site
│   ├── coach/                  # Language Lab interfaces
│   ├── assets/                 # CSS, JS
│   ├── GUIDES.md               # Operational guides
│   ├── DEPLOY.md               # GitHub Pages deployment
│   └── AWS_MIGRATION_GUIDE.md  # AWS setup guide
│
├── Project Team (PT)/          # Development infrastructure
│   ├── src/                    # Rust CLI
│   ├── Team Files/             # Shared docs
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
- [AWS_MIGRATION_GUIDE.md](../../docs/AWS_MIGRATION_GUIDE.md) — Cloud deployment guide
- [GUIDES.md](../../docs/GUIDES.md) — Language Lab operational guides
