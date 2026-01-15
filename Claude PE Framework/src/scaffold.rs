// scaffold.rs - Tools for scaffolding new agent projects
//
// This module provides:
// - Project directory structure creation
// - Template file generation
// - Cargo.toml generation for new agents

use crate::toolkit::Tool;
use std::fs;
use std::path::{Path, PathBuf};

// =============================================================================
// DIRECTORY STRUCTURE
// =============================================================================

const DIRS: &[&str] = &[
    "src",
    "Agent Files",
    "Agent Files/Arch",
    "History",
    "Output",
];

// =============================================================================
// SCAFFOLD FUNCTIONS
// =============================================================================

/// Initialize a new agent project with complete directory structure and template files
pub fn scaffold_project(
    project_path: &Path,
    project_name: &str,
    description: &str,
    model: Option<&str>,
) -> Result<String, String> {
    // Validate project path
    if !project_path.exists() {
        return Err(format!("Project path does not exist: {}", project_path.display()));
    }

    let mut status = Vec::new();

    // Create directories
    for dir in DIRS {
        let dir_path = project_path.join(dir);
        if !dir_path.exists() {
            fs::create_dir_all(&dir_path)
                .map_err(|e| format!("Failed to create {}: {}", dir, e))?;
            status.push(format!("Created: {}/", dir));
        } else {
            status.push(format!("Exists: {}/", dir));
        }
    }

    // Create template files
    let templates = get_templates(project_name, description, model);

    for (path, content) in templates {
        let file_path = project_path.join(&path);
        if file_path.exists() {
            status.push(format!("Exists (not modified): {}", path));
        } else {
            // Create parent directories if needed
            if let Some(parent) = file_path.parent() {
                fs::create_dir_all(parent)
                    .map_err(|e| format!("Failed to create parent dir for {}: {}", path, e))?;
            }
            fs::write(&file_path, content)
                .map_err(|e| format!("Failed to write {}: {}", path, e))?;
            status.push(format!("Created: {}", path));
        }
    }

    // Add .gitkeep files
    for dir in &["History", "Output"] {
        let gitkeep = project_path.join(dir).join(".gitkeep");
        if !gitkeep.exists() {
            fs::write(&gitkeep, "").map_err(|e| format!("Failed to create .gitkeep: {}", e))?;
        }
    }

    Ok(status.join("\n"))
}

/// Get all template files for a new project
fn get_templates(name: &str, description: &str, model: Option<&str>) -> Vec<(String, String)> {
    let model_str = model.unwrap_or("claude-sonnet-4-20250514");

    vec![
        ("Cargo.toml".to_string(), generate_cargo_toml(name, description)),
        ("src/main.rs".to_string(), generate_main_rs(name)),
        ("src/agent.rs".to_string(), AGENT_RS_TEMPLATE.to_string()),
        ("src/toolkit.rs".to_string(), TOOLKIT_RS_TEMPLATE.to_string()),
        ("Agent Files/Instructions.md".to_string(), generate_instructions(name, description, model_str)),
        ("Agent Files/Toolkit.md".to_string(), generate_toolkit_docs()),
        ("Agent Files/Domain_Knowledge.md".to_string(), generate_domain_knowledge(name)),
        ("Agent Files/Conventions.md".to_string(), generate_conventions(name)),
        ("Agent Files/Version_History.md".to_string(), generate_version_history(name)),
        ("README.md".to_string(), generate_readme(name, description)),
        ("agent.ipynb".to_string(), generate_notebook(name, model_str)),
    ]
}

// =============================================================================
// TEMPLATE GENERATORS
// =============================================================================

