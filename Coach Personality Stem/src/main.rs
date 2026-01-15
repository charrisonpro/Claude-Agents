// main.rs - Entry point for Coach Personality Stem Agent

mod agent;
mod toolkit;

use std::env;
use std::path::PathBuf;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    if env::var("ANTHROPIC_API_KEY").is_err() {
        eprintln!("Error: ANTHROPIC_API_KEY environment variable not set");
        std::process::exit(1);
    }

    let working_dir = env::current_dir()?;

    // Verify Agent Files exists
    let agent_files_dir = working_dir.join(toolkit::AGENT_FILES_DIR);
    if !agent_files_dir.exists() {
        eprintln!("Error: Agent Files directory not found");
        std::process::exit(1);
    }

    println!("Starting Coach Personality Stem Agent...");

    let agent_state = agent::AgentState::new(
        working_dir,
        None,
        "Coach Personality Stem".to_string(),
    )?;

    let tools = toolkit::get_standard_tools();
    agent::run_interactive(agent_state, tools).await?;

    Ok(())
}
