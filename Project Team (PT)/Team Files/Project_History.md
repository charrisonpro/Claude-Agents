# Project History
This will record the history of agent project over all.

## 2025-01-19 — Language Coach Deployment & Directory Restructure

### First Specialist Agents Deployed
Three language coaches moved from design to deployment in `Specialists/`:
- **Spanish Coach (Costa Rican)** — Full 7-file structure, evaluation framework, ready for external testing
- **Japanese Coach (Kyoto-ben)** — Full 7-file structure, recognition-first approach
- **French Coach (Québécois)** — Full 7-file structure, ear-training focus

All coaches extend Coach Stem v1.2 and implement Language Coach Interview Extensions v1.0.

### Directory Structure Formalized
- **Specialists/** folder established for all non-Project Team agents
- **PE/Archive/** created for processed Output files
- Output → Archive protocol documented in Claude_Code_Workflow.md

### Templates/Functions Established
New function modules distributed to `PE/Agent Files/Templates/Functions/`:
- Interview Function v0.2
- Lazy File Loading v0.1
- Language Coach Interview Extensions v1.0

### Infrastructure Updates
- PM Instructions updated with project exploration tools (list_agents, read_agent_file, read_team_file, write_master_plan)
- Toolkit.md updated with project exploration documentation
- Session logging added to all agent Instructions with model-appropriate guidance
- Conversation_Log_Template.md created in Team Files

### PM Kernel
PM Kernel v0.1 added to PM Agent Files — dual-mode operation (Opus planning / Sonnet interactive) with priority stack framework.

---

## 2025-01-16 — Agent Expansion & Modular Design Framework

### New Design Principle
Adopted modular function construction: build capabilities as separate, testable functions before integration into agent prompts. Allows systematic testing and cross-agent adaptation.

### PM Architecture Evolution
PM designated as model-flexible (Sonnet/Opus) to support dual operational modes:
- Planning mode (Opus) — strategic work with queue interaction, room for specialized tools
- Sounding board/counseling mode (Sonnet) — interactive sessions

Planning function built modularly, unique to PM initially but designed for potential adaptation.

### New Agent Capabilities
**Interview Mode:** Default behavior when agents lack conversation context; invokable when detailed information needed. Applies across all agent types, adaptable for SME knowledge extraction.

**Selective File Loading:** Agents read files only when tasks require them, rather than loading everything at startup. Experiment in context management through multi-step processes.

**Self-Testing Recording:** Agents can record their own testing data for transmission back to development team. Enables remote testing with external collaborators.

### Task Hierarchy Speculation
Exploring model-based task distribution:
- Opus: discover & create
- Sonnet: find and explain  
- Haiku: go get and replace

### Prospective Agents Added
- **Spanish Language Coach** (Costa Rican dialect, Sonnet) — P0 for Sunday deployment with external tester
- Personal Assistant/Scheduler (Sonnet)
- Weightlifting Programmer (Sonnet, may escalate to Opus)
- Scientific Literature Researcher (Sonnet)
- SME Researcher
- Data Analyst
- Reading Notes Helper (Haiku)

### Major Project Test Planned
Systematic comparison of instruction patterns: trigger phrases referencing Conventions.md vs. full instruction blocks in main prompt. Will inform architectural decisions across all agents.

### Deployment
Preparing deployment guide for local Rust directory setup with GitHub integration to support external testing.