fn generate_cargo_toml(name: &str, description: &str) -> String {
    let snake_name = to_snake_case(name);
    format!(r#"[package]
name = "{}"
version = "0.1.0"
edition = "2021"
description = "{}"

[dependencies]
tokio = {{ version = "1.35", features = ["full"] }}
reqwest = {{ version = "0.11", features = ["json"] }}
serde = {{ version = "1.0", features = ["derive"] }}
serde_json = "1.0"
regex = "1.10"
ratatui = "0.26"
crossterm = "0.27"
"#, snake_name, description)
}

fn generate_main_rs(name: &str) -> String {
    format!(r#"// main.rs - Entry point for {} Agent

mod agent;
mod toolkit;

use std::env;
use std::path::PathBuf;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {{
    if env::var("ANTHROPIC_API_KEY").is_err() {{
        eprintln!("Error: ANTHROPIC_API_KEY environment variable not set");
        std::process::exit(1);
    }}

    let working_dir = env::current_dir()?;

    // Verify Agent Files exists
    let agent_files_dir = working_dir.join(toolkit::AGENT_FILES_DIR);
    if !agent_files_dir.exists() {{
        eprintln!("Error: Agent Files directory not found");
        std::process::exit(1);
    }}

    println!("Starting {} Agent...");

    let agent_state = agent::AgentState::new(
        working_dir,
        None,
        "{}".to_string(),
    )?;

    let tools = toolkit::get_standard_tools();
    agent::run_interactive(agent_state, tools).await?;

    Ok(())
}}
"#, name, name, name)
}

fn generate_instructions(name: &str, description: &str, model: &str) -> String {
    format!(r#"# {} - Instructions

## Overview

{}

## Model

This agent runs on: {}

## Role

You are the {} agent. Your purpose is to [describe specific purpose].

## Available Tools

You have access to file tools. **Read `Toolkit.md` for full documentation.**

Quick reference:
- **File tools:** `read_file`, `write_file`, `list_files`, `save_history`, `write_output`

## Behavior Guidelines

1. [Guideline 1]
2. [Guideline 2]
3. [Guideline 3]

## Output Format

[Describe expected output format]

## Examples

[Add examples of expected behavior]
"#, name, description, model, name)
}

fn generate_toolkit_docs() -> String {
    r#"# Toolkit

Reference documentation for available tools. Read this file when you need tool details.

---

## File Tools

### read_file
Read a file from `Agent Files/`.

**Parameters:**
- `filename` (required): Relative path to the file

---

### write_file
Write content to `Agent Files/`. Creates or overwrites.

**Parameters:**
- `filename` (required): Relative path
- `content` (required): Content to write

---

### list_files
List available files in `Agent Files/`.

**Parameters:**
- `include_archive` (optional): Set `true` to include `Arch/` contents

---

### save_history
Save content to `History/` for session persistence.

**Parameters:**
- `filename` (required): Filename
- `content` (required): Content to save

---

### write_output
Write deliverables to `Output/`.

**Parameters:**
- `filename` (required): Filename
- `content` (required): Content to write
"#.to_string()
}

fn generate_domain_knowledge(name: &str) -> String {
    format!(r#"# {} - Domain Knowledge

## Key Concepts

[Define important domain concepts]

## Terminology

| Term | Definition |
|------|------------|
| [Term 1] | [Definition] |
| [Term 2] | [Definition] |

## Business Rules

[Document important business rules]

## Reference Materials

[Links to external documentation]

## Expert Notes

[Notes from subject matter experts]
"#, name)
}

fn generate_conventions(name: &str) -> String {
    format!(r#"# {} - Conventions

## File Naming

- Use snake_case for file names
- Use descriptive names that indicate content
- Version archived files with date suffix: `filename_YYYYMMDD.md`

## Output Format

[Define standard output format]

## Communication Style

[Define how the agent should communicate]

## Error Handling

[Define how errors should be reported]
"#, name)
}

fn generate_version_history(name: &str) -> String {
    format!(r#"# {} - Version History

## Current Version: 0.1.0

### v0.1.0 - Initial Setup

- Initial project structure created
- Base instructions defined
- Core tools configured

---

## Planned Updates

- [ ] [Future enhancement 1]
- [ ] [Future enhancement 2]
"#, name)
}

fn generate_readme(name: &str, description: &str) -> String {
    let snake_name = to_snake_case(name);
    format!(r#"# {}

{}

## Setup

1. Ensure you have Rust installed
2. Set your API key: `export ANTHROPIC_API_KEY=your-key`
3. Build: `cargo build --release`
4. Run: `cargo run --release`

## Directory Structure

```
{}/
├── Cargo.toml
├── src/
│   ├── main.rs        # Entry point
│   ├── agent.rs       # TUI and API logic
│   └── toolkit.rs     # File tools
├── Agent Files/
│   ├── Instructions.md
│   ├── Domain_Knowledge.md
│   ├── Conventions.md
│   ├── Version_History.md
│   └── Arch/
├── History/
└── Output/
```

## Usage

- Type messages and press Enter to send
- Use `@filename.md` to include file context
- Press Ctrl+C to quit (saves conversation)
- Use Up/Down arrows to scroll

## Tools Available

- `read_file` - Read from Agent Files
- `write_file` - Write to Agent Files
- `list_files` - List available files
- `save_history` - Save to History directory
- `write_output` - Write to Output directory
"#, name, description, snake_name)
}

fn generate_notebook(name: &str, model: &str) -> String {
    // Build notebook JSON using serde_json for proper escaping
    let setup_code = format!(r#"# Setup - Run once per session
import os
import json
import requests
from pathlib import Path
from IPython.display import display, Markdown

API_URL = "https://api.anthropic.com/v1/messages"
ANTHROPIC_VERSION = "2023-06-01"
MODEL = "{}"

WORKING_DIR = Path.cwd()
AGENT_FILES = WORKING_DIR / "Agent Files"
HISTORY_DIR = WORKING_DIR / "History"
OUTPUT_DIR = WORKING_DIR / "Output"

SYSTEM_PROMPT = (AGENT_FILES / "Instructions.md").read_text()
conversation_history = []

TOOLS = [
    {{"name": "read_file", "description": "Read a file from Agent Files directory", "input_schema": {{"type": "object", "properties": {{"filename": {{"type": "string"}}}}, "required": ["filename"]}}}},
    {{"name": "write_file", "description": "Write a file to Agent Files directory", "input_schema": {{"type": "object", "properties": {{"filename": {{"type": "string"}}, "content": {{"type": "string"}}}}, "required": ["filename", "content"]}}}},
    {{"name": "list_files", "description": "List files in Agent Files directory", "input_schema": {{"type": "object", "properties": {{}}}}}},
    {{"name": "write_output", "description": "Write a deliverable to Output directory", "input_schema": {{"type": "object", "properties": {{"filename": {{"type": "string"}}, "content": {{"type": "string"}}}}, "required": ["filename", "content"]}}}}
]

def execute_tool(name, inputs):
    if name == "read_file":
        path = AGENT_FILES / inputs["filename"]
        return path.read_text() if path.exists() else f"File not found: {{inputs['filename']}}"
    elif name == "write_file":
        path = AGENT_FILES / inputs["filename"]
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_text(inputs["content"])
        return f"Wrote {{inputs['filename']}}"
    elif name == "list_files":
        files = [f.name for f in AGENT_FILES.iterdir() if f.is_file()]
        return "\n".join(sorted(files)) if files else "No files found"
    elif name == "write_output":
        path = OUTPUT_DIR / inputs["filename"]
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_text(inputs["content"])
        return f"Wrote output: {{inputs['filename']}}"
    return f"Unknown tool: {{name}}"

def chat(message):
    global conversation_history
    api_key = os.environ.get("ANTHROPIC_API_KEY")
    if not api_key:
        return "Error: ANTHROPIC_API_KEY not set"
    conversation_history.append({{"role": "user", "content": message}})
    while True:
        response = requests.post(API_URL, headers={{"x-api-key": api_key, "anthropic-version": ANTHROPIC_VERSION, "content-type": "application/json"}}, json={{"model": MODEL, "max_tokens": 4096, "system": SYSTEM_PROMPT, "messages": conversation_history, "tools": TOOLS}})
        if not response.ok:
            return f"API Error: {{response.text}}"
        data = response.json()
        conversation_history.append({{"role": "assistant", "content": data["content"]}})
        tool_uses = [b for b in data["content"] if b["type"] == "tool_use"]
        if not tool_uses:
            return "".join(b.get("text", "") for b in data["content"] if b["type"] == "text")
        tool_results = []
        for tool in tool_uses:
            print(f"[Tool: {{tool['name']}}]")
            result = execute_tool(tool["name"], tool["input"])
            tool_results.append({{"type": "tool_result", "tool_use_id": tool["id"], "content": result}})
        conversation_history.append({{"role": "user", "content": tool_results}})

def reset_conversation():
    global conversation_history
    conversation_history = []
    print("Conversation reset.")

print(f"{} Agent ready. Working directory: {{WORKING_DIR}}")"#, model, name);

    let chat_code = r#"# Chat - Edit message and run
message = """
Hello! What can you help me with?
"""

response = chat(message.strip())
display(Markdown(response))"#;

    let view_code = r#"# View Agent File - Change filename and run
filename = "Instructions.md"

content = (AGENT_FILES / filename).read_text()
display(Markdown(f"**{filename}**\n\n---\n\n{content}"))"#;

    let edit_code = r#"# Edit Agent File - Modify and run to save
filename = "Instructions.md"
content = """
# Your content here

Edit this cell with the file content, then run to save.
"""

# Uncomment the line below to save:
# (AGENT_FILES / filename).write_text(content.strip())
# print(f"Saved {filename}")"#;

    let list_code = r#"# List all Agent Files
print("Agent Files:")
for f in sorted(AGENT_FILES.iterdir()):
    if f.is_file():
        print(f"  {f.name}")
    elif f.is_dir():
        print(f"  {f.name}/")
        for sub in sorted(f.iterdir()):
            print(f"    {sub.name}")"#;

    let reset_code = "# Reset conversation\nreset_conversation()";

    let save_code = r#"# Save conversation to History
from datetime import datetime

timestamp = datetime.now().strftime("%Y%m%d_%H%M%S")
history_file = HISTORY_DIR / f"conversation_{timestamp}.json"
history_file.write_text(json.dumps(conversation_history, indent=2))
print(f"Saved to {history_file}")"#;

    let notebook = serde_json::json!({
        "cells": [
            {
                "cell_type": "markdown",
                "metadata": {},
                "source": [format!("# {} Agent\n\nRun the setup cell once, then use the chat cell for conversation.", name)]
            },
            {
                "cell_type": "code",
                "execution_count": null,
                "metadata": {},
                "outputs": [],
                "source": [setup_code]
            },
            {
                "cell_type": "code",
                "execution_count": null,
                "metadata": {},
                "outputs": [],
                "source": [chat_code]
            },
            {
                "cell_type": "markdown",
                "metadata": {},
                "source": ["---\n## Agent Files\n\nUse the cells below to view and edit agent files directly."]
            },
            {
                "cell_type": "code",
                "execution_count": null,
                "metadata": {},
                "outputs": [],
                "source": [view_code]
            },
            {
                "cell_type": "code",
                "execution_count": null,
                "metadata": {},
                "outputs": [],
                "source": [edit_code]
            },
            {
                "cell_type": "code",
                "execution_count": null,
                "metadata": {},
                "outputs": [],
                "source": [list_code]
            },
            {
                "cell_type": "markdown",
                "metadata": {},
                "source": ["---\n## Utilities"]
            },
            {
                "cell_type": "code",
                "execution_count": null,
                "metadata": {},
                "outputs": [],
                "source": [reset_code]
            },
            {
                "cell_type": "code",
                "execution_count": null,
                "metadata": {},
                "outputs": [],
                "source": [save_code]
            }
        ],
        "metadata": {
            "kernelspec": {
                "display_name": "Python 3",
                "language": "python",
                "name": "python3"
            },
            "language_info": {
                "name": "python",
                "version": "3.11.0"
            }
        },
        "nbformat": 4,
        "nbformat_minor": 4
    });

    serde_json::to_string_pretty(&notebook).unwrap_or_else(|_| "{}".to_string())
}

fn to_snake_case(s: &str) -> String {
    let mut result = String::new();
    for (i, c) in s.chars().enumerate() {
        if c.is_uppercase() {
            if i > 0 {
                result.push('_');
            }
            result.push(c.to_lowercase().next().unwrap());
        } else if c == ' ' || c == '-' {
            result.push('_');
        } else {
            result.push(c);
        }
    }
    result
}

// =============================================================================
// TOOL DEFINITIONS
// =============================================================================

/// Get scaffold-specific tools for the PE agent
pub fn get_scaffold_tools() -> Vec<Tool> {
    vec![
        Tool {
            name: "scaffold_project".to_string(),
            description: "Create a new agent project with complete directory structure, Rust source files, and template Agent Files. Creates everything needed for a new TUI-based Claude agent. If project_path is not provided, creates the project as a sibling directory to the current working directory.".to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {
                    "project_name": {
                        "type": "string",
                        "description": "Name of the agent (e.g., 'Code Review Agent', 'Research Assistant'). This becomes the folder name."
                    },
                    "description": {
                        "type": "string",
                        "description": "Brief description of what this agent does"
                    },
                    "project_path": {
                        "type": "string",
                        "description": "Optional: Absolute path where the project folder should be created. If omitted, creates as sibling to current directory."
                    },
                    "model": {
                        "type": "string",
                        "description": "Claude model to use (default: claude-sonnet-4-20250514)"
                    }
                },
                "required": ["project_name", "description"]
            }),
        },
        Tool {
            name: "list_project_structure".to_string(),
            description: "List the expected directory structure for an agent project.".to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {}
            }),
        },
        Tool {
            name: "list_agents".to_string(),
            description: "List all directories in the parent folder (assumes all are agent projects).".to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {}
            }),
        },
    ]
}

/// Execute scaffold tools
pub fn execute_scaffold_tool(
    tool_name: &str,
    tool_input: &serde_json::Value,
    working_dir: &Path,
) -> Result<String, String> {
    match tool_name {
        "scaffold_project" => {
            let project_name = tool_input["project_name"]
                .as_str()
                .ok_or("Missing project_name parameter")?;
            let description = tool_input["description"]
                .as_str()
                .ok_or("Missing description parameter")?;
            let model = tool_input["model"].as_str();

            // Determine project path - use provided or default to sibling directory
            let project_path = if let Some(path) = tool_input["project_path"].as_str() {
                PathBuf::from(path)
            } else {
                // Default: create as sibling to current working directory
                let parent = working_dir.parent()
                    .ok_or("Cannot determine parent directory for sibling placement")?;
                parent.join(project_name)
            };

            // Create the project directory if it doesn't exist
            if !project_path.exists() {
                fs::create_dir_all(&project_path)
                    .map_err(|e| format!("Failed to create project directory: {}", e))?;
            }

            scaffold_project(&project_path, project_name, description, model)
        }
        "list_project_structure" => {
            Ok(r#"Agent Project Structure:

{project_name}/
├── Cargo.toml                 # Rust project configuration
├── README.md                  # Project documentation
├── src/
│   ├── main.rs               # Entry point
│   ├── agent.rs              # TUI and Claude API logic
│   └── toolkit.rs            # File operation tools
├── Agent Files/
│   ├── Instructions.md       # Main agent instructions (system prompt)
│   ├── Domain_Knowledge.md   # Subject matter content
│   ├── Conventions.md        # Output format standards
│   ├── Version_History.md    # Changelog
│   └── Arch/                 # Archived versions
├── History/
│   └── conversation_log.md   # Auto-saved conversations
└── Output/
    └── [deliverables]        # Agent output files
"#.to_string())
        }
        "list_agents" => {
            list_sibling_agents(working_dir)
        }
        _ => Err(format!("Unknown scaffold tool: {}", tool_name)),
    }
}

/// Find agent projects in sibling directories
/// Assumes all directories in the parent folder are agent projects
fn list_sibling_agents(working_dir: &Path) -> Result<String, String> {
    let parent = working_dir.parent()
        .ok_or("Cannot determine parent directory")?;

    let mut agents = Vec::new();

    let entries = fs::read_dir(parent)
        .map_err(|e| format!("Failed to read parent directory: {}", e))?;

    for entry in entries {
        let entry = entry.map_err(|e| format!("Failed to read entry: {}", e))?;
        let path = entry.path();

        if path.is_dir() {
            let name = path.file_name()
                .map(|n| n.to_string_lossy().to_string())
                .unwrap_or_else(|| "Unknown".to_string());

            // Try to get description from Instructions.md if it exists
            let instructions = path.join("Agent Files").join("Instructions.md");
            let desc = if instructions.exists() {
                fs::read_to_string(&instructions)
                    .ok()
                    .and_then(|content| content.lines().next().map(|l| l.trim_start_matches('#').trim().to_string()))
                    .unwrap_or_else(|| "Agent project".to_string())
            } else {
                "Agent project".to_string()
            };

            agents.push(format!("- **{}**\n  Path: {}\n  {}", name, path.display(), desc));
        }
    }

    if agents.is_empty() {
        Ok("No directories found.".to_string())
    } else {
        Ok(format!("Found {} project(s):\n\n{}", agents.len(), agents.join("\n\n")))
    }
}

// =============================================================================
// EMBEDDED TEMPLATES (for generated projects)
// =============================================================================

const AGENT_RS_TEMPLATE: &str = r#"// agent.rs - TUI interface and Claude API communication
// This is a generated file - see PE Framework for the source template

use crate::toolkit::{self, Tool};

use std::env;
use std::fs;
use std::io;
use std::path::PathBuf;
use std::sync::Arc;

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState, Wrap},
    Frame, Terminal,
};
use reqwest;
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc;

const API_URL: &str = "https://api.anthropic.com/v1/messages";
const ANTHROPIC_VERSION: &str = "2023-06-01";
const DEFAULT_MODEL: &str = "claude-sonnet-4-20250514";
const INSTRUCTIONS_FILE: &str = "Instructions.md";

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MessageContent {
    Text(String),
    Blocks(Vec<ContentBlock>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub role: String,
    pub content: MessageContent,
}

#[derive(Debug, Serialize)]
struct ApiRequest {
    model: String,
    max_tokens: u32,
    system: String,
    messages: Vec<Message>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tools: Option<Vec<Tool>>,
}

#[derive(Debug, Deserialize)]
struct ApiResponse {
    content: Vec<ContentBlock>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentBlock {
    #[serde(rename = "type")]
    pub content_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_use_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
}

#[derive(Debug, Clone)]
pub struct AgentState {
    pub system_prompt: String,
    pub conversation_history: Vec<Message>,
    pub model: String,
    pub working_directory: PathBuf,
    pub agent_name: String,
}

impl AgentState {
    pub fn new(working_dir: PathBuf, model: Option<String>, agent_name: String) -> Result<Self, Box<dyn std::error::Error>> {
        let instructions_path = working_dir.join(toolkit::AGENT_FILES_DIR).join(INSTRUCTIONS_FILE);
        let system_prompt = fs::read_to_string(&instructions_path)?;
        Ok(Self {
            system_prompt,
            conversation_history: Vec::new(),
            model: model.unwrap_or_else(|| DEFAULT_MODEL.to_string()),
            working_directory: working_dir,
            agent_name,
        })
    }
}

#[derive(Clone)]
pub enum ChatMessage {
    User(String),
    Assistant(String),
    System(String),
    ToolUse(String),
    Error(String),
}

#[derive(Clone)]
pub enum TuiEvent {
    Response(String),
    ToolUse(String),
    Thinking,
    Error(String),
    Done,
}

pub struct TuiApp {
    pub messages: Vec<ChatMessage>,
    pub input: String,
    pub input_cursor: usize,
    pub scroll_offset: usize,
    pub status: String,
    pub is_waiting: bool,
    pub agent_name: String,
}

impl TuiApp {
    pub fn new(agent_name: &str) -> Self {
        Self {
            messages: vec![ChatMessage::System(format!("{} - Ctrl+C to quit", agent_name))],
            input: String::new(),
            input_cursor: 0,
            scroll_offset: 0,
            status: "Ready".to_string(),
            is_waiting: false,
            agent_name: agent_name.to_string(),
        }
    }

    pub fn add_message(&mut self, msg: ChatMessage) {
        self.messages.push(msg);
        self.scroll_offset = usize::MAX;
    }

    pub fn take_input(&mut self) -> String {
        let input = self.input.clone();
        self.input.clear();
        self.input_cursor = 0;
        input
    }
}

pub fn render_tui(frame: &mut Frame, app: &mut TuiApp) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(5), Constraint::Length(1), Constraint::Length(3)])
        .split(frame.size());

    // Messages
    let mut lines: Vec<Line> = Vec::new();
    for msg in &app.messages {
        let (prefix, style, content) = match msg {
            ChatMessage::User(t) => ("You: ", Style::default().fg(Color::Cyan), t.as_str()),
            ChatMessage::Assistant(t) => ("Agent: ", Style::default().fg(Color::Green), t.as_str()),
            ChatMessage::System(t) => ("» ", Style::default().fg(Color::Yellow), t.as_str()),
            ChatMessage::ToolUse(t) => ("⚙ ", Style::default().fg(Color::Magenta), t.as_str()),
            ChatMessage::Error(t) => ("✗ ", Style::default().fg(Color::Red), t.as_str()),
        };
        lines.push(Line::from(Span::styled(prefix, style)));
        for line in content.lines() {
            lines.push(Line::from(vec![Span::raw("  "), Span::styled(line, style)]));
        }
        lines.push(Line::from(""));
    }

    let total = lines.len();
    let visible = chunks[0].height.saturating_sub(2) as usize;
    let max_scroll = total.saturating_sub(visible);
    if app.scroll_offset > max_scroll { app.scroll_offset = max_scroll; }

    let msg_widget = Paragraph::new(Text::from(lines))
        .block(Block::default().borders(Borders::ALL).title(format!(" {} ", app.agent_name)))
        .wrap(Wrap { trim: false })
        .scroll((app.scroll_offset as u16, 0));
    frame.render_widget(msg_widget, chunks[0]);

    // Status
    let status = Paragraph::new(format!(" {} ", app.status))
        .style(if app.is_waiting { Style::default().fg(Color::Yellow) } else { Style::default().fg(Color::Gray) });
    frame.render_widget(status, chunks[1]);

    // Input
    let input = Paragraph::new(app.input.as_str())
        .block(Block::default().borders(Borders::ALL).title(" Input "));
    frame.render_widget(input, chunks[2]);
    if !app.is_waiting {
        frame.set_cursor(chunks[2].x + 1 + app.input_cursor as u16, chunks[2].y + 1);
    }
}

