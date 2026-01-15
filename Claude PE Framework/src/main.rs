// main.rs - Entry point for Claude Agent CLI
//
// Usage:
//   cargo run                      # Run in current directory
//   cargo run -- --path /some/dir  # Run with specific working directory
//   cargo run -- --model opus      # Use specific model (overrides config)

mod agent;
mod scaffold;
mod toolkit;

use serde::Deserialize;
use std::env;
use std::fs;
use std::path::PathBuf;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const CONFIG_FILE: &str = "config.toml";

// =============================================================================
// CONFIGURATION
// =============================================================================

#[derive(Debug, Deserialize, Default)]
struct Config {
    #[serde(default)]
    model: Option<String>,
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
    // Priority: env var > config file
    env::var("ANTHROPIC_API_KEY").ok().or_else(|| config.api_key.clone())
}

// =============================================================================
// CLI
// =============================================================================

fn print_usage() {
    println!("Claude Agent CLI v{}", VERSION);
    println!();
    println!("USAGE:");
    println!("    claude-agent [OPTIONS]");
    println!();
    println!("OPTIONS:");
    println!("    --path <DIR>     Working directory (default: current directory)");
    println!("    --model <MODEL>  Claude model to use (overrides config.toml)");
    println!("    --name <NAME>    Agent name for display (default: directory name)");
    println!("    --help           Show this help message");
    println!();
    println!("CONFIGURATION (config.toml):");
    println!("    model = \"claude-opus-4-20250514\"");
    println!("    api_key = \"sk-ant-...\"  # Optional, can also use ANTHROPIC_API_KEY env var");
    println!();
    println!("DIRECTORY STRUCTURE:");
    println!("    The working directory should contain:");
    println!("    config.toml                Optional configuration");
    println!("    Agent Files/");
    println!("        Instructions.md        Main agent instructions (required)");
    println!("        Toolkit.md             Tool documentation");
    println!("        Domain_Knowledge.md");
    println!("        Conventions.md");
    println!("        Version_History.md");
    println!("        Arch/                  Archived versions");
    println!("    History/                   Conversation logs");
    println!("    Output/                    Agent output files");
}

fn parse_args() -> Result<(PathBuf, Option<String>, Option<String>), String> {
    let args: Vec<String> = env::args().collect();

    let mut path: Option<PathBuf> = None;
    let mut model: Option<String> = None;
    let mut name: Option<String> = None;

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--help" | "-h" => {
                print_usage();
                std::process::exit(0);
            }
            "--path" | "-p" => {
                i += 1;
                if i >= args.len() {
                    return Err("--path requires a directory argument".to_string());
                }
                path = Some(PathBuf::from(&args[i]));
            }
            "--model" | "-m" => {
                i += 1;
                if i >= args.len() {
                    return Err("--model requires a model name argument".to_string());
                }
                model = Some(args[i].clone());
            }
            "--name" | "-n" => {
                i += 1;
                if i >= args.len() {
                    return Err("--name requires a name argument".to_string());
                }
                name = Some(args[i].clone());
            }
            arg => {
                return Err(format!("Unknown argument: {}", arg));
            }
        }
        i += 1;
    }

    let working_dir = path.unwrap_or_else(|| env::current_dir().expect("Failed to get current directory"));

    Ok((working_dir, model, name))
}

fn derive_agent_name(working_dir: &PathBuf, explicit_name: Option<String>) -> String {
    explicit_name.unwrap_or_else(|| {
        working_dir
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_else(|| "Agent".to_string())
    })
}

// =============================================================================
// MAIN
// =============================================================================

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Parse arguments first to get working directory
    let (working_dir, cli_model, name) = match parse_args() {
        Ok(args) => args,
        Err(e) => {
            eprintln!("Error: {}", e);
            eprintln!("Use --help for usage information");
            std::process::exit(1);
        }
    };

    // Verify working directory exists
    if !working_dir.exists() {
        eprintln!("Error: Working directory does not exist: {}", working_dir.display());
        std::process::exit(1);
    }

    // Load config from working directory
    let config = load_config(&working_dir);

    // Get API key (env var takes priority, then config file)
    let api_key = match get_api_key(&config) {
        Some(key) => key,
        None => {
            eprintln!("Error: No API key found");
            eprintln!("Set ANTHROPIC_API_KEY environment variable or add api_key to config.toml");
            std::process::exit(1);
        }
    };

    // Set the API key as env var for the agent module to use
    env::set_var("ANTHROPIC_API_KEY", &api_key);

    // Determine model: CLI arg > config file > default
    let has_custom_model = cli_model.is_some() || config.model.is_some();
    let model = cli_model.or(config.model);

    // Verify Agent Files directory exists
    let agent_files_dir = working_dir.join(toolkit::AGENT_FILES_DIR);
    if !agent_files_dir.exists() {
        eprintln!("Error: Agent Files directory not found at: {}", agent_files_dir.display());
        eprintln!("Expected structure:");
        eprintln!("  {}/", working_dir.display());
        eprintln!("    Agent Files/");
        eprintln!("      Instructions.md");
        std::process::exit(1);
    }

    // Derive agent name
    let agent_name = derive_agent_name(&working_dir, name);

    // Print startup info (before TUI takes over)
    println!("=================================================");
    println!("{}", agent_name);
    println!("=================================================");
    println!("Working Directory: {}", working_dir.display());
    if has_custom_model {
        println!("Model: {}", model.as_deref().unwrap_or("default"));
    }
    println!("Agent Files:");

    if let Ok(files) = toolkit::list_files(&working_dir) {
        for file in files {
            println!("  - {}", file);
        }
    }
    println!("=================================================");
    println!("Starting TUI...\n");

    // Initialize agent state
    let agent_state = agent::AgentState::new(working_dir, model, agent_name)?;

    // Get tools - standard file tools plus scaffold tools for PE agent
    let mut tools = toolkit::get_standard_tools();
    tools.extend(scaffold::get_scaffold_tools());

    // Run interactive TUI
    agent::run_interactive(agent_state, tools).await?;

    Ok(())
}
