// main.rs - Entry point for Project Team CLI
//
// Usage:
//   cargo run -- pm     # Run Project Manager
//   cargo run -- pe     # Run Prompt Engineer
//   cargo run -- pc     # Run Project Coordinator
//   cargo run -- help   # Show help

mod agent;
mod scaffold;
mod toolkit;

use serde::Deserialize;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::PathBuf;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const CONFIG_FILE: &str = "config.toml";

// =============================================================================
// AGENT REGISTRY
// =============================================================================

struct AgentInfo {
    name: &'static str,
    folder: &'static str,
    model: &'static str,
    has_scaffold: bool,
    has_project_explore: bool,
}

fn get_agents() -> HashMap<&'static str, AgentInfo> {
    let mut agents = HashMap::new();
    agents.insert("pm", AgentInfo {
        name: "Project Manager",
        folder: "Project Manager (PM)",
        model: "claude-sonnet-4-20250514",
        has_scaffold: false,
        has_project_explore: true,  // PM can explore all agent files
    });
    agents.insert("pe", AgentInfo {
        name: "Prompt Engineer",
        folder: "Prompt Engieer (PE)",  // Note: typo preserved from existing folder
        model: "claude-opus-4-20250514",
        has_scaffold: true,
        has_project_explore: false,
    });
    agents.insert("pc", AgentInfo {
        name: "Project Coordinator",
        folder: "Project Coordinator (PC)",
        model: "claude-haiku-4-20250514",
        has_scaffold: false,
        has_project_explore: false,
    });
    agents
}

// =============================================================================
// CONFIGURATION
// =============================================================================

#[derive(Debug, Deserialize, Default)]
struct Config {
    #[serde(default)]
    api_key: Option<String>,
}

fn load_config(working_dir: &PathBuf) -> Config {
    let config_path = working_dir.join(CONFIG_FILE);
    if config_path.exists() {
        match fs::read_to_string(&config_path) {
            Ok(content) => {
                match toml::from_str(&content) {
                    Ok(config) => return config,
                    Err(e) => eprintln!("Warning: Failed to parse {}: {}", CONFIG_FILE, e),
                }
            }
            Err(e) => eprintln!("Warning: Failed to read {}: {}", CONFIG_FILE, e),
        }
    }
    Config::default()
}

fn get_api_key(config: &Config) -> Option<String> {
    env::var("ANTHROPIC_API_KEY").ok().or_else(|| config.api_key.clone())
}

// =============================================================================
// CLI
// =============================================================================

fn print_usage() {
    println!("Project Team CLI v{}", VERSION);
    println!();
    println!("USAGE:");
    println!("    cargo run -- <agent>");
    println!();
    println!("AGENTS:");
    println!("    pm      Project Manager (Sonnet) - Strategy & decisions");
    println!("    pe      Prompt Engineer (Opus) - Agent design");
    println!("    pc      Project Coordinator (Haiku) - Queue processing");
    println!();
    println!("OPTIONS:");
    println!("    help    Show this help message");
    println!();
    println!("EXAMPLES:");
    println!("    cargo run -- pm     # Start Project Manager");
    println!("    cargo run -- pe     # Start Prompt Engineer");
    println!("    cargo run -- pc     # Start Project Coordinator");
}