pub fn chat_with_agent(
    mut state: AgentState,
    msg: String,
    files: Option<Vec<String>>,
    tx: mpsc::UnboundedSender<TuiEvent>,
    tools: Vec<Tool>,
) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<AgentState, Box<dyn std::error::Error + Send + Sync>>> + Send>> {
    Box::pin(async move {
        let api_key = env::var("ANTHROPIC_API_KEY")?;
        let _ = tx.send(TuiEvent::Thinking);

        let mut enhanced = msg;
        if let Some(files) = files {
            for f in files {
                if let Ok(content) = toolkit::read_file(&state.working_directory, &f) {
                    enhanced.push_str(&format!("\n\n<file name='{}'>\n{}\n</file>", f, content));
                }
            }
        }

        state.conversation_history.push(Message { role: "user".to_string(), content: MessageContent::Text(enhanced) });

        let req = ApiRequest {
            model: state.model.clone(),
            max_tokens: 4096,
            system: state.system_prompt.clone(),
            messages: state.conversation_history.clone(),
            tools: if tools.is_empty() { None } else { Some(tools.clone()) },
        };

        let resp = reqwest::Client::new()
            .post(API_URL)
            .header("x-api-key", &api_key)
            .header("anthropic-version", ANTHROPIC_VERSION)
            .header("content-type", "application/json")
            .json(&req)
            .send()
            .await?;

        if !resp.status().is_success() {
            let err = resp.text().await?;
            let _ = tx.send(TuiEvent::Error(err.clone()));
            return Err(err.into());
        }

        let api_resp: ApiResponse = resp.json().await?;
        state.conversation_history.push(Message { role: "assistant".to_string(), content: MessageContent::Blocks(api_resp.content.clone()) });

        let has_tools = api_resp.content.iter().any(|b| b.content_type == "tool_use");
        if has_tools {
            let mut results = Vec::new();
            for block in &api_resp.content {
                if block.content_type == "tool_use" {
                    let name = block.name.as_ref().unwrap();
                    let input = block.input.as_ref().unwrap();
                    let id = block.id.as_ref().unwrap();
                    let _ = tx.send(TuiEvent::ToolUse(format!("Executing: {}", name)));
                    let result = toolkit::execute_tool(name, input, &state.working_directory)
                        .unwrap_or_else(|e| format!("Error: {}", e));
                    results.push(ContentBlock {
                        content_type: "tool_result".to_string(),
                        tool_use_id: Some(id.clone()),
                        content: Some(result),
                        text: None, id: None, name: None, input: None,
                    });
                }
            }
            state.conversation_history.push(Message { role: "user".to_string(), content: MessageContent::Blocks(results) });
            return chat_with_agent(state, String::new(), None, tx, tools).await;
        }

        let text: String = api_resp.content.iter().filter_map(|b| b.text.clone()).collect();
        let _ = tx.send(TuiEvent::Response(text));
        let _ = tx.send(TuiEvent::Done);
        Ok(state)
    })
}

