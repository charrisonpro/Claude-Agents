// agent.rs - TUI interface and Claude API communication for PM Helper

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
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame, Terminal,
};
use reqwest;
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc;

const API_URL: &str = "https://api.anthropic.com/v1/messages";
const ANTHROPIC_VERSION: &str = "2023-06-01";
const DEFAULT_MODEL: &str = "claude-haiku-4-20250514";
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
        .split(frame.area());

    // Messages
    let mut lines: Vec<Line> = Vec::new();
    for msg in &app.messages {
        let (prefix, style, content) = match msg {
            ChatMessage::User(t) => ("You: ", Style::default().fg(Color::Cyan), t.as_str()),
            ChatMessage::Assistant(t) => ("PM: ", Style::default().fg(Color::Green), t.as_str()),
            ChatMessage::System(t) => (">> ", Style::default().fg(Color::Yellow), t.as_str()),
            ChatMessage::ToolUse(t) => ("[] ", Style::default().fg(Color::Magenta), t.as_str()),
            ChatMessage::Error(t) => ("!! ", Style::default().fg(Color::Red), t.as_str()),
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
    if app.scroll_offset > max_scroll {
        app.scroll_offset = max_scroll;
    }

    let msg_widget = Paragraph::new(Text::from(lines))
        .block(Block::default().borders(Borders::ALL).title(format!(" {} ", app.agent_name)))
        .wrap(Wrap { trim: false })
        .scroll((app.scroll_offset as u16, 0));
    frame.render_widget(msg_widget, chunks[0]);

    // Status
    let status = Paragraph::new(format!(" {} ", app.status))
        .style(if app.is_waiting {
            Style::default().fg(Color::Yellow)
        } else {
            Style::default().fg(Color::Gray)
        });
    frame.render_widget(status, chunks[1]);

    // Input
    let input = Paragraph::new(app.input.as_str())
        .block(Block::default().borders(Borders::ALL).title(" Input "));
    frame.render_widget(input, chunks[2]);
    if !app.is_waiting {
        frame.set_cursor_position((chunks[2].x + 1 + app.input_cursor as u16, chunks[2].y + 1));
    }
}

pub fn chat_with_agent(
    mut state: AgentState,
    msg: String,
    tx: mpsc::UnboundedSender<TuiEvent>,
    tools: Vec<Tool>,
) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<AgentState, Box<dyn std::error::Error + Send + Sync>>> + Send>>
{
    Box::pin(async move {
        let api_key = env::var("ANTHROPIC_API_KEY")?;
        let _ = tx.send(TuiEvent::Thinking);

        if !msg.is_empty() {
            state.conversation_history.push(Message {
                role: "user".to_string(),
                content: MessageContent::Text(msg),
            });
        }

        let req = ApiRequest {
            model: state.model.clone(),
            max_tokens: 2048,
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
        state.conversation_history.push(Message {
            role: "assistant".to_string(),
            content: MessageContent::Blocks(api_resp.content.clone()),
        });

        let has_tools = api_resp.content.iter().any(|b| b.content_type == "tool_use");
        if has_tools {
            let mut results = Vec::new();
            for block in &api_resp.content {
                if block.content_type == "tool_use" {
                    let name = block.name.as_ref().unwrap();
                    let input = block.input.as_ref().unwrap();
                    let id = block.id.as_ref().unwrap();
                    let _ = tx.send(TuiEvent::ToolUse(format!("{}", name)));
                    let result = toolkit::execute_tool(name, input, &state.working_directory)
                        .unwrap_or_else(|e| format!("Error: {}", e));
                    results.push(ContentBlock {
                        content_type: "tool_result".to_string(),
                        tool_use_id: Some(id.clone()),
                        content: Some(result),
                        text: None,
                        id: None,
                        name: None,
                        input: None,
                    });
                }
            }
            state.conversation_history.push(Message {
                role: "user".to_string(),
                content: MessageContent::Blocks(results),
            });
            return chat_with_agent(state, String::new(), tx, tools).await;
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
    app.add_message(ChatMessage::System(format!("Model: {} | Type 'process' to run queue", state.model)));

    let (tx, mut rx) = mpsc::unbounded_channel::<TuiEvent>();

    loop {
        terminal.draw(|f| render_tui(f, &mut app))?;

        if event::poll(std::time::Duration::from_millis(50))? {
            if let Event::Key(key) = event::read()? {
                if key.modifiers.contains(KeyModifiers::CONTROL) && key.code == KeyCode::Char('c') {
                    break;
                }
                if app.is_waiting {
                    continue;
                }
                match key.code {
                    KeyCode::Enter => {
                        let input = app.take_input();
                        if !input.is_empty() {
                            app.add_message(ChatMessage::User(input.clone()));
                            app.is_waiting = true;
                            app.status = "Processing...".to_string();
                            let agent_clone = Arc::clone(&agent);
                            let tx_clone = tx.clone();
                            let tools_clone = tools.clone();
                            tokio::spawn(async move {
                                let mut guard = agent_clone.lock().await;
                                if let Ok(new) = chat_with_agent(guard.clone(), input, tx_clone, tools_clone).await {
                                    *guard = new;
                                }
                            });
                        }
                    }
                    KeyCode::Char(c) => {
                        app.input.insert(app.input_cursor, c);
                        app.input_cursor += 1;
                    }
                    KeyCode::Backspace if app.input_cursor > 0 => {
                        app.input_cursor -= 1;
                        app.input.remove(app.input_cursor);
                    }
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
                TuiEvent::ToolUse(t) => {
                    app.status = t.clone();
                    app.add_message(ChatMessage::ToolUse(t));
                }
                TuiEvent::Thinking => app.status = "Processing...".to_string(),
                TuiEvent::Error(t) => app.add_message(ChatMessage::Error(t)),
                TuiEvent::Done => {
                    app.is_waiting = false;
                    app.status = "Ready".to_string();
                }
            }
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen, DisableMouseCapture)?;
    Ok(())
}
