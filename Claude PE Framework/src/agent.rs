// agent.rs - TUI interface and Claude API communication
//
// This module provides:
// - TUI rendering with ratatui
// - Claude API message handling
// - Conversation state management
// - Interactive chat loop

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

// =============================================================================
// CONFIGURATION
// =============================================================================

const API_URL: &str = "https://api.anthropic.com/v1/messages";
const ANTHROPIC_VERSION: &str = "2023-06-01";
const DEFAULT_MODEL: &str = "claude-opus-4-20250514";
const INSTRUCTIONS_FILE: &str = "Instructions.md";

// =============================================================================
// API DATA STRUCTURES
// =============================================================================

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
    #[serde(default)]
    #[allow(dead_code)]
    stop_reason: Option<String>,
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

// =============================================================================
// AGENT STATE
// =============================================================================

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
        let system_prompt = fs::read_to_string(&instructions_path)
            .map_err(|e| format!("Failed to read {}: {}", instructions_path.display(), e))?;

        Ok(Self {
            system_prompt,
            conversation_history: Vec::new(),
            model: model.unwrap_or_else(|| DEFAULT_MODEL.to_string()),
            working_directory: working_dir,
            agent_name,
        })
    }
}

// =============================================================================
// TUI DATA STRUCTURES
// =============================================================================

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
            messages: vec![ChatMessage::System(format!(
                "{} - Press Ctrl+C to quit, Enter to send",
                agent_name
            ))],
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
        self.scroll_to_bottom();
    }

    pub fn scroll_to_bottom(&mut self) {
        self.scroll_offset = usize::MAX;
    }

    pub fn scroll_up(&mut self, amount: usize) {
        self.scroll_offset = self.scroll_offset.saturating_sub(amount);
    }

    pub fn scroll_down(&mut self, amount: usize) {
        self.scroll_offset = self.scroll_offset.saturating_add(amount);
    }

    pub fn insert_char(&mut self, c: char) {
        self.input.insert(self.input_cursor, c);
        self.input_cursor += 1;
    }

    pub fn delete_char(&mut self) {
        if self.input_cursor > 0 {
            self.input_cursor -= 1;
            self.input.remove(self.input_cursor);
        }
    }

    pub fn delete_char_forward(&mut self) {
        if self.input_cursor < self.input.len() {
            self.input.remove(self.input_cursor);
        }
    }

    pub fn move_cursor_left(&mut self) {
        self.input_cursor = self.input_cursor.saturating_sub(1);
    }

    pub fn move_cursor_right(&mut self) {
        if self.input_cursor < self.input.len() {
            self.input_cursor += 1;
        }
    }

    pub fn move_cursor_home(&mut self) {
        self.input_cursor = 0;
    }

    pub fn move_cursor_end(&mut self) {
        self.input_cursor = self.input.len();
    }

    pub fn take_input(&mut self) -> String {
        let input = self.input.clone();
        self.input.clear();
        self.input_cursor = 0;
        input
    }
}

// =============================================================================
// TUI RENDERING
// =============================================================================

pub fn render_tui(frame: &mut Frame, app: &mut TuiApp) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(5),
            Constraint::Length(1),
            Constraint::Length(3),
        ])
        .split(frame.size());

    render_messages(frame, app, chunks[0]);
    render_status(frame, app, chunks[1]);
    render_input(frame, app, chunks[2]);
}

