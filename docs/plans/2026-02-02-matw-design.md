# MATW - Claude Code Rust 复刻设计文档

**日期**: 2026-02-02
**作者**: Claude Code Brainstorming Session
**状态**: 设计阶段

---

## 项目概述

MATW 是一个用 Rust 编写的 AI 编程助手，复刻 Claude Code 的核心功能。

### 核心目标

- 用 Rust 实现高性能、内存安全的 AI 编程工具
- 支持可插拔的 AI 提供商（Claude, OpenAI, Ollama, GLM, Kimi）
- 模块化架构，支持插件和 MCP 协议
- 终端优先的用户界面（ratatui）

### 第一阶段优先级

1. **终端对话** - 核心的交互界面和 AI 消息流
2. **代码操作** - 文件读取、编辑、搜索功能
3. **插件系统** - MCP 协议和 Hook 扩展机制

---

## 整体架构

### 分层架构图

```
┌─────────────────────────────────────────────────────────┐
│                    Presentation Layer                    │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐  │
│  │   TUI Views  │  │   Input      │  │   Status     │  │
│  │   (ratatui)  │  │   Handler    │  │   Bar        │  │
│  └──────────────┘  └──────────────┘  └──────────────┘  │
└─────────────────────────────────────────────────────────┘
                            ▼
┌─────────────────────────────────────────────────────────┐
│                     Application Layer                    │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐  │
│  │   Session    │  │   Agent      │  │   Plugin     │  │
│  │   Manager    │  │   Orchestr.. │  │   Manager    │  │
│  └──────────────┘  └──────────────┘  └──────────────┘  │
└─────────────────────────────────────────────────────────┘
                            ▼
┌─────────────────────────────────────────────────────────┐
│                       Domain Layer                       │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐  │
│  │   Message    │  │   Tool       │  │   Context    │  │
│  │   Bus        │  │   Executor   │  │   Manager    │  │
│  └──────────────┘  └──────────────┘  └──────────────┘  │
└─────────────────────────────────────────────────────────┘
                            ▼
┌─────────────────────────────────────────────────────────┐
│                    Infrastructure Layer                  │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐  │
│  │   AI         │  │   File       │  │   Git        │  │
│  │   Provider   │  │   System     │  │   Adapter    │  │
│  └──────────────┘  └──────────────┘  └──────────────┘  │
└─────────────────────────────────────────────────────────┘
```

### 各层职责

| 层级 | 职责 | 关键组件 |
|------|------|----------|
| **Presentation** | 用户界面渲染和输入处理 | ratatui 组件、事件循环 |
| **Application** | 业务逻辑和协调 | 会话管理、代理编排 |
| **Domain** | 核心领域模型 | 消息、工具、上下文 |
| **Infrastructure** | 外部系统集成 | AI、文件系统、Git |

---

## 项目目录结构

```
matw/
├── crates/
│   ├── matw-core/          # 核心领域模型
│   │   ├── src/
│   │   │   ├── message.rs
│   │   │   ├── session.rs
│   │   │   ├── context.rs
│   │   │   ├── tool.rs
│   │   │   └── error.rs
│   │   └── Cargo.toml
│   │
│   ├── matw-ui/            # ratatui UI 组件
│   │   ├── src/
│   │   │   ├── layout.rs
│   │   │   ├── messages.rs
│   │   │   ├── input.rs
│   │   │   ├── status.rs
│   │   │   ├── event.rs
│   │   │   └── main.rs
│   │   └── Cargo.toml
│   │
│   ├── matw-ai/            # AI 提供商抽象
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── provider.rs
│   │   │   ├── config.rs
│   │   │   └── providers/
│   │   │       ├── mod.rs
│   │   │       ├── claude.rs
│   │   │       ├── openai.rs
│   │   │       ├── ollama.rs
│   │   │       ├── glm.rs
│   │   │       └── kimi.rs
│   │   └── Cargo.toml
│   │
│   ├── matw-tools/         # 工具执行器
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── executor.rs
│   │   │   └── tools/
│   │   │       ├── mod.rs
│   │   │       ├── read.rs
│   │   │       ├── write.rs
│   │   │       ├── edit.rs
│   │   │       ├── glob.rs
│   │   │       ├── grep.rs
│   │   │       └── bash.rs
│   │   └── Cargo.toml
│   │
│   └── matw-plugins/       # 插件系统
│       ├── src/
│       │   ├── lib.rs
│       │   ├── manager.rs
│       │   ├── skill.rs
│       │   ├── hooks.rs
│       │   └── mcp/
│       │       ├── mod.rs
│       │       ├── server.rs
│       │       ├── stdio.rs
│       │       ├── http.rs
│       │       └── sse.rs
│       └── Cargo.toml
│
├── src/                    # CLI 入口
│   └── main.rs
├── tests/                  # 集成测试
│   └── integration_test.rs
├── docs/
│   └── plans/
├── Cargo.toml              # Workspace 配置
└── README.md
```

