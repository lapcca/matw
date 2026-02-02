# MATW - AI-powered Coding Assistant in Rust

[![Rust](https://img.shields.io/badge/rust-1.75%2B-orange.svg)](https://www.rust-lang.org)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

MATW (Mate) is a terminal-based AI coding assistant written in Rust, featuring a rich TUI (Terminal User Interface), pluggable AI providers, and an extensible tool system.

## Features

- **🤖 Multi-Provider AI Support**: Works with Claude, GLM, Kimi, and OpenAI-compatible APIs
- **🛠️ Built-in Tools**: Read, Write, Glob, and Bash command execution
- **🔌 MCP Plugin System**: Model Context Protocol support for external tool integration
- **📱 Rich TUI**: Beautiful terminal interface powered by [ratatui](https://github.com/ratatui-org/ratatui)
- **💬 Session Management**: Persistent conversation sessions with context
- **⚡ Async Architecture**: Built on Tokio for high-performance async operations

## Architecture

MATW uses a layered architecture with clear separation of concerns:

```
┌─────────────────────────────────────────┐
│  Presentation Layer (matw-tui)          │
│  - Terminal UI with ratatui             │
│  - Event handling and rendering         │
├─────────────────────────────────────────┤
│  Application Layer (matw-agent)         │
│  - Agent orchestration loop             │
│  - Streaming response handling          │
├─────────────────────────────────────────┤
│  Domain Layer (matw-core)               │
│  - Message, Session, Context            │
│  - Role and Content types               │
├─────────────────────────────────────────┤
│  Infrastructure Layer                   │
│  - matw-ai: AI provider abstraction     │
│  - matw-tools: Tool implementations     │
│  - matw-mcp: MCP plugin system          │
└─────────────────────────────────────────┘
```

## Project Structure

```
matw/
├── crates/
│   ├── matw-core/      # Core domain types (Message, Session, Context)
│   ├── matw-ai/        # AI provider abstraction and implementations
│   ├── matw-tools/     # Built-in tools (read, write, glob, bash)
│   ├── matw-tui/       # Terminal user interface
│   ├── matw-mcp/       # Model Context Protocol support
│   ├── matw-agent/     # Agent orchestration
│   └── matw-cli/       # Command-line interface
├── Cargo.toml          # Workspace configuration
└── README.md           # This file
```

## Installation

### Prerequisites

- Rust 1.75 or higher
- Git (for repository context detection)

### Build from Source

```bash
# Clone the repository
git clone https://github.com/lapcca/mate.git
cd mate

# Build release version
cargo build --release

# The binary will be at target/release/matw
```

### Development Build

```bash
cargo build
```

## Configuration

MATW looks for configuration files in the following locations:

- Linux/macOS: `~/.config/matw/config.toml`
- Windows: `%APPDATA%\matw\config.toml`

### Example Configuration

```toml
# Default AI provider
provider = "glm"
model = "glm-4-plus"
api_key = "your-api-key-here"

# Optional: Custom base URL
# base_url = "https://custom.api.endpoint.com/v1"
```

### Provider-Specific Configuration

#### GLM (Zhipu AI)
```toml
provider = "glm"
model = "glm-4-plus"
api_key = "your-glm-api-key"
```

#### Kimi (Moonshot)
```toml
provider = "kimi"
model = "moonshot-v1-8k"
api_key = "your-kimi-api-key"
```

#### Claude (Anthropic)
```toml
provider = "claude"
model = "claude-sonnet-4-20250514"
api_key = "your-anthropic-api-key"
```

## Usage

### Interactive TUI Mode

```bash
# Start interactive session
cargo run --release

# Or if installed to PATH
matw
```

**TUI Shortcuts:**
- Type your message and press `Enter` to send
- `Backspace` to delete characters
- `Esc` or `q` (when input is empty) to quit

### Simple Mode

```bash
# Display session info without TUI
matw --simple
```

### Command Line Options

```bash
matw [OPTIONS]

Options:
  -d, --dir <DIR>          Working directory (defaults to current)
      --provider <PROVIDER>  AI provider to use
      --model <MODEL>        Model to use
      --api-key <API_KEY>    API key (overrides config)
  -c, --config <CONFIG>    Configuration file path
      --simple             Run in simple mode (without TUI)
  -h, --help               Print help
  -V, --version            Print version
```

### Examples

```bash
# Use specific provider
matw --provider glm --model glm-4-plus

# Use custom working directory
matw --dir /path/to/project

# Override API key
matw --api-key sk-...
```

## Built-in Tools

MATW comes with several built-in tools for code assistance:

### Read Tool
Reads file contents.
```json
{
  "name": "read",
  "description": "Read the contents of a file",
  "parameters": {
    "path": "path/to/file"
  }
}
```

### Write Tool
Writes content to files (creates directories if needed).
```json
{
  "name": "write",
  "description": "Write content to a file",
  "parameters": {
    "path": "path/to/file",
    "content": "file content"
  }
}
```

### Glob Tool
Finds files matching patterns (gitignore-aware).
```json
{
  "name": "glob",
  "description": "Find files matching a glob pattern",
  "parameters": {
    "pattern": "**/*.rs",
    "path": "."
  }
}
```

### Bash Tool
Executes shell commands.
```json
{
  "name": "bash",
  "description": "Execute a shell command",
  "parameters": {
    "command": "ls -la",
    "timeout": 120000
  }
}
```

## MCP (Model Context Protocol)

MATW supports MCP for extending functionality with external tools.

### Registering MCP Tools

```rust
use matw_mcp::{MCPServer, register_tools};
use matw_tools::all_tools;

let server = MCPServer::new();
let tools = all_tools();

// Register matw-tools as MCP tools
register_tools(&server, tools).await;
```

### MCP Protocol Methods

- `tools/list` - List all available tools
- `tools/call` - Execute a tool with given arguments

## Development

### Running Tests

```bash
# Run all tests
cargo test

# Run tests for specific crate
cargo test -p matw-core
cargo test -p matw-agent
```

### Project Structure

Each crate has a specific responsibility:

| Crate | Description |
|-------|-------------|
| `matw-core` | Domain types: Message, Session, Context, Role, Content |
| `matw-ai` | AI provider trait and implementations (Claude, GLM, Kimi) |
| `matw-tools` | Tool trait and built-in tools (read, write, glob, bash) |
| `matw-tui` | Terminal UI with ratatui |
| `matw-mcp` | MCP protocol implementation and server |
| `matw-agent` | Agent orchestration loop |
| `matw-cli` | Command-line interface |

### Adding a New AI Provider

1. Create a new file in `crates/matw-ai/src/providers/`
2. Implement the `AIProvider` trait
3. Export the provider in `crates/matw-ai/src/providers/mod.rs`
4. Add to `crates/matw-ai/src/lib.rs` exports

Example:
```rust
use matw_ai::{AIProvider, CompletionRequest, CompletionResponse};

pub struct MyProvider;

#[async_trait]
impl AIProvider for MyProvider {
    fn name(&self) -> &str { "my-provider" }

    async fn complete(&self, request: CompletionRequest) -> Result<CompletionResponse, AIError> {
        // Implementation
    }

    async fn stream_completion(&self, request: CompletionRequest) -> Result<ChunkStream, AIError> {
        // Implementation
    }
}
```

### Adding a New Tool

1. Create a new file in `crates/matw-tools/src/tools/`
2. Implement the `Tool` trait
3. Add to `crates/matw-tools/src/tools/mod.rs`
4. Export from `crates/matw-tools/src/lib.rs`

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Acknowledgments

- Built with [ratatui](https://github.com/ratatui-org/ratatui) for the terminal UI
- Async runtime powered by [Tokio](https://tokio.rs)
- Inspired by Claude Code and other AI coding assistants

## Roadmap

- [x] Core domain models
- [x] AI provider abstraction
- [x] Built-in tools (read, write, glob, bash)
- [x] TUI with ratatui
- [x] MCP plugin system
- [x] Agent orchestration
- [ ] File watching and auto-reload
- [ ] Plugin marketplace
- [ ] Multi-session support
- [ ] Configuration UI