fn render_messages(frame: &mut Frame, app: &mut TuiApp, area: Rect) {
    let mut lines: Vec<Line> = Vec::new();

    for msg in &app.messages {
        let (prefix, style, content) = match msg {
            ChatMessage::User(text) => (
                "You: ",
                Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
                text.as_str(),
            ),
            ChatMessage::Assistant(text) => (
                "Agent: ",
                Style::default().fg(Color::Green),
                text.as_str(),
            ),
            ChatMessage::System(text) => (
                "» ",
                Style::default().fg(Color::Yellow).add_modifier(Modifier::ITALIC),
                text.as_str(),
            ),
            ChatMessage::ToolUse(text) => (
                "⚙ ",
                Style::default().fg(Color::Magenta),
                text.as_str(),
            ),
            ChatMessage::Error(text) => (
                "✗ ",
                Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
                text.as_str(),
            ),
        };

        lines.push(Line::from(vec![Span::styled(prefix, style)]));

        for line in content.lines() {
            lines.push(Line::from(vec![
                Span::raw("  "),
                Span::styled(line, style),
            ]));
        }

        lines.push(Line::from(""));
    }

    let total_lines = lines.len();
    let visible_height = area.height.saturating_sub(2) as usize;

    let max_scroll = total_lines.saturating_sub(visible_height);
    if app.scroll_offset > max_scroll {
        app.scroll_offset = max_scroll;
    }

    let text = Text::from(lines);
    let messages_widget = Paragraph::new(text)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(format!(" {} ", app.agent_name))
                .title_style(Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
        )
        .wrap(Wrap { trim: false })
        .scroll((app.scroll_offset as u16, 0));

    frame.render_widget(messages_widget, area);

    if total_lines > visible_height {
        let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight)
            .begin_symbol(Some("↑"))
            .end_symbol(Some("↓"));
        let mut scrollbar_state = ScrollbarState::new(total_lines).position(app.scroll_offset);
        frame.render_stateful_widget(
            scrollbar,
            area.inner(&ratatui::layout::Margin { vertical: 1, horizontal: 0 }),
            &mut scrollbar_state,
        );
    }
}

fn render_status(frame: &mut Frame, app: &TuiApp, area: Rect) {
    let status_style = if app.is_waiting {
        Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::Gray)
    };

    let status = Paragraph::new(format!(" {} ", app.status)).style(status_style);
    frame.render_widget(status, area);
}

fn render_input(frame: &mut Frame, app: &TuiApp, area: Rect) {
    let input_style = if app.is_waiting {
        Style::default().fg(Color::DarkGray)
    } else {
        Style::default().fg(Color::White)
    };

    let input = Paragraph::new(app.input.as_str()).style(input_style).block(
        Block::default()
            .borders(Borders::ALL)
            .title(if app.is_waiting {
                " Input (waiting...) "
            } else {
                " Input "
            })
            .title_style(Style::default().fg(Color::White)),
    );

    frame.render_widget(input, area);

    if !app.is_waiting {
        frame.set_cursor(area.x + 1 + app.input_cursor as u16, area.y + 1);
    }
}

// =============================================================================
// API COMMUNICATION
// =============================================================================