---

## 核心领域模型

### 消息结构

```rust
// matw-core/src/message.rs
use uuid::Uuid;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

pub struct Message {
    pub id: Uuid,
    pub role: Role,
    pub content: Content,
    pub timestamp: DateTime<Utc>,
    pub metadata: Metadata,
}

pub enum Role {
    User,
    Assistant,
    System,
    Tool,
}

pub enum Content {
    Text(String),
    ToolUse {
        id: String,
        name: String,
        input: serde_json::Value,
    },
    ToolResult {
        id: String,
        content: String,
        is_error: bool,
    },
}

pub type Metadata = HashMap<String, serde_json::Value>;
```

### 会话结构

```rust
// matw-core/src/session.rs
pub struct Session {
    pub id: Uuid,
    pub messages: Vec<Message>,
    pub context: Context,
    pub state: SessionState,
}

pub enum SessionState {
    Active,
    Paused,
    Closed,
}

pub struct Context {
    pub working_dir: PathBuf,
    pub git_info: Option<GitInfo>,
    pub environment: HashMap<String, String>,
    pub claude_md: Option<String>,  // CLAUDE.md 内容
}

pub struct GitInfo {
    pub branch: String,
    pub commit: String,
    pub root: PathBuf,
}
```

### 工具抽象

```rust
// matw-core/src/tool.rs
pub trait Tool: Send + Sync {
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    fn parameters_schema(&self) -> &serde_json::Value;
    fn execute(&self, input: serde_json::Value) -> Result<ToolOutput, ToolError>;
}

pub struct ToolOutput {
    pub content: String,
    pub is_error: bool,
}

#[derive(Debug, thiserror::Error)]
pub enum ToolError {
    #[error("Invalid parameters: {0}")]
    InvalidParameters(String),
    #[error("Execution failed: {0}")]
    ExecutionFailed(String),
    #[error("IO error: {0}")]
    IO(#[from] io::Error),
}
```

---

## AI 提供商抽象

### Provider Trait

```rust
// matw-ai/src/lib.rs
use futures::Stream;
use pin_project::pin_project;

#[async_trait]
pub trait AIProvider: Send + Sync {
    fn name(&self) -> &str;

    async fn stream_completion(
        &self,
        request: CompletionRequest,
    ) -> Result<Pin<Box<dyn Stream<Item = Result<Chunk, AIError>> + Send>>, AIError>;

    async fn complete(
        &self,
        request: CompletionRequest,
    ) -> Result<CompletionResponse, AIError>;
}

pub struct CompletionRequest {
    pub messages: Vec<Message>,
    pub tools: Vec<ToolDefinition>,
    pub model: String,
    pub max_tokens: Option<u32>,
    pub temperature: Option<f32>,
    pub system_prompt: Option<String>,
}

pub struct CompletionResponse {
    pub content: String,
    pub tool_uses: Vec<ToolUse>,
    pub stop_reason: StopReason,
    pub usage: Usage,
}

pub enum Chunk {
    Delta(String),
    ToolUse(ToolUse),
    Done,
}
```

### 支持的提供商

```rust
// matw-ai/src/providers/claude.rs
pub struct ClaudeProvider {
    api_key: String,
    client: reqwest::Client,
    base_url: String,
}

// matw-ai/src/providers/openai.rs
pub struct OpenAIProvider {
    api_key: String,
    client: reqwest::Client,
    base_url: String,
}

// matw-ai/src/providers/ollama.rs
pub struct OllamaProvider {
    base_url: String,
    client: reqwest::Client,
}

// matw-ai/src/providers/glm.rs
pub struct GLMProvider {
    api_key: String,
    client: reqwest::Client,
    base_url: String,  // https://open.bigmodel.cn/api/paas/v4/
}

// matw-ai/src/providers/kimi.rs
pub struct KimiProvider {
    api_key: String,
    client: reqwest::Client,
    base_url: String,  // https://api.moonshot.cn/v1
}
```

### 配置文件