pub async fn run_interactive(state: AgentState, tools: Vec<Tool>) -> Result<(), Box<dyn std::error::Error>> {
    let agent = Arc::new(tokio::sync::Mutex::new(state.clone()));

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout))?;

    let mut app = TuiApp::new(&state.agent_name);
    let (tx, mut rx) = mpsc::unbounded_channel::<TuiEvent>();

    loop {
        terminal.draw(|f| render_tui(f, &mut app))?;

        if event::poll(std::time::Duration::from_millis(50))? {
            if let Event::Key(key) = event::read()? {
                if key.modifiers.contains(KeyModifiers::CONTROL) && key.code == KeyCode::Char('c') {
                    break;
                }
                if app.is_waiting { continue; }
                match key.code {
                    KeyCode::Enter => {
                        let input = app.take_input();
                        if !input.is_empty() {
                            app.add_message(ChatMessage::User(input.clone()));
                            app.is_waiting = true;
                            app.status = "Thinking...".to_string();
                            let agent_clone = Arc::clone(&agent);
                            let tx_clone = tx.clone();
                            let tools_clone = tools.clone();
                            tokio::spawn(async move {
                                let mut guard = agent_clone.lock().await;
                                if let Ok(new) = chat_with_agent(guard.clone(), input, None, tx_clone, tools_clone).await {
                                    *guard = new;
                                }
                            });
                        }
                    }
                    KeyCode::Char(c) => { app.input.insert(app.input_cursor, c); app.input_cursor += 1; }
                    KeyCode::Backspace if app.input_cursor > 0 => { app.input_cursor -= 1; app.input.remove(app.input_cursor); }
                    KeyCode::Left => app.input_cursor = app.input_cursor.saturating_sub(1),
                    KeyCode::Right if app.input_cursor < app.input.len() => app.input_cursor += 1,
                    KeyCode::Up => app.scroll_offset = app.scroll_offset.saturating_sub(3),
                    KeyCode::Down => app.scroll_offset = app.scroll_offset.saturating_add(3),
                    _ => {}
                }
            }
        }

        while let Ok(ev) = rx.try_recv() {
            match ev {
                TuiEvent::Response(t) => app.add_message(ChatMessage::Assistant(t)),
                TuiEvent::ToolUse(t) => { app.status = t.clone(); app.add_message(ChatMessage::ToolUse(t)); }
                TuiEvent::Thinking => app.status = "Thinking...".to_string(),
                TuiEvent::Error(t) => app.add_message(ChatMessage::Error(t)),
                TuiEvent::Done => { app.is_waiting = false; app.status = "Ready".to_string(); }
            }
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen, DisableMouseCapture)?;
    Ok(())
}
"#;

