# Master Plan

Strategic overview for the Claude Agents project.

---

## Current Sprint: 2026-01-21 → 2026-01-24

### Critical Path
1. Fix interview verbosity (all coaches) — discovery interview methodology
2. Design RAG/Knowledge Graph architecture for agent domain knowledge
3. AWS cloud deployment — Rust-wrapped agents, API endpoints
4. Website redesign — coach interface, research portfolio, Leaflet blog
5. File structure migration — Quality_Control/ consolidation

---

## Project Team

| Agent | Model | Role |
|-------|-------|------|
| Claude Code (Claudio) | Opus | Master implementation, file management, git |
| Project Manager (PM) | Flexible | Strategy, decisions, user sounding board |
| Prompt Engineer (PE) | Opus | Agent design, framework maintenance |
| Project Coordinator (PC) | Haiku | Async queue processing, status tracking |

---

## Architecture Paradigm Shift

### From: Ad-hoc file access
### To: RAG with Knowledge Graph

**Key insight:** Current system already implements RAG patterns — formalizing and optimizing.

**New approach:**
1. **Knowledge Graph foundation** — structured representation of domain knowledge
2. **Vectorization** — embed graph for efficient retrieval
3. **Chunked queries** — reference graph chunks to zero in on relevant data
4. **Context control** — compartmentalize responses and knowledge searches

**Open question:** How to integrate knowledge sets — part of RAG knowledge graph architecture?

---

## Infrastructure

### Current: GitHub Pages
- Static hosting at charrisonpro.github.io/Claude-Agents
- Language Lab with 3 coaches (Spanish, Japanese, French)
- localStorage for session data
- User-provided API keys

### Target: AWS Cloud

**Primary Goals:**
1. Host server for language coaches — gather user feedback and interaction data
2. Data Analytics/Data Science project hosting — subsequent use
3. Rust-wrapped Claude agents — production deployment
4. Backend for personal website — API endpoint for coach conversations

**Architecture:**
- EC2 Linux server
- Server-side API proxy (centralized API key)
- S3 for session storage
- Rust wrapper for Claude agents
- Interface layer for routing

### Multi-Agent Interface Strategy

**New paradigm:**
- **Primary interlocutor = Interface layer**
- Interface processes user questions
- Routes to appropriate prompt sets
- **One agent may split into several prompt sets called independently**
- Compartmentalized responses for context control

**Research needed:**
- Multi-agent implementation patterns
- Prompt set routing logic
- Context window management across splits

See: [AWS_MIGRATION_GUIDE.md](../../docs/AWS_MIGRATION_GUIDE.md)

---

## Personal Website Architecture

### Goals
1. **Professional research portfolio** — showcase data projects for grad school
2. **Language coach interface** — connects to AWS backend for live conversations
3. **Blog/content distribution** — via Leaflet (AT Protocol)
4. **Aesthetic overhaul** — professional design upgrade

### Technical Stack
- **Frontend:** Website redesign with modern aesthetic
- **Backend:** AWS server running Rust-wrapped agents
- **API:** Connect website to AWS for agent conversations
- **Distribution:** Leaflet (AT Protocol) for blog/content publishing

### Components

**1. Language Coach Interface**
- Browser-based chat UI
- Connects to AWS backend via API
- Background data collection for evaluation
- No manual feedback required from users

**2. Research Portfolio**
- Professional presentation of data projects
- Data visualization capabilities
- Academic credibility for grad school applications

**3. Blog/Content Platform**
- Leaflet integration for AT Protocol distribution
- Content published through AT Protocol network
- Preferential platform for all work distribution going forward

**4. Aesthetic Redesign**
- Modern, professional design
- Cohesive visual identity
- Academic yet accessible presentation

**Decision:** AT Protocol (Leaflet) is preferential platform for all work distribution going forward.

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

**Root level (3 files):**
| File | Purpose |
|------|---------|
| Instructions.md | Core identity and behavior |
| Domain_Knowledge.md | Facts and patterns |
| Conventions.md | Procedures and formats |

**Quality_Control/ subfolder (4 files):**
| File | Purpose |
|------|---------|
| Evaluation_Framework.md | Test cases and feedback |
| Roadmap.md | Planned features |
| Bug_Report.md | Known issues |
| Version_History.md | Changelog |