```toml
# ~/.config/matw/config.toml
[default_provider]
name = "glm"

[providers.glm]
type = "glm"
api_key = "your-glm-api-key"
base_url = "https://open.bigmodel.cn/api/paas/v4/"
model = "glm-4-plus"

[providers.kimi]
type = "kimi"
api_key = "your-kimi-api-key"
base_url = "https://api.moonshot.cn/v1"
model = "moonshot-v1-128k"

[providers.claude]
type = "claude"
api_key = "your-claude-api-key"
model = "claude-sonnet-4-20250514"

[providers.openai]
type = "openai"
api_key = "your-openai-api-key"
base_url = "https://api.openai.com/v1"
model = "gpt-4o"

[providers.ollama]
type = "ollama"
base_url = "http://localhost:11434"
model = "llama3.2"
```

---

## 插件系统

### MCP 协议支持

```rust
// matw-plugins/src/mcp/server.rs
#[async_trait]
pub trait MCPServer: Send + Sync {
    fn name(&self) -> &str;
    async fn start(&mut self) -> Result<(), MCPError>;
    async fn stop(&mut self) -> Result<(), MCPError>;
    async fn call_tool(&self, name: &str, args: serde_json::Value) -> Result<serde_json::Value, MCPError>;
    async fn list_tools(&self) -> Result<Vec<ToolDefinition>, MCPError>;
}

// stdio 服务器
pub struct StdioServer {
    name: String,
    command: String,
    args: Vec<String>,
    child: Option<Child>,
}

// HTTP 服务器
pub struct HttpServer {
    name: String,
    url: String,
    client: reqwest::Client,
}

// SSE 服务器
pub struct SSEServer {
    name: String,
    url: String,
    client: reqwest::Client,
}
```

### 插件配置

```toml
# ~/.config/matw/plugins.toml
[[plugins]]
name = "filesystem"
type = "stdio"
command = "npx"
args = ["-y", "@modelcontextprotocol/server-filesystem", "$PROJECT_DIR"]

[[plugins]]
name = "github"
type = "sse"
url = "https://mcp.github.com/sse"

[[plugins]]
name = "custom-tools"
type = "http"
url = "https://api.example.com/mcp"
headers = { Authorization = "Bearer $API_TOKEN" }
```

### Hook 系统

```rust
// matw-plugins/src/hooks.rs
#[async_trait]
pub trait Hook: Send + Sync {
    async fn execute(&self, context: &HookContext) -> Result<HookOutput, HookError>;
}

pub enum HookEvent {
    SessionStart,
    PreToolUse { tool: String, input: serde_json::Value },
    PostToolUse { tool: String, output: Result<String, ToolError> },
    SessionEnd,
}

pub struct HookContext {
    pub session_id: Uuid,
    pub working_dir: PathBuf,
    pub environment: HashMap<String, String>,
}
```

### 技能定义

```rust
// matw-plugins/src/skill.rs
pub struct Skill {
    pub name: String,
    pub description: String,
    pub content: String,  // Markdown 格式
    pub permissions: Vec<Permission>,
    pub hooks: Vec<HookConfig>,
}

#[derive(Debug, Clone)]
pub enum Permission {
    Bash { pattern: String },
    FileRead { pattern: String },
    FileWrite { pattern: String },
    Network { url: String },
}
```

---

## TUI 界面

### 界面布局

```
┌──────────────────────────────────────────────────────────────┐
│  matw - session-name                    [✓ branch] [●●○]    │  ← Header
├──────────────────────────────────────────────────────────────┤
│                                                              │
│  User: 帮我重构这个函数                                        │
│                                                              │
│  Assistant: 我来帮你重构这个函数...                             │
│                                                              │
│  ┌─────────────────────────────────────────────────────┐   │
│  │ fn example() {                                      │   │
│  │     let x = 1;                                      │   │
│  │ }                                                   │   │
│  └─────────────────────────────────────────────────────┘   │
│                                                              │
│  [Reading file...]                               █          │  ← Status
├──────────────────────────────────────────────────────────────┤
│> 输入消息...                                    Alt+Enter 发送│  ← Input
└──────────────────────────────────────────────────────────────┘
```

### 核心组件