const TOOLKIT_RS_TEMPLATE: &str = r#"// toolkit.rs - File operations and tool definitions
// This is a generated file - see PE Framework for the source template

use serde::Serialize;
use std::fs;
use std::path::Path;

pub const AGENT_FILES_DIR: &str = "Agent Files";
pub const HISTORY_DIR: &str = "History";
pub const OUTPUT_DIR: &str = "Output";

#[derive(Debug, Clone, Serialize)]
pub struct Tool {
    pub name: String,
    pub description: String,
    pub input_schema: serde_json::Value,
}

pub fn read_file(working_dir: &Path, filename: &str) -> Result<String, std::io::Error> {
    fs::read_to_string(working_dir.join(AGENT_FILES_DIR).join(filename))
}

pub fn write_file(working_dir: &Path, filename: &str, content: &str) -> Result<(), std::io::Error> {
    let path = working_dir.join(AGENT_FILES_DIR).join(filename);
    if let Some(parent) = path.parent() { fs::create_dir_all(parent)?; }
    fs::write(&path, content)
}

pub fn list_files(working_dir: &Path) -> Result<Vec<String>, std::io::Error> {
    let dir = working_dir.join(AGENT_FILES_DIR);
    let mut files = Vec::new();
    if dir.exists() {
        for entry in fs::read_dir(&dir)? {
            let entry = entry?;
            if entry.path().is_file() {
                if let Some(name) = entry.path().file_name() {
                    files.push(name.to_string_lossy().to_string());
                }
            }
        }
    }
    files.sort();
    Ok(files)
}

