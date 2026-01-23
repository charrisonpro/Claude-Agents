// toolkit.rs - File operations and tool definitions for Claude agents
//
// This module provides:
// - File read/write/list operations for Agent Files
// - Tool definitions for the Claude API
// - Tool execution dispatch

use serde::Serialize;
use std::fs;
use std::path::{Path, PathBuf};
use tokio;

// =============================================================================
// CONFIGURATION
// =============================================================================

pub const AGENT_FILES_DIR: &str = "Agent Files";
pub const HISTORY_DIR: &str = "History";
pub const OUTPUT_DIR: &str = "Output";
pub const TEAM_FILES_DIR: &str = "Team Files";
pub const QUEUE_FILE: &str = "PM_Queue.md";

// =============================================================================
// TOOL SCHEMA TYPES
// =============================================================================

#[derive(Debug, Clone, Serialize)]
pub struct Tool {
    pub name: String,
    pub description: String,
    pub input_schema: serde_json::Value,
}

// =============================================================================
// FILE OPERATIONS
// =============================================================================

/// Read a file from the Agent Files directory
pub fn read_file(working_dir: &Path, filename: &str) -> Result<String, std::io::Error> {
    let filepath = working_dir.join(AGENT_FILES_DIR).join(filename);
    fs::read_to_string(filepath)
}

/// Write content to a file in the Agent Files directory
pub fn write_file(
    working_dir: &Path,
    filename: &str,
    content: &str,
) -> Result<(), std::io::Error> {
    let filepath = working_dir.join(AGENT_FILES_DIR).join(filename);
    if let Some(parent) = filepath.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(&filepath, content)
}

/// List files in the Agent Files directory
pub fn list_files(working_dir: &Path) -> Result<Vec<String>, std::io::Error> {
    list_files_with_options(working_dir, false)
}

/// List files with option to include archived files
pub fn list_files_with_options(working_dir: &Path, include_archive: bool) -> Result<Vec<String>, std::io::Error> {
    let agent_dir = working_dir.join(AGENT_FILES_DIR);
    let mut files = Vec::new();
    list_files_recursive(&agent_dir, &agent_dir, &mut files, include_archive)?;
    files.sort();
    Ok(files)
}

/// Recursively list files, preserving relative paths
fn list_files_recursive(
    base_dir: &Path,
    current_dir: &Path,
    files: &mut Vec<String>,
    include_archive: bool,
) -> Result<(), std::io::Error> {
    if !current_dir.exists() {
        return Ok(());
    }

    for entry in fs::read_dir(current_dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            if let Some(dir_name) = path.file_name() {
                if dir_name == "Arch" && !include_archive {
                    continue;
                }
            }
            list_files_recursive(base_dir, &path, files, include_archive)?;
        } else if path.is_file() {
            if let Some(ext) = path.extension() {
                if ext == "md" || ext == "txt" || ext == "json" {
                    if let Ok(relative) = path.strip_prefix(base_dir) {
                        files.push(relative.to_string_lossy().to_string());
                    }
                }
            }
        }
    }
    Ok(())
}

/// Read multiple files in parallel
pub async fn read_files_parallel(
    working_dir: PathBuf,
    filenames: Vec<String>,
) -> Vec<(String, Result<String, String>)> {
    let tasks: Vec<_> = filenames
        .into_iter()
        .map(|filename| {
            let dir = working_dir.clone();
            tokio::spawn(async move {
                let result = read_file(&dir, &filename)
                    .map_err(|e| format!("Error reading {}: {}", filename, e));
                (filename, result)
            })
        })
        .collect();

    let mut results = Vec::new();
    for task in tasks {
        if let Ok(result) = task.await {
            results.push(result);
        }
    }
    results
}

// =============================================================================
// HISTORY OPERATIONS
// =============================================================================

/// Save content to the History directory
pub fn save_to_history(
    working_dir: &Path,
    filename: &str,
    content: &str,
) -> Result<(), std::io::Error> {
    let history_dir = working_dir.join(HISTORY_DIR);
    fs::create_dir_all(&history_dir)?;
    let filepath = history_dir.join(filename);
    fs::write(&filepath, content)
}

/// Read from the History directory
#[allow(dead_code)]
pub fn read_from_history(working_dir: &Path, filename: &str) -> Result<String, std::io::Error> {
    let filepath = working_dir.join(HISTORY_DIR).join(filename);
    fs::read_to_string(filepath)
}

// =============================================================================
// OUTPUT OPERATIONS
// =============================================================================

/// Write to the Output directory
pub fn write_output(
    working_dir: &Path,
    filename: &str,
    content: &str,
) -> Result<(), std::io::Error> {
    let output_dir = working_dir.join(OUTPUT_DIR);
    fs::create_dir_all(&output_dir)?;
    let filepath = output_dir.join(filename);
    fs::write(&filepath, content)
}