```rust
// matw-ui/src/layout.rs
pub struct AppLayout {
    header: HeaderWidget,
    messages: MessagesWidget,
    status: StatusWidget,
    input: InputWidget,
}

// matw-ui/src/messages.rs
pub struct MessagesWidget {
    messages: Vec<RenderedMessage>,
    scroll_offset: usize,
}

pub enum RenderedMessage {
    User { content: String, timestamp: DateTime<Utc> },
    Assistant { content: String, timestamp: DateTime<Utc> },
    ToolUse { tool: String, params: String },
    ToolResult { tool: String, output: String, is_error: bool },
}

// matw-ui/src/input.rs
pub struct InputWidget {
    content: String,
    cursor: usize,
    history: Vec<String>,
    history_index: usize,
    mode: InputMode,
}

pub enum InputMode {
    Normal,
    Insert,
    Multiline,
}

// matw-ui/src/status.rs
pub struct StatusWidget {
    tasks: Vec<TaskStatus>,
    git_status: Option<GitStatus>,
    model_name: String,
}
```

### 事件循环

```rust
// matw-ui/src/event.rs
use tokio::sync::mpsc;
use crossterm::event::{KeyEvent, event as crossterm_event};

pub enum UIEvent {
    Input(KeyEvent),
    Paste(String),
    Tick,
    MessageChunk(String),
    ToolExecution { tool: String, status: ExecutionStatus },
}

pub async fn run_ui(
    tx: mpsc::Sender<UIEvent>,
    rx: mpsc::Receiver<AppEvent>,
) -> Result<(), io::Error> {
    let mut terminal = ratatui::init();

    loop {
        select! {
            // 处理键盘输入
            event = crossterm::event::read()? => {
                handle_input(event, &tx).await?;
            }
            // 处理应用事件
            app_event = rx.recv() => {
                match app_event {
                    Some(AppEvent::MessageChunk(chunk)) => {
                        // 更新显示
                    }
                    Some(AppEvent::Quit) => break,
                    _ => {}
                }
            }
        }
        terminal.draw(|f| render(f, &mut app))?;
    }

    ratatui::restore();
    Ok(())
}
```

---

## 错误处理

### 错误类型体系

```rust
// matw-core/src/error.rs
#[derive(Debug, thiserror::Error)]
pub enum MatwError {
    #[error("AI provider error: {0}")]
    AI(#[from] AIError),

    #[error("Tool execution error: {0}")]
    Tool(#[from] ToolError),

    #[error("IO error: {0}")]
    IO(#[from] io::Error),

    #[error("Plugin error: {0}")]
    Plugin(#[from] PluginError),

    #[error("Session not found: {0}")]
    SessionNotFound(Uuid),

    #[error("Configuration error: {0}")]
    Config(String),
}

pub type Result<T> = std::result::Result<T, MatwError>;

// matw-ai/src/error.rs
#[derive(Debug, thiserror::Error)]
pub enum AIError {
    #[error("API request failed: {0}")]
    RequestFailed(#[from] reqwest::Error),

    #[error("API returned error: {code} - {message}")]
    APIError { code: String, message: String },

    #[error("Invalid response format")]
    InvalidResponse,

    #[error("Provider not configured: {0}")]
    NotConfigured(String),

    #[error("Stream interrupted")]
    StreamInterrupted,
}

// matw-plugins/src/error.rs
#[derive(Debug, thiserror::Error)]
pub enum PluginError {
    #[error("MCP server error: {0}")]
    MCPServer(String),

    #[error("Hook execution failed: {0}")]
    HookExecution(String),

    #[error("Skill not found: {0}")]
    SkillNotFound(String),

    #[error("Invalid plugin configuration: {0}")]
    InvalidConfig(String),
}
```

---

## 测试策略

### 单元测试

```rust
// matw-core/src/message/tests.rs
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_message_creation() {
        let msg = Message::new_user("hello".to_string());
        assert_eq!(msg.role(), Role::User);
    }

    #[test]
    fn test_tool_use_parsing() {
        let content = r#"<tool_use>
{"name": "read", "input": {"path": "test.rs"}}
</tool_use>"#;
        assert!(Message::parse_tool_use(content).is_ok());
    }
}

// matw-tools/src/read/tests.rs
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_read_file() {
        let temp = tempfile::tempdir().unwrap();
        let file_path = temp.path().join("test.txt");
        fs::write(&file_path, "hello world").unwrap();

        let tool = ReadTool::new();
        let result = tool.execute(json!({"path": file_path})).await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap().content, "hello world");
    }
}
```

### Mock Provider

```rust
// matw-ai/src/mocking.rs
#[cfg(test)]
pub struct MockAIProvider {
    responses: Vec<String>,
}

#[async_trait]
impl AIProvider for MockAIProvider {
    async fn stream_completion(
        &self,
        _request: CompletionRequest,
    ) -> Result<Pin<Box<dyn Stream<Item = Result<Chunk, AIError>> + Send>>, AIError> {
        // 返回预设的模拟响应
    }
}
```