pub fn get_standard_tools() -> Vec<Tool> {
    vec![
        Tool {
            name: "read_file".to_string(),
            description: "Read a file from Agent Files directory".to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": { "filename": { "type": "string" } },
                "required": ["filename"]
            }),
        },
        Tool {
            name: "write_file".to_string(),
            description: "Write a file to Agent Files directory".to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": { "filename": { "type": "string" }, "content": { "type": "string" } },
                "required": ["filename", "content"]
            }),
        },
        Tool {
            name: "list_files".to_string(),
            description: "List files in Agent Files directory".to_string(),
            input_schema: serde_json::json!({ "type": "object", "properties": {} }),
        },
    ]
}

pub fn execute_tool(name: &str, input: &serde_json::Value, working_dir: &Path) -> Result<String, String> {
    match name {
        "read_file" => {
            let filename = input["filename"].as_str().ok_or("Missing filename")?;
            read_file(working_dir, filename).map_err(|e| e.to_string())
        }
        "write_file" => {
            let filename = input["filename"].as_str().ok_or("Missing filename")?;
            let content = input["content"].as_str().ok_or("Missing content")?;
            write_file(working_dir, filename, content).map_err(|e| e.to_string())?;
            Ok(format!("Wrote {}", filename))
        }
        "list_files" => {
            let files = list_files(working_dir).map_err(|e| e.to_string())?;
            Ok(files.join("\n"))
        }
        _ => Err(format!("Unknown tool: {}", name)),
    }
}
"#;
