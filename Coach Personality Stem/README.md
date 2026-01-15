# Coach Personality Stem

A foundational coaching personality prompt focused on trust-building and open communication, designed to be specialized into different domains

## Setup

1. Ensure you have Rust installed
2. Set your API key: `export ANTHROPIC_API_KEY=your-key`
3. Build: `cargo build --release`
4. Run: `cargo run --release`

## Directory Structure

```
coach__personality__stem/
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