pub fn chat_with_agent(
    mut agent_state: AgentState,
    user_message: String,
    files_to_include: Option<Vec<String>>,
    event_tx: mpsc::UnboundedSender<TuiEvent>,
    tools: Vec<Tool>,
) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<AgentState, Box<dyn std::error::Error + Send + Sync>>> + Send>> {
    Box::pin(async move {
        let api_key = env::var("ANTHROPIC_API_KEY")
            .expect("ANTHROPIC_API_KEY environment variable not set");

        let _ = event_tx.send(TuiEvent::Thinking);

        let mut enhanced_message = user_message;

        if let Some(files) = files_to_include {
            if !files.is_empty() {
                let _ = event_tx.send(TuiEvent::ToolUse(format!("Reading {} files...", files.len())));

                let file_results = toolkit::read_files_parallel(
                    agent_state.working_directory.clone(),
                    files,
                )
                .await;

                let mut file_context = String::from("\n\n## File Context:");
                for (filename, result) in file_results {
                    match result {
                        Ok(content) => {
                            file_context.push_str(&format!(
                                "\n\n<file name='{}'>\n{}\n</file>",
                                filename, content
                            ));
                        }
                        Err(e) => {
                            let _ = event_tx.send(TuiEvent::Error(e));
                        }
                    }
                }
                enhanced_message.push_str(&file_context);
            }
        }

        agent_state.conversation_history.push(Message {
            role: "user".to_string(),
            content: MessageContent::Text(enhanced_message),
        });

        let request_body = ApiRequest {
            model: agent_state.model.clone(),
            max_tokens: 4096,
            system: agent_state.system_prompt.clone(),
            messages: agent_state.conversation_history.clone(),
            tools: if tools.is_empty() { None } else { Some(tools.clone()) },
        };

        let client = reqwest::Client::new();
        let response = client
            .post(API_URL)
            .header("x-api-key", api_key.clone())
            .header("anthropic-version", ANTHROPIC_VERSION)
            .header("content-type", "application/json")
            .json(&request_body)
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            let _ = event_tx.send(TuiEvent::Error(format!("API Error: {}", error_text)));
            return Err(format!("API Error: {}", error_text).into());
        }

        let api_response: ApiResponse = response.json().await?;

        agent_state.conversation_history.push(Message {
            role: "assistant".to_string(),
            content: MessageContent::Blocks(api_response.content.clone()),
        });

        let has_tool_use = api_response
            .content
            .iter()
            .any(|block| block.content_type == "tool_use");

        if has_tool_use {
            let mut tool_results = Vec::new();
            for block in &api_response.content {
                if block.content_type == "tool_use" {
                    let tool_name = block.name.as_ref().unwrap();
                    let tool_input = block.input.as_ref().unwrap();
                    let tool_id = block.id.as_ref().unwrap();

                    let _ = event_tx.send(TuiEvent::ToolUse(format!("Executing: {}", tool_name)));

                    let result = toolkit::execute_tool(tool_name, tool_input, &agent_state.working_directory);
                    let result_text = match result {
                        Ok(output) => {
                            let _ = event_tx.send(TuiEvent::ToolUse(format!("✓ {} complete", tool_name)));
                            output
                        }
                        Err(e) => {
                            let _ = event_tx.send(TuiEvent::Error(format!("Tool error: {}", e)));
                            format!("Error: {}", e)
                        }
                    };

                    tool_results.push(ContentBlock {
                        content_type: "tool_result".to_string(),
                        text: None,
                        id: None,
                        name: None,
                        input: None,
                        tool_use_id: Some(tool_id.clone()),
                        content: Some(result_text),
                    });
                }
            }

            agent_state.conversation_history.push(Message {
                role: "user".to_string(),
                content: MessageContent::Blocks(tool_results),
            });

            return chat_with_agent(agent_state, String::new(), None, event_tx, tools).await;
        }

        let mut response_text = String::new();
        for block in &api_response.content {
            if block.content_type == "text" {
                if let Some(text) = &block.text {
                    response_text.push_str(text);
                }
            }
        }

        let _ = event_tx.send(TuiEvent::Response(response_text));
        let _ = event_tx.send(TuiEvent::Done);

        Ok(agent_state)
    })
}

// =============================================================================
// CONVERSATION MANAGEMENT
// =============================================================================

pub fn save_conversation(
    agent_state: &AgentState,
    filename: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut conversation_text = format!("# {} Conversation Log\n\n", agent_state.agent_name);

    for msg in &agent_state.conversation_history {
        let content_str = match &msg.content {
            MessageContent::Text(text) => text.clone(),
            MessageContent::Blocks(blocks) => blocks
                .iter()
                .filter_map(|b| b.text.as_ref())
                .cloned()
                .collect::<Vec<_>>()
                .join("\n"),
        };

        conversation_text.push_str(&format!(
            "## {}\n\n{}\n\n---\n\n",
            msg.role.to_uppercase(),
            content_str
        ));
    }

    toolkit::save_to_history(&agent_state.working_directory, filename, &conversation_text)?;
    Ok(())
}

// =============================================================================
// FILE REFERENCE PARSING
// =============================================================================

pub fn extract_file_references(input: &str) -> Vec<String> {
    let re = regex::Regex::new(r"@([A-Za-z0-9_/.-]+\.(md|txt|json|rs))").unwrap();
    re.captures_iter(input)
        .map(|cap| cap[1].to_string())
        .collect()
}