### 集成测试

```rust
// tests/integration_test.rs
#[tokio::test]
async fn test_full_workflow() {
    // 1. 创建会话
    let session = Session::new(tempdir.path());

    // 2. 添加用户消息
    session.add_message(Message::new_user("帮我创建一个 hello world".to_string()));

    // 3. 调用 AI
    let provider = MockAIProvider::new();
    let response = provider.complete(session.to_request()).await.unwrap();

    // 4. 执行工具调用
    for tool_use in response.tool_uses {
        let result = execute_tool(tool_use).await;
        session.add_result(result);
    }

    // 5. 验证结果
    assert!(session.has_tool_result("write"));
}
```

---

## 依赖项

### Cargo.toml Workspace

```toml
[workspace]
members = [
    "crates/matw-core",
    "crates/matw-ui",
    "crates/matw-ai",
    "crates/matw-tools",
    "crates/matw-plugins",
]
resolver = "2"

[workspace.dependencies]
# 异步运行时
tokio = { version = "1.40", features = ["full"] }
futures = "0.3"

# 序列化
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"

# 错误处理
thiserror = "2.0"
anyhow = "1.0"

# HTTP 客户端
reqwest = { version = "0.12", features = ["json", "stream"] }

# TUI
ratatui = "0.29"
crossterm = "0.28"

# UUID 和时间
uuid = { version = "1.10", features = ["v4", "serde"] }
chrono = { version = "0.4", features = ["serde"] }

# 文件系统
walkdir = "2.5"
ignore = "0.4"

# Git
git2 = "0.19"

# 测试
tempfile = "3.13"
```

---

## 实施路线图

### Phase 1: 基础设施 (Week 1-2)

- [ ] 设置 Workspace 和 Cargo 项目结构
- [ ] 实现核心领域模型 (Message, Session, Context)
- [ ] 实现错误处理体系
- [ ] 编写基础单元测试

### Phase 2: AI 集成 (Week 2-3)

- [ ] 实现 AIProvider trait
- [ ] 实现 Claude Provider
- [ ] 实现 GLM Provider
- [ ] 实现 Kimi Provider
- [ ] 实现 OpenAI Provider
- [ ] 实现 Ollama Provider
- [ ] 配置系统

### Phase 3: 工具系统 (Week 3-4)

- [ ] 实现 Tool trait
- [ ] 实现 Read 工具
- [ ] 实现 Write 工具
- [ ] 实现 Edit 工具
- [ ] 实现 Glob 工具
- [ ] 实现 Grep 工具
- [ ] 实现 Bash 工具

### Phase 4: TUI 界面 (Week 4-5)

- [ ] 实现 ratatui 布局
- [ ] 实现消息显示组件
- [ ] 实现输入处理
- [ ] 实现状态栏
- [ ] 事件循环

### Phase 5: 插件系统 (Week 5-6)

- [ ] MCP 协议实现
- [ ] Stdio/HTTP/SSE 服务器
- [ ] Hook 系统
- [ ] 技能定义和加载

### Phase 6: 集成与测试 (Week 6-7)

- [ ] 集成测试
- [ ] 端到端测试
- [ ] 性能优化
- [ ] 文档完善

---

## 设计决策记录

### ADR-001: 使用 Rust 实现

**原因**:
- 内存安全，无 GC 开销
- 高性能异步运行时 (tokio)
- 强类型系统，减少运行时错误
- 优秀的包管理 (Cargo)

**权衡**:
- 开发速度可能比动态语言慢
- 编译时间较长

### ADR-002: 可插拔 AI 架构

**原因**:
- 不依赖单一 AI 提供商
- 用户可根据需求选择
- 支持本地模型 (Ollama)

**实现**:
- trait-based 抽象
- 配置驱动的提供商选择

### ADR-003: ratatui 作为 TUI 框架

**原因**:
- Rust 生态最成熟的 TUI 库
- 跨平台支持
- 活跃的社区维护

### ADR-004: Workspace 结构

**原因**:
- 清晰的关注点分离
- 独立的 crate 可以独立发布
- 更好的编译时依赖管理

---

## 参考资料

- [Claude Code 官方文档](https://code.claude.com/docs/en/overview)
- [MCP 协议规范](https://modelcontextprotocol.io/)
- [ratatui 文档](https://ratatui.rs/)
- [tokio 异步运行时](https://tokio.rs/)
