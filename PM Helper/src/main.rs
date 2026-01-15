// main.rs - Entry point for PM Helper Agent

mod agent;
mod toolkit;

use serde::Deserialize;
use std::env;
use std::fs;
use std::path::PathBuf;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const CONFIG_FILE: &str = "config.toml";

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
        if let Ok(content) = fs::read_to_string(&config_path) {
            if let Ok(config) = toml::from_str(&content) {
                return config;
            }
        }
    }
    Config::default()
}

fn get_api_key(config: &Config) -> Option<String> {
    env::var("ANTHROPIC_API_KEY").ok().or_else(|| config.api_key.clone())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let working_dir = env::current_dir()?;
    let config = load_config(&working_dir);

    let api_key = match get_api_key(&config) {
        Some(key) => key,
        None => {
            eprintln!("Error: No API key found");
            eprintln!("Set ANTHROPIC_API_KEY or add api_key to config.toml");
            std::process::exit(1);
        }
    };

    env::set_var("ANTHROPIC_API_KEY", &api_key);

    let model = config.model;

    let agent_files_dir = working_dir.join(toolkit::AGENT_FILES_DIR);
    if !agent_files_dir.exists() {
        eprintln!("Error: Agent Files directory not found");
        std::process::exit(1);
    }

    println!("=================================================");
    println!("PM Helper v{}", VERSION);
    println!("=================================================");
    println!("Working Directory: {}", working_dir.display());
    println!("Model: {}", model.as_deref().unwrap_or("claude-haiku-4-20250514"));
    println!("=================================================");
    println!("Starting TUI...\n");

    let agent_state = agent::AgentState::new(
        working_dir,
        model,
        "PM Helper".to_string(),
    )?;

    let tools = toolkit::get_standard_tools();
    agent::run_interactive(agent_state, tools).await?;

    Ok(())
}
