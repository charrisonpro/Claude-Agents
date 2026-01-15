// toolkit.rs - File operations and tool definitions for PM Helper

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
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
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

pub fn write_output(working_dir: &Path, filename: &str, content: &str) -> Result<(), std::io::Error> {
    let output_dir = working_dir.join(OUTPUT_DIR);
    fs::create_dir_all(&output_dir)?;
    fs::write(output_dir.join(filename), content)
}

/// Read the PM Queue from PE Framework
pub fn read_pm_queue(working_dir: &Path) -> Result<String, std::io::Error> {
    // PM Queue lives in sibling PE Framework directory
    let parent = working_dir.parent().ok_or_else(|| {
        std::io::Error::new(std::io::ErrorKind::NotFound, "Cannot find parent directory")
    })?;
    let queue_path = parent.join("Claude PE Framework").join("Agent Files").join("PM_Queue.md");
    fs::read_to_string(queue_path)
}

/// Write updated PM Queue back to PE Framework
pub fn write_pm_queue(working_dir: &Path, content: &str) -> Result<(), std::io::Error> {
    let parent = working_dir.parent().ok_or_else(|| {
        std::io::Error::new(std::io::ErrorKind::NotFound, "Cannot find parent directory")
    })?;
    let queue_path = parent.join("Claude PE Framework").join("Agent Files").join("PM_Queue.md");
    fs::write(queue_path, content)
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
        Tool {
            name: "write_output".to_string(),
            description: "Write a deliverable to Output directory (for status reports)".to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": { "filename": { "type": "string" }, "content": { "type": "string" } },
                "required": ["filename", "content"]
            }),
        },
        Tool {
            name: "read_pm_queue".to_string(),
            description: "Read the PM Task Queue from PE Framework".to_string(),
            input_schema: serde_json::json!({ "type": "object", "properties": {} }),
        },
        Tool {
            name: "write_pm_queue".to_string(),
            description: "Write updated PM Task Queue back to PE Framework".to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": { "content": { "type": "string" } },
                "required": ["content"]
            }),
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
        "write_output" => {
            let filename = input["filename"].as_str().ok_or("Missing filename")?;
            let content = input["content"].as_str().ok_or("Missing content")?;
            write_output(working_dir, filename, content).map_err(|e| e.to_string())?;
            Ok(format!("Wrote Output/{}", filename))
        }
        "read_pm_queue" => {
            read_pm_queue(working_dir).map_err(|e| e.to_string())
        }
        "write_pm_queue" => {
            let content = input["content"].as_str().ok_or("Missing content")?;
            write_pm_queue(working_dir, content).map_err(|e| e.to_string())?;
            Ok("Updated PM_Queue.md".to_string())
        }
        _ => Err(format!("Unknown tool: {}", name)),
    }
}