// =============================================================================
// QUEUE OPERATIONS
// =============================================================================

/// Read the shared PM_Queue.md from Team Files
/// The queue is at the Project Team level, not in individual agent directories
pub fn read_queue(working_dir: &Path) -> Result<String, std::io::Error> {
    // working_dir is agent's directory (e.g., "Project Manager (PM)")
    // Go up one level to Project Team, then into Team Files
    let parent = working_dir.parent().ok_or_else(|| {
        std::io::Error::new(std::io::ErrorKind::NotFound, "Cannot find parent directory")
    })?;
    let queue_path = parent.join(TEAM_FILES_DIR).join(QUEUE_FILE);
    fs::read_to_string(queue_path)
}

/// Write updated content to PM_Queue.md in Team Files
pub fn write_queue(working_dir: &Path, content: &str) -> Result<(), std::io::Error> {
    let parent = working_dir.parent().ok_or_else(|| {
        std::io::Error::new(std::io::ErrorKind::NotFound, "Cannot find parent directory")
    })?;
    let queue_path = parent.join(TEAM_FILES_DIR).join(QUEUE_FILE);
    fs::write(queue_path, content)
}

// =============================================================================
// PROJECT EXPLORATION (PM tools)
// =============================================================================

/// List all agent directories in the Project Team
pub fn list_agents(working_dir: &Path) -> Result<Vec<String>, std::io::Error> {
    let parent = working_dir.parent().ok_or_else(|| {
        std::io::Error::new(std::io::ErrorKind::NotFound, "Cannot find parent directory")
    })?;

    let mut agents = Vec::new();
    for entry in fs::read_dir(parent)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            // Check if it has Agent Files/ subdirectory
            if path.join(AGENT_FILES_DIR).exists() {
                if let Some(name) = path.file_name() {
                    agents.push(name.to_string_lossy().to_string());
                }
            }
        }
    }
    agents.sort();
    Ok(agents)
}

/// Read a file from any agent's directory
pub fn read_agent_file(working_dir: &Path, agent: &str, filename: &str) -> Result<String, std::io::Error> {
    let parent = working_dir.parent().ok_or_else(|| {
        std::io::Error::new(std::io::ErrorKind::NotFound, "Cannot find parent directory")
    })?;
    let filepath = parent.join(agent).join(AGENT_FILES_DIR).join(filename);
    fs::read_to_string(filepath)
}

/// Read a file from Team Files
pub fn read_team_file(working_dir: &Path, filename: &str) -> Result<String, std::io::Error> {
    let parent = working_dir.parent().ok_or_else(|| {
        std::io::Error::new(std::io::ErrorKind::NotFound, "Cannot find parent directory")
    })?;
    let filepath = parent.join(TEAM_FILES_DIR).join(filename);
    fs::read_to_string(filepath)
}

/// Write to Master_Plan.md in Team Files
pub fn write_master_plan(working_dir: &Path, content: &str) -> Result<(), std::io::Error> {
    let parent = working_dir.parent().ok_or_else(|| {
        std::io::Error::new(std::io::ErrorKind::NotFound, "Cannot find parent directory")
    })?;
    let filepath = parent.join(TEAM_FILES_DIR).join("Master_Plan.md");
    fs::write(filepath, content)
}

// =============================================================================
// TOOL DEFINITIONS
// =============================================================================

/// Get standard file tools available to all agents
pub fn get_standard_tools() -> Vec<Tool> {
    vec![
        Tool {
            name: "read_file".to_string(),
            description: "Read a file from the Agent Files directory. Use relative paths like 'Domain_Knowledge.md' or 'Arch/v1.0.md'.".to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {
                    "filename": {
                        "type": "string",
                        "description": "Relative path to the file"
                    }
                },
                "required": ["filename"]
            }),
        },
        Tool {
            name: "write_file".to_string(),
            description: "Write content to a file in the Agent Files directory. Creates or overwrites.".to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {
                    "filename": {
                        "type": "string",
                        "description": "Relative path for the file"
                    },
                    "content": {
                        "type": "string",
                        "description": "Content to write"
                    }
                },
                "required": ["filename", "content"]
            }),
        },
        Tool {
            name: "list_files".to_string(),
            description: "List all files in the Agent Files directory. Set include_archive=true to include Arch/ folder.".to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {
                    "include_archive": {
                        "type": "boolean",
                        "description": "Include archived versions from Arch/ folder"
                    }
                }
            }),
        },
        Tool {
            name: "save_history".to_string(),
            description: "Save content to the History directory for session persistence.".to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {
                    "filename": {
                        "type": "string",
                        "description": "Filename for the history entry"
                    },
                    "content": {
                        "type": "string",
                        "description": "Content to save"
                    }
                },
                "required": ["filename", "content"]
            }),
        },
        Tool {
            name: "write_output".to_string(),
            description: "Write content to the Output directory for final deliverables.".to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {
                    "filename": {
                        "type": "string",
                        "description": "Filename for the output"
                    },
                    "content": {
                        "type": "string",
                        "description": "Content to write"
                    }
                },
                "required": ["filename", "content"]
            }),
        },
    ]
}