// =============================================================================
// MAIN
// =============================================================================

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    // Need at least one argument (the agent name)
    if args.len() < 2 {
        print_usage();
        std::process::exit(1);
    }

    let agent_key = args[1].to_lowercase();

    if agent_key == "help" || agent_key == "--help" || agent_key == "-h" {
        print_usage();
        return Ok(());
    }

    let agents = get_agents();

    let agent_info = match agents.get(agent_key.as_str()) {
        Some(info) => info,
        None => {
            eprintln!("Error: Unknown agent '{}'", agent_key);
            eprintln!();
            eprintln!("Available agents: pm, pe, pc");
            eprintln!("Use 'cargo run -- help' for usage information");
            std::process::exit(1);
        }
    };

    // Get the Project Team directory (where Cargo.toml lives)
    let project_team_dir = env::current_dir()?;

    // Agent's working directory
    let agent_dir = project_team_dir.join(agent_info.folder);

    if !agent_dir.exists() {
        eprintln!("Error: Agent directory not found: {}", agent_dir.display());
        std::process::exit(1);
    }

    // Load config from Project Team directory
    let config = load_config(&project_team_dir);

    // Get API key
    let api_key = match get_api_key(&config) {
        Some(key) => key,
        None => {
            eprintln!("Error: No API key found");
            eprintln!("Set ANTHROPIC_API_KEY environment variable or add api_key to config.toml");
            std::process::exit(1);
        }
    };

    env::set_var("ANTHROPIC_API_KEY", &api_key);

    // Verify Agent Files directory exists
    let agent_files_dir = agent_dir.join(toolkit::AGENT_FILES_DIR);
    if !agent_files_dir.exists() {
        eprintln!("Error: Agent Files directory not found at: {}", agent_files_dir.display());
        std::process::exit(1);
    }

    // Print startup info
    println!("=================================================");
    println!("{}", agent_info.name);
    println!("=================================================");
    println!("Working Directory: {}", agent_dir.display());
    println!("Model: {}", agent_info.model);
    println!("=================================================");
    println!("Starting TUI...\n");

    // Initialize agent state
    let agent_state = agent::AgentState::new(
        agent_dir,
        Some(agent_info.model.to_string()),
        agent_info.name.to_string(),
    )?;

    // Get tools - standard file tools, plus agent-specific tools
    let mut tools = toolkit::get_standard_tools();

    // Add queue tools for all agents
    tools.extend(get_queue_tools());

    // Add scaffold tools only for PE
    if agent_info.has_scaffold {
        tools.extend(scaffold::get_scaffold_tools());
    }

    // Add project exploration tools for PM
    if agent_info.has_project_explore {
        tools.extend(get_project_explore_tools());
    }

    // Run interactive TUI
    agent::run_interactive(agent_state, tools).await?;

    Ok(())
}

// =============================================================================
// QUEUE TOOLS
// =============================================================================

fn get_queue_tools() -> Vec<toolkit::Tool> {
    vec![
        toolkit::Tool {
            name: "read_queue".to_string(),
            description: "Read the shared PM_Queue.md from Team Files.".to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {}
            }),
        },
        toolkit::Tool {
            name: "write_queue".to_string(),
            description: "Write updated content to PM_Queue.md in Team Files.".to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {
                    "content": {
                        "type": "string",
                        "description": "Full queue content to write"
                    }
                },
                "required": ["content"]
            }),
        },
    ]
}

// =============================================================================
// PROJECT EXPLORATION TOOLS (PM only)
// =============================================================================

fn get_project_explore_tools() -> Vec<toolkit::Tool> {
    vec![
        toolkit::Tool {
            name: "list_agents".to_string(),
            description: "List all agent directories in the Project Team.".to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {}
            }),
        },
        toolkit::Tool {
            name: "read_agent_file".to_string(),
            description: "Read a file from any agent's Agent Files directory. Use to review Evaluation_Framework.md, Bug_Report.md, etc.".to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {
                    "agent": {
                        "type": "string",
                        "description": "Agent folder name (e.g., 'Project Coordinator (PC)')"
                    },
                    "filename": {
                        "type": "string",
                        "description": "File to read (e.g., 'Evaluation_Framework.md', 'Bug_Report.md')"
                    }
                },
                "required": ["agent", "filename"]
            }),
        },
        toolkit::Tool {
            name: "read_team_file".to_string(),
            description: "Read a file from Team Files directory.".to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {
                    "filename": {
                        "type": "string",
                        "description": "File to read (e.g., 'Master_Plan.md', 'Toolkit.md')"
                    }
                },
                "required": ["filename"]
            }),
        },
        toolkit::Tool {
            name: "write_master_plan".to_string(),
            description: "Update Master_Plan.md in Team Files. Use to update the To Do section.".to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {
                    "content": {
                        "type": "string",
                        "description": "Full Master_Plan.md content"
                    }
                },
                "required": ["content"]
            }),
        },
    ]
}