pub fn remove_file_references(input: &str) -> String {
    let re = regex::Regex::new(r"@[A-Za-z0-9_/.-]+\.(md|txt|json|rs)").unwrap();
    re.replace_all(input, "").trim().to_string()
}

// =============================================================================
// INTERACTIVE TUI LOOP
// =============================================================================

pub async fn run_interactive(
    agent_state: AgentState,
    tools: Vec<Tool>,
) -> Result<(), Box<dyn std::error::Error>> {
    let agent = Arc::new(tokio::sync::Mutex::new(agent_state.clone()));

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = TuiApp::new(&agent_state.agent_name);
    app.add_message(ChatMessage::System(format!(
        "Model: {} | Use @filename.md to include files | Ctrl+C to quit",
        agent_state.model
    )));

    let (event_tx, mut event_rx) = mpsc::unbounded_channel::<TuiEvent>();

    let result: Result<(), Box<dyn std::error::Error>> = loop {
        terminal.draw(|f| render_tui(f, &mut app))?;

        if event::poll(std::time::Duration::from_millis(50))? {
            if let Event::Key(key) = event::read()? {
                if key.modifiers.contains(KeyModifiers::CONTROL) && key.code == KeyCode::Char('c') {
                    break Ok(());
                }

                if app.is_waiting {
                    continue;
                }

                match key.code {
                    KeyCode::Enter => {
                        let input = app.take_input();
                        if !input.is_empty() {
                            let files = extract_file_references(&input);
                            let clean_msg = remove_file_references(&input);

                            app.add_message(ChatMessage::User(input.clone()));
                            app.is_waiting = true;
                            app.status = "Thinking...".to_string();

                            let agent_clone = Arc::clone(&agent);
                            let tx = event_tx.clone();
                            let files_opt = if files.is_empty() { None } else { Some(files) };
                            let tools_clone = tools.clone();

                            tokio::spawn(async move {
                                let mut agent_guard = agent_clone.lock().await;
                                match chat_with_agent(
                                    agent_guard.clone(),
                                    clean_msg,
                                    files_opt,
                                    tx,
                                    tools_clone,
                                )
                                .await
                                {
                                    Ok(new_agent) => {
                                        *agent_guard = new_agent;
                                    }
                                    Err(_) => {}
                                }
                            });
                        }
                    }
                    KeyCode::Char(c) => app.insert_char(c),
                    KeyCode::Backspace => app.delete_char(),
                    KeyCode::Delete => app.delete_char_forward(),
                    KeyCode::Left => app.move_cursor_left(),
                    KeyCode::Right => app.move_cursor_right(),
                    KeyCode::Home => app.move_cursor_home(),
                    KeyCode::End => app.move_cursor_end(),
                    KeyCode::Up => app.scroll_up(3),
                    KeyCode::Down => app.scroll_down(3),
                    KeyCode::PageUp => app.scroll_up(10),
                    KeyCode::PageDown => app.scroll_down(10),
                    _ => {}
                }
            }
        }

        while let Ok(tui_event) = event_rx.try_recv() {
            match tui_event {
                TuiEvent::Response(text) => {
                    app.add_message(ChatMessage::Assistant(text));
                }
                TuiEvent::ToolUse(text) => {
                    app.status = format!("Tool: {}", text.chars().take(40).collect::<String>());
                    app.add_message(ChatMessage::ToolUse(text));
                }
                TuiEvent::Thinking => {
                    app.status = "Thinking...".to_string();
                }
                TuiEvent::Error(text) => {
                    app.add_message(ChatMessage::Error(text));
                }
                TuiEvent::Done => {
                    app.is_waiting = false;
                    app.status = "Ready".to_string();
                }
            }
        }
    };

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    let agent_guard = agent.lock().await;
    let _ = save_conversation(&agent_guard, "conversation_log.md");

    result
}