// =============================================================================
// TOOL EXECUTION
// =============================================================================

/// Execute a tool by name with given input
pub fn execute_tool(
    tool_name: &str,
    tool_input: &serde_json::Value,
    working_dir: &Path,
) -> Result<String, String> {
    match tool_name {
        "read_file" => {
            let filename = tool_input["filename"]
                .as_str()
                .ok_or("Missing filename parameter")?;
            read_file(working_dir, filename)
                .map_err(|e| format!("Error reading file: {}", e))
        }
        "write_file" => {
            let filename = tool_input["filename"]
                .as_str()
                .ok_or("Missing filename parameter")?;
            let content = tool_input["content"]
                .as_str()
                .ok_or("Missing content parameter")?;
            write_file(working_dir, filename, content)
                .map_err(|e| format!("Error writing file: {}", e))?;
            Ok(format!("Successfully wrote to {}", filename))
        }
        "list_files" => {
            let include_archive = tool_input["include_archive"].as_bool().unwrap_or(false);
            let files = list_files_with_options(working_dir, include_archive)
                .map_err(|e| format!("Error listing files: {}", e))?;
            if files.is_empty() {
                Ok("No files found in Agent Files directory".to_string())
            } else {
                Ok(format!("Files:\n{}", files.join("\n")))
            }
        }
        "save_history" => {
            let filename = tool_input["filename"]
                .as_str()
                .ok_or("Missing filename parameter")?;
            let content = tool_input["content"]
                .as_str()
                .ok_or("Missing content parameter")?;
            save_to_history(working_dir, filename, content)
                .map_err(|e| format!("Error saving history: {}", e))?;
            Ok(format!("Saved to History/{}", filename))
        }
        "write_output" => {
            let filename = tool_input["filename"]
                .as_str()
                .ok_or("Missing filename parameter")?;
            let content = tool_input["content"]
                .as_str()
                .ok_or("Missing content parameter")?;
            write_output(working_dir, filename, content)
                .map_err(|e| format!("Error writing output: {}", e))?;
            Ok(format!("Wrote to Output/{}", filename))
        }
        "read_queue" => {
            read_queue(working_dir)
                .map_err(|e| format!("Error reading queue: {}", e))
        }
        "write_queue" => {
            let content = tool_input["content"]
                .as_str()
                .ok_or("Missing content parameter")?;
            write_queue(working_dir, content)
                .map_err(|e| format!("Error writing queue: {}", e))?;
            Ok("Updated PM_Queue.md".to_string())
        }
        "list_agents" => {
            let agents = list_agents(working_dir)
                .map_err(|e| format!("Error listing agents: {}", e))?;
            if agents.is_empty() {
                Ok("No agent directories found".to_string())
            } else {
                Ok(format!("Agents:\n{}", agents.join("\n")))
            }
        }
        "read_agent_file" => {
            let agent = tool_input["agent"]
                .as_str()
                .ok_or("Missing agent parameter")?;
            let filename = tool_input["filename"]
                .as_str()
                .ok_or("Missing filename parameter")?;
            read_agent_file(working_dir, agent, filename)
                .map_err(|e| format!("Error reading {}/{}: {}", agent, filename, e))
        }
        "read_team_file" => {
            let filename = tool_input["filename"]
                .as_str()
                .ok_or("Missing filename parameter")?;
            read_team_file(working_dir, filename)
                .map_err(|e| format!("Error reading Team Files/{}: {}", filename, e))
        }
        "write_master_plan" => {
            let content = tool_input["content"]
                .as_str()
                .ok_or("Missing content parameter")?;
            write_master_plan(working_dir, content)
                .map_err(|e| format!("Error writing Master_Plan.md: {}", e))?;
            Ok("Updated Master_Plan.md".to_string())
        }
        // Forward to scaffold tools if not a standard tool
        _ => crate::scaffold::execute_scaffold_tool(tool_name, tool_input, working_dir),
    }
}

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_tool_definitions_valid() {
        let tools = get_standard_tools();
        assert!(!tools.is_empty());
        for tool in tools {
            assert!(!tool.name.is_empty());
            assert!(!tool.description.is_empty());
        }
    }
}
