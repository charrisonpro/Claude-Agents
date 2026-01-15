# PE Framework Toolkit

Reference documentation for available tools. Read this file when you need to use file operations or project scaffolding.

---

## File Tools

Standard tools for managing files within this agent's directories.

### read_file
Read a file from the `Agent Files/` directory.

**Parameters:**
- `filename` (required): Relative path to the file (e.g., `Domain_Knowledge.md` or `Arch/v1.0.md`)

**Example:**
```json
{"filename": "Domain_Knowledge.md"}
```

---

### write_file
Write content to a file in `Agent Files/`. Creates or overwrites.

**Parameters:**
- `filename` (required): Relative path for the file
- `content` (required): Content to write

**Example:**
```json
{"filename": "notes.md", "content": "# Notes\n\nSome content here."}
```

---

### list_files
List available files in `Agent Files/`.

**Parameters:**
- `include_archive` (optional): Set `true` to include `Arch/` folder contents

**Example:**
```json
{"include_archive": true}
```

---

### save_history
Save content to the `History/` directory for session persistence.

**Parameters:**
- `filename` (required): Filename for the history entry
- `content` (required): Content to save

---

### write_output
Write deliverables to the `Output/` directory.

**Parameters:**
- `filename` (required): Filename for the output
- `content` (required): Content to write

---

## Project Scaffolding Tools

Tools for creating and managing agent projects.

### scaffold_project
Create a complete new agent project with full directory structure and Rust source files.

**Creates:**
- `src/main.rs`, `src/agent.rs`, `src/toolkit.rs`
- `Agent Files/` with template Instructions, Domain_Knowledge, Conventions, Version_History
- `History/` and `Output/` directories
- `Cargo.toml` and `README.md`

**Parameters:**
- `project_name` (required): Name of the agent (becomes folder name)
- `description` (required): Brief description of what the agent does
- `project_path` (optional): Absolute path for project. If omitted, creates as sibling directory to current agent.
- `model` (optional): Claude model to use (default: claude-sonnet-4-20250514)

**Example - minimal:**
```json
{
  "project_name": "Code Review Agent",
  "description": "Reviews code for best practices and security issues"
}
```
Creates: `../Code Review Agent/`

**Example - with path:**
```json
{
  "project_name": "Research Assistant",
  "description": "Helps with literature review and summarization",
  "project_path": "/Users/me/projects/research-agent",
  "model": "claude-sonnet-4-20250514"
}
```

---

### list_agents
Discover existing agent projects in sibling directories. Looks for folders containing `Agent Files/Instructions.md`.

**Parameters:** None

**Returns:** List of found agents with paths and descriptions (from first line of Instructions.md)

---

### list_project_structure
Display the standard directory layout for agent projects.

**Parameters:** None

**Returns:** ASCII tree showing expected structure

---

## Standard Agent Project Structure

```
{ProjectName}/
├── Cargo.toml                 # Rust project configuration
├── README.md                  # Project documentation
├── src/
│   ├── main.rs               # Entry point, CLI args
│   ├── agent.rs              # TUI and Claude API logic
│   └── toolkit.rs            # File operation tools
├── Agent Files/
│   ├── Instructions.md       # Main agent instructions
│   ├── Domain_Knowledge.md   # Subject matter content
│   ├── Conventions.md        # Output format standards
│   ├── Version_History.md    # Changelog
│   └── Arch/                 # Archived versions
├── History/
│   └── conversation_log.md   # Auto-saved conversations
└── Output/
    └── [deliverables]        # Agent output files
```