**Decision:** Agent file structure reorganized — 3 files in root (Instructions, Domain_Knowledge, Conventions), 4 in Quality_Control/ subfolder.

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

### P0 — Architecture & Infrastructure
| Task | Owner | Status |
|------|-------|--------|
| Restructure project documentation to use RAG/Knowledge Graph terminology | PM + PE | Pending |
| Design knowledge graph structure for agent domain knowledge | PE | Pending |
| Research vectorization strategies for knowledge graph chunks | Designer | Pending |
| Research multi-agent implementation patterns — interface routing, prompt set splitting, context management | Designer | Pending |
| Design AWS deployment architecture — Rust wrapper for Claude agents, interface layer for routing, API endpoints for website | Claude Code + Designer | Pending |
| Design interface layer for language coaches — processes user input, routes to appropriate prompt sets | PE + Claude Code | Pending |

### P1 — Website Development
| Task | Owner | Status |
|------|-------|--------|
| Design website architecture — connects to AWS backend, Leaflet integration for blog, research portfolio section | Web Designer + Webmaster | Pending |
| Implement language coach browser interface — chat UI connecting to AWS API endpoint | Web Designer + Claude Code | Pending |
| Design aesthetic overhaul for personal website — modern professional design, cohesive visual identity | Web Designer | Pending |
| Integrate Leaflet (AT Protocol) for blog and content distribution | Webmaster + Claude Code | Pending |
| Design API endpoints for AWS server — enable website to AWS agent communication | Claude Code | Pending |
| Implement data collection pipeline — capture interactions from website interface for evaluation framework | Claude Code | Pending |
| Set up blog infrastructure using Leaflet — content creation and distribution via AT Protocol | Designer + Webmaster | Pending |

### P2 — File Structure & Templates
| Task | Owner | Status |
|------|-------|--------|
| Update Master_Plan agent template — reflect Quality_Control/ folder structure | Claude Code | Pending |
| Update standard agent scaffold in Rust tool — create Quality_Control/ folder, move 4 files | Claude Code | Pending |
| Migrate existing agents to new structure — create Quality_Control/ folders, move files | Designer | In Progress (manual) |

### P3 — Testing & Evaluation
| Task | Owner | Status |
|------|-------|--------|
| Design Bayesian testing framework with success rubric template — P(success|model), P(success|file) with 3-5 point scale | PE | Pending |
| Design agent collaboration logging function for Claude Code — track handoffs, model interlocutors, PT coordination | PE | Pending |
| Implement collaboration logger in Rust toolkit | Claude Code | Pending |

### P4 — Language Coach Improvements
| Task | Owner | Status |
|------|-------|--------|
| Redesign interview function to guide questions rather than present walls — use discovery interview methodology, build trust incrementally | PE | Pending |
| Research discovery interview best practices and resources | Designer | Pending |
| Design "absorb outside lesson" function for language coaches — extracts grammar, vocab, curriculum context from external learning | PE | Pending |
| Research and implement vocal coaching capability for language coaches — pronunciation feedback, listening practice | Designer + Claude Code | Pending |

### P5 — Dialect Support
| Task | Owner | Status |
|------|-------|--------|
| Locate and catalog dialect support resources (videos, text) for each language coach | Designer | Pending |
| Design behavior for coaches to interact with external dialect resources | PE | Pending |
| Reorganize language coach prompt structure — standard language focus with dialect differentiation as layer | PE | Pending |
| Add to Language Coach roadmap — student choice of standard vs. dialect | PE | Pending |

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
├── Project_Team_(PT)/          # Development infrastructure
│   ├── src/                    # Rust CLI
│   ├── Team_Files/             # Shared docs
│   ├── Project_Manager_(PM)/
│   ├── Prompt_Engineer_(PE)/
│   │   ├── Agent_Files/
│   │   │   └── Templates/      # Stems, Functions
│   │   ├── Output/             # Active designs
│   │   └── Archive/            # Processed designs
│   └── Project_Coordinator_(PC)/
│
└── Specialists/                # Deployed agents
    └── [Agent Name]/
        └── Agent_Files/
            ├── Instructions.md
            ├── Domain_Knowledge.md
            ├── Conventions.md
            └── Quality_Control/
                ├── Evaluation_Framework.md
                ├── Roadmap.md
                ├── Bug_Report.md
                └── Version_History.md
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
