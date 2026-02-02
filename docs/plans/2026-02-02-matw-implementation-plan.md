# MATW Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Build a Rust-based AI coding assistant with terminal UI, pluggable AI providers, and plugin system

**Architecture:** Layered architecture with Presentation (ratatui TUI), Application (session/agent management), Domain (messages/tools/context), and Infrastructure (AI/files/git) layers. Workspace structure with 5 crates for clear separation of concerns.

**Tech Stack:** Rust 1.75+, tokio async runtime, ratatui TUI, reqwest HTTP, serde JSON, thiserror error handling

---

## Phase 1: Project Setup and Core Domain Models

### Task 1: Initialize Cargo Workspace

**Files:**
- Create: `Cargo.toml`
- Create: `crates/matw-core/Cargo.toml`
- Create: `crates/matw-core/src/lib.rs`

**Step 1: Create workspace Cargo.toml**

```toml
[workspace]
members = [
    "crates/matw-core",
]
resolver = "2"

[workspace.dependencies]
# Async runtime
tokio = { version = "1.40", features = ["full"] }
futures = "0.3"

# Serialization
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"

# Error handling
thiserror = "2.0"
anyhow = "1.0"

# UUID and time
uuid = { version = "1.10", features = ["v4", "serde"] }
chrono = { version = "0.4", features = ["serde"] }

# Testing
tempfile = "3.13"
```

**Step 2: Create matw-core package**

```toml
# crates/matw-core/Cargo.toml
[package]
name = "matw-core"
version = "0.1.0"
edition = "2021"

[dependencies]
tokio = { workspace = true }
serde = { workspace = true }
serde_json = { workspace = true }
thiserror = { workspace = true }
anyhow = { workspace = true }
uuid = { workspace = true }
chrono = { workspace = true }

[dev-dependencies]
tempfile = { workspace = true }
```

**Step 3: Create lib.rs with module declarations**

```rust
// crates/matw-core/src/lib.rs
pub mod message;
pub mod role;
pub mod content;
pub mod error;

pub use message::Message;
pub use role::Role;
pub use content::Content;
pub use error::{MatwError, Result};
```

**Step 4: Run cargo check**

Run: `cargo check`
Expected: OK (no errors)

**Step 5: Commit**

```bash
git add Cargo.toml crates/matw-core/
git commit -m "feat: initialize workspace and matw-core crate"
```

---

### Task 2: Implement Role Enum

**Files:**
- Create: `crates/matw-core/src/role.rs`
- Create: `crates/matw-core/src/role/tests.rs`

**Step 1: Write the failing test**

```rust
// crates/matw-core/src/role/tests.rs
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_role_serialization() {
        let role = Role::User;
        let json = serde_json::to_string(&role).unwrap();
        assert_eq!(json, "\"user\"");
    }

    #[test]
    fn test_role_deserialization() {
        let role: Role = serde_json::from_str("\"assistant\"").unwrap();
        assert_eq!(role, Role::Assistant);
    }

    #[test]
    fn test_all_roles() {
        assert_eq!(Role::User.to_string(), "user");
        assert_eq!(Role::Assistant.to_string(), "assistant");
        assert_eq!(Role::System.to_string(), "system");
        assert_eq!(Role::Tool.to_string(), "tool");
    }
}
```

**Step 2: Run test to verify it fails**

Run: `cargo test -p matw-core role`
Expected: COMPILER ERROR - Role not defined

**Step 3: Write minimal implementation**

```rust
// crates/matw-core/src/role.rs
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    User,
    Assistant,
    System,
    Tool,
}

impl fmt::Display for Role {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Role::User => write!(f, "user"),
            Role::Assistant => write!(f, "assistant"),
            Role::System => write!(f, "system"),
            Role::Tool => write!(f, "tool"),
        }
    }
}

#[cfg(test)]
mod tests;
```

**Step 4: Run test to verify it passes**

Run: `cargo test -p matw-core role`
Expected: PASS (all 3 tests pass)

**Step 5: Update lib.rs**

```rust
// crates/matw-core/src/lib.rs
pub mod role;

pub use role::Role;
```

**Step 6: Commit**

```bash
git add crates/matw-core/src/role.rs
git commit -m "feat: add Role enum with serialization"
```

---

### Task 3: Implement Content Enum

**Files:**
- Create: `crates/matw-core/src/content.rs`
- Create: `crates/matw-core/src/content/tests.rs`

**Step 1: Write the failing test**

```rust
// crates/matw-core/src/content/tests.rs
#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_text_content() {
        let content = Content::Text("Hello".to_string());
        assert_eq!(content.as_str(), Some("Hello"));
    }

    #[test]
    fn test_tool_use_content() {
        let tool_use = Content::ToolUse {
            id: "call_123".to_string(),
            name: "read".to_string(),
            input: json!({"path": "test.rs"}),
        };
        assert_eq!(tool_use.tool_name(), Some("read"));
    }

    #[test]
    fn test_tool_result_content() {
        let result = Content::ToolResult {
            id: "call_123".to_string(),
            content: "file content".to_string(),
            is_error: false,
        };
        assert_eq!(result.as_str(), Some("file content"));
        assert!(!result.is_error());
    }
}
```

**Step 2: Run test to verify it fails**

Run: `cargo test -p matw-core content`
Expected: COMPILER ERROR - Content not defined

**Step 3: Write minimal implementation**

```rust
// crates/matw-core/src/content.rs
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Content {
    Text(String),
    ToolUse {
        id: String,
        name: String,
        input: JsonValue,
    },
    ToolResult {
        id: String,
        content: String,
        is_error: bool,
    },
}

impl Content {
    pub fn as_str(&self) -> Option<&str> {
        match self {
            Content::Text(s) => Some(s),
            Content::ToolResult { content, .. } => Some(content),
            _ => None,
        }
    }

    pub fn tool_name(&self) -> Option<&str> {
        match self {
            Content::ToolUse { name, .. } => Some(name),
            _ => None,
        }
    }

    pub fn is_error(&self) -> bool {
        matches!(self, Content::ToolResult { is_error: true, .. })
    }
}

#[cfg(test)]
mod tests;
```

**Step 4: Run test to verify it passes**

Run: `cargo test -p matw-core content`
Expected: PASS (all 3 tests pass)

**Step 5: Update lib.rs**

```rust
// crates/matw-core/src/lib.rs
pub mod content;

pub use content::Content;
```

**Step 6: Commit**

```bash
git add crates/matw-core/src/content.rs
git commit -m "feat: add Content enum with helper methods"
```

---

### Task 4: Implement Message Struct

**Files:**
- Create: `crates/matw-core/src/message.rs`
- Create: `crates/matw-core/src/message/tests.rs`

**Step 1: Write the failing test**

```rust
// crates/matw-core/src/message/tests.rs
#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Content, Role};
    use uuid::Uuid;

    #[test]
    fn test_new_user_message() {
        let msg = Message::new_user("Hello".to_string());
        assert_eq!(msg.role(), Role::User);
        assert_eq!(msg.content().as_str(), Some("Hello"));
    }

    #[test]
    fn test_new_assistant_message() {
        let msg = Message::new_assistant("Hi there!".to_string());
        assert_eq!(msg.role(), Role::Assistant);
    }

    #[test]
    fn test_message_with_tool_use() {
        let content = Content::ToolUse {
            id: "call_123".to_string(),
            name: "read".to_string(),
            input: serde_json::json!({"path": "test.txt"}),
        };
        let msg = Message::new(Role::Assistant, content);
        assert!(msg.has_tool_use());
    }

    #[test]
    fn test_message_serialization() {
        let msg = Message::new_user("test".to_string());
        let json = serde_json::to_string(&msg).unwrap();
        assert!(json.contains("\"role\":\"user\""));
    }

    #[test]
    fn test_message_id_is_unique() {
        let msg1 = Message::new_user("test".to_string());
        let msg2 = Message::new_user("test".to_string());
        assert_ne!(msg1.id(), msg2.id());
    }
}
```

**Step 2: Run test to verify it fails**

Run: `cargo test -p matw-core message`
Expected: COMPILER ERROR - Message not defined

**Step 3: Write minimal implementation**

```rust
// crates/matw-core/src/message.rs
use crate::{Content, Role};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    id: Uuid,
    role: Role,
    content: Content,
    timestamp: DateTime<Utc>,
    metadata: HashMap<String, serde_json::Value>,
}

impl Message {
    pub fn new(role: Role, content: Content) -> Self {
        Self {
            id: Uuid::new_v4(),
            role,
            content,
            timestamp: Utc::now(),
            metadata: HashMap::new(),
        }
    }

    pub fn new_user(text: String) -> Self {
        Self::new(Role::User, Content::Text(text))
    }

    pub fn new_assistant(text: String) -> Self {
        Self::new(Role::Assistant, Content::Text(text))
    }

    pub fn new_system(text: String) -> Self {
        Self::new(Role::System, Content::Text(text))
    }

    pub fn new_tool_use(id: String, name: String, input: serde_json::Value) -> Self {
        Self::new(Role::Assistant, Content::ToolUse { id, name, input })
    }

    pub fn new_tool_result(id: String, content: String, is_error: bool) -> Self {
        Self::new(Role::Tool, Content::ToolResult { id, content, is_error })
    }

    // Getters
    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn role(&self) -> Role {
        self.role
    }

    pub fn content(&self) -> &Content {
        &self.content
    }

    pub fn timestamp(&self) -> DateTime<Utc> {
        self.timestamp
    }

    pub fn metadata(&self) -> &HashMap<String, serde_json::Value> {
        &self.metadata
    }

    // Helper methods
    pub fn has_tool_use(&self) -> bool {
        matches!(&self.content, Content::ToolUse { .. })
    }

    pub fn is_tool_result(&self) -> bool {
        matches!(&self.content, Content::ToolResult { .. })
    }

    pub fn is_error(&self) -> bool {
        self.content.is_error()
    }
}

#[cfg(test)]
mod tests;
```

**Step 4: Run test to verify it passes**

Run: `cargo test -p matw-core message`
Expected: PASS (all 5 tests pass)

**Step 5: Update lib.rs**

```rust
// crates/matw-core/src/lib.rs
pub mod message;

pub use message::Message;
```

**Step 6: Commit**

```bash
git add crates/matw-core/src/message.rs
git commit -m "feat: add Message struct with constructors"
```

---

### Task 5: Implement Error Types

**Files:**
- Create: `crates/matw-core/src/error.rs`
- Create: `crates/matw-core/src/error/tests.rs`

**Step 1: Write the failing test**

```rust
// crates/matw-core/src/error/tests.rs
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let err = MatwError::Config("missing api key".to_string());
        assert_eq!(err.to_string(), "Configuration error: missing api key");
    }

    #[test]
    fn test_error_from_io() {
        let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "file not found");
        let err: MatwError = io_err.into();
        assert!(matches!(err, MatwError::IO(_)));
    }

    #[test]
    fn test_result_type_alias() {
        fn returns_ok() -> Result<String> {
            Ok("success".to_string())
        }
        assert!(returns_ok().is_ok());
    }
}
```

**Step 2: Run test to verify it fails**

Run: `cargo test -p matw-core error`
Expected: COMPILER ERROR - MatwError not defined

**Step 3: Write minimal implementation**

```rust
// crates/matw-core/src/error.rs
use std::io;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum MatwError {
    #[error("AI provider error: {0}")]
    AI(String),

    #[error("Tool execution error: {0}")]
    Tool(String),

    #[error("IO error: {0}")]
    IO(#[from] io::Error),

    #[error("Plugin error: {0}")]
    Plugin(String),

    #[error("Session not found: {0}")]
    SessionNotFound(String),

    #[error("Configuration error: {0}")]
    Config(String),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("UTF-8 error: {0}")]
    Utf8(#[from] std::str::Utf8Error),
}

pub type Result<T> = std::result::Result<T, MatwError>;

#[cfg(test)]
mod tests;
```

**Step 4: Run test to verify it passes**

Run: `cargo test -p matw-core error`
Expected: PASS (all 3 tests pass)

**Step 5: Update lib.rs**

```rust
// crates/matw-core/src/lib.rs
pub mod error;

pub use error::{MatwError, Result};
```

**Step 6: Commit**

```bash
git add crates/matw-core/src/error.rs
git commit -m "feat: add error types and Result alias"
```

---

### Task 6: Implement Session and Context

**Files:**
- Create: `crates/matw-core/src/context.rs`
- Create: `crates/matw-core/src/session.rs`
- Create: `crates/matw-core/src/session/tests.rs`

**Step 1: Write the failing test**

```rust
// crates/matw-core/src/session/tests.rs
#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Content, Message, Role};
    use std::path::PathBuf;

    #[test]
    fn test_new_session() {
        let session = Session::new(PathBuf::from("/tmp"));
        assert_eq!(session.message_count(), 0);
        assert!(session.is_active());
    }

    #[test]
    fn test_add_message() {
        let mut session = Session::new(PathBuf::from("/tmp"));
        session.add_message(Message::new_user("hello".to_string()));
        assert_eq!(session.message_count(), 1);
    }

    #[test]
    fn test_context_working_dir() {
        let session = Session::new(PathBuf::from("/home/user/project"));
        assert_eq!(session.context().working_dir(), PathBuf::from("/home/user/project"));
    }

    #[test]
    fn test_session_id_is_unique() {
        let s1 = Session::new(PathBuf::from("/tmp"));
        let s2 = Session::new(PathBuf::from("/tmp"));
        assert_ne!(s1.id(), s2.id());
    }

    #[test]
    fn test_close_session() {
        let mut session = Session::new(PathBuf::from("/tmp"));
        session.close();
        assert!(!session.is_active());
    }
}
```

**Step 2: Run test to verify it fails**

Run: `cargo test -p matw-core session`
Expected: COMPILER ERROR - Session/Context not defined

**Step 3: Write minimal implementation**

```rust
// crates/matw-core/src/context.rs
use crate::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitInfo {
    pub branch: String,
    pub commit: String,
    pub root: PathBuf,
}

#[derive(Debug, Clone)]
pub struct Context {
    working_dir: PathBuf,
    git_info: Option<GitInfo>,
    environment: HashMap<String, String>,
    claude_md: Option<String>,
}

impl Context {
    pub fn new(working_dir: PathBuf) -> Self {
        Self {
            working_dir,
            git_info: None,
            environment: HashMap::new(),
            claude_md: None,
        }
    }

    pub fn working_dir(&self) -> &Path {
        &self.working_dir
    }

    pub fn git_info(&self) -> Option<&GitInfo> {
        self.git_info.as_ref()
    }

    pub fn set_git_info(&mut self, info: GitInfo) {
        self.git_info = Some(info);
    }

    pub fn environment(&self) -> &HashMap<String, String> {
        &self.environment
    }

    pub fn set_env(&mut self, key: String, value: String) {
        self.environment.insert(key, value);
    }
}
```

```rust
// crates/matw-core/src/session.rs
use crate::{context::Context, message::Message, MatwError, Result};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SessionState {
    Active,
    Paused,
    Closed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    id: Uuid,
    messages: Vec<Message>,
    context: Context,
    state: SessionState,
}

impl Session {
    pub fn new(working_dir: PathBuf) -> Self {
        Self {
            id: Uuid::new_v4(),
            messages: Vec::new(),
            context: Context::new(working_dir),
            state: SessionState::Active,
        }
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn context(&self) -> &Context {
        &self.context
    }

    pub fn context_mut(&mut self) -> &mut Context {
        &mut self.context
    }

    pub fn messages(&self) -> &[Message] {
        &self.messages
    }

    pub fn message_count(&self) -> usize {
        self.messages.len()
    }

    pub fn state(&self) -> SessionState {
        self.state
    }

    pub fn is_active(&self) -> bool {
        self.state == SessionState::Active
    }

    pub fn add_message(&mut self, message: Message) {
        self.messages.push(message);
    }

    pub fn close(&mut self) {
        self.state = SessionState::Closed;
    }

    pub fn pause(&mut self) {
        self.state = SessionState::Paused;
    }

    pub fn resume(&mut self) {
        self.state = SessionState::Active;
    }

    pub fn to_ai_request(&self) -> Vec<&Message> {
        self.messages.iter().collect()
    }
}

#[cfg(test)]
mod tests;
```

**Step 4: Update lib.rs**

```rust
// crates/matw-core/src/lib.rs
pub mod context;
pub mod session;

pub use context::{Context, GitInfo};
pub use session::{Session, SessionState};
```

**Step 5: Run test to verify it passes**

Run: `cargo test -p matw-core session`
Expected: PASS (all 5 tests pass)

**Step 6: Commit**

```bash
git add crates/matw-core/src/context.rs crates/matw-core/src/session.rs
git commit -m "feat: add Session and Context structures"
```

---

## Phase 2: AI Provider Abstraction

### Task 7: Create matw-ai Crate and Provider Trait

**Files:**
- Create: `crates/matw-ai/Cargo.toml`
- Create: `crates/matw-ai/src/lib.rs`
- Create: `crates/matw-ai/src/provider.rs`
- Create: `crates/matw-ai/src/provider/tests.rs`

**Step 1: Update workspace Cargo.toml**

```toml
[workspace]
members = [
    "crates/matw-core",
    "crates/matw-ai",
]
```

**Step 2: Create matw-ai Cargo.toml**

```toml
[package]
name = "matw-ai"
version = "0.1.0"
edition = "2021"

[dependencies]
matw-core = { path = "../matw-core" }
tokio = { workspace = true }
futures = { workspace = true }
serde = { workspace = true }
serde_json = { workspace = true }
thiserror = { workspace = true }
async-trait = "0.1"
pin-project = "1.1"
```

**Step 3: Write the failing test**

```rust
// crates/matw-ai/src/provider/tests.rs
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tool_definition_serialization() {
        let tool = ToolDefinition {
            name: "read".to_string(),
            description: "Read a file".to_string(),
            parameters: serde_json::json!({
                "type": "object",
                "properties": {
                    "path": {"type": "string"}
                }
            }),
        };
        let json = serde_json::to_string(&tool).unwrap();
        assert!(json.contains("\"name\":\"read\""));
    }
}
```

**Step 4: Run test to verify it fails**

Run: `cargo test -p matw-ai`
Expected: COMPILER ERROR - types not defined

**Step 5: Write minimal implementation**

```rust
// crates/matw-ai/src/provider.rs
use async_trait::async_trait;
use futures::Stream;
use matw_core::Message;
use pin_project::pin_project;
use serde::{Deserialize, Serialize};
use std::pin::Pin;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolDefinition {
    pub name: String,
    pub description: String,
    pub parameters: serde_json::Value,
}

#[derive(Debug, Clone)]
pub struct ToolUse {
    pub id: String,
    pub name: String,
    pub input: serde_json::Value,
}

#[derive(Debug, Clone)]
pub struct CompletionRequest {
    pub messages: Vec<Message>,
    pub tools: Vec<ToolDefinition>,
    pub model: String,
    pub max_tokens: Option<u32>,
    pub temperature: Option<f32>,
    pub system_prompt: Option<String>,
}

#[derive(Debug, Clone)]
pub struct CompletionResponse {
    pub content: String,
    pub tool_uses: Vec<ToolUse>,
    pub stop_reason: StopReason,
    pub usage: Usage,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StopReason {
    EndTurn,
    MaxTokens,
    StopSequence,
    ToolUse,
}

#[derive(Debug, Clone, Copy)]
pub struct Usage {
    pub input_tokens: u32,
    pub output_tokens: u32,
}

#[pin_project]
pub struct ChunkStream {
    #[pin]
    inner: Pin<Box<dyn Stream<Item = Result<Chunk, AIError>> + Send>>,
}

impl Stream for ChunkStream {
    type Item = Result<Chunk, AIError>;

    fn poll_next(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        self.project().inner.poll_next(cx)
    }
}

#[derive(Debug, Clone)]
pub enum Chunk {
    Delta(String),
    ToolUse(ToolUse),
    Done,
}

#[async_trait]
pub trait AIProvider: Send + Sync {
    fn name(&self) -> &str;

    async fn stream_completion(
        &self,
        request: CompletionRequest,
    ) -> Result<ChunkStream, AIError>;

    async fn complete(
        &self,
        request: CompletionRequest,
    ) -> Result<CompletionResponse, AIError>;
}

#[cfg(test)]
mod tests;
```

**Step 6: Update lib.rs**

```rust
// crates/matw-ai/src/lib.rs
pub mod provider;
pub mod error;

pub use provider::{
    AIProvider, Chunk, ChunkStream, CompletionRequest, CompletionResponse,
    StopReason, ToolDefinition, ToolUse, Usage,
};
pub use error::{AIError, AIResult};
```

**Step 7: Create error.rs**

```rust
// crates/matw-ai/src/error.rs
use thiserror::Error;

pub type AIResult<T> = Result<T, AIError>;

#[derive(Debug, Error)]
pub enum AIError {
    #[error("API request failed: {0}")]
    RequestFailed(String),

    #[error("API returned error: {code} - {message}")]
    APIError { code: String, message: String },

    #[error("Invalid response format: {0}")]
    InvalidResponse(String),

    #[error("Provider not configured: {0}")]
    NotConfigured(String),

    #[error("Stream interrupted")]
    StreamInterrupted,

    #[error("Authentication failed")]
    AuthenticationFailed,

    #[error("Rate limit exceeded")]
    RateLimitExceeded,
}
```

**Step 8: Run test to verify it passes**

Run: `cargo test -p matw-ai`
Expected: PASS

**Step 9: Commit**

```bash
git add Cargo.toml crates/matw-ai/
git commit -m "feat: add matw-ai crate with AIProvider trait"
```

---

### Task 8: Implement GLM Provider

**Files:**
- Create: `crates/matw-ai/src/config.rs`
- Create: `crates/matw-ai/src/providers/mod.rs`
- Create: `crates/matw-ai/src/providers/glm.rs`
- Create: `crates/matw-ai/src/providers/glm/tests.rs`

**Step 1: Write the failing test**

```rust
// crates/matw-ai/src/providers/glm/tests.rs
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_glm_provider_name() {
        let provider = GLMProvider::new("test-key".to_string(), None);
        assert_eq!(provider.name(), "glm");
    }

    #[test]
    fn test_default_base_url() {
        let provider = GLMProvider::new("test-key".to_string(), None);
        assert_eq!(provider.base_url(), "https://open.bigmodel.cn/api/paas/v4/");
    }

    #[test]
    fn test_custom_base_url() {
        let provider = GLMProvider::new("test-key".to_string(), Some("https://custom.api".to_string()));
        assert_eq!(provider.base_url(), "https://custom.api");
    }
}
```

**Step 2: Run test to verify it fails**

Run: `cargo test -p matw-ai glm`
Expected: COMPILER ERROR

**Step 3: Add reqwest dependency**

Update `crates/matw-ai/Cargo.toml`:

```toml
reqwest = { version = "0.12", features = ["json", "stream"] }
```

**Step 4: Write minimal implementation**

```rust
// crates/matw-ai/src/config.rs
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Deserialize)]
pub struct AIConfig {
    pub default_provider: String,
    pub providers: HashMap<String, ProviderConfig>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ProviderConfig {
    #[serde(flatten)]
    pub config: ProviderTypeConfig,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum ProviderTypeConfig {
    Claude {
        api_key: String,
        base_url: Option<String>,
        model: String,
    },
    OpenAI {
        api_key: String,
        base_url: Option<String>,
        model: String,
    },
    Ollama {
        base_url: Option<String>,
        model: String,
    },
    GLM {
        api_key: String,
        base_url: Option<String>,
        model: String,
    },
    Kimi {
        api_key: String,
        base_url: Option<String>,
        model: String,
    },
}
```

```rust
// crates/matw-ai/src/providers/glm.rs
use super::super::{AIError, AIResult, Chunk, ChunkStream, CompletionRequest, CompletionResponse, StopReason, ToolDefinition, ToolUse, Usage};
use async_trait::async_trait;
use futures::{stream, StreamExt};
use matw_core::Message;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::Duration;

const DEFAULT_BASE_URL: &str = "https://open.bigmodel.cn/api/paas/v4/";
const DEFAULT_MODEL: &str = "glm-4-plus";

pub struct GLMProvider {
    api_key: String,
    base_url: String,
    client: Client,
}

impl GLMProvider {
    pub fn new(api_key: String, base_url: Option<String>) -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(120))
            .build()
            .expect("Failed to create HTTP client");

        Self {
            api_key,
            base_url: base_url.unwrap_or_else(|| DEFAULT_BASE_URL.to_string()),
            client,
        }
    }

    pub fn base_url(&self) -> &str {
        &self.base_url
    }

    fn convert_messages(messages: Vec<Message>) -> Vec<GLMMessage> {
        messages
            .into_iter()
            .map(|m| GLMMessage {
                role: match m.role() {
                    matw_core::Role::User => "user",
                    matw_core::Role::Assistant => "assistant",
                    matw_core::Role::System => "system",
                    matw_core::Role::Tool => "tool",
                }
                .to_string(),
                content: m.content().as_str().unwrap_or("").to_string(),
            })
            .collect()
    }
}

#[async_trait]
impl AIProvider for GLMProvider {
    fn name(&self) -> &str {
        "glm"
    }

    async fn stream_completion(
        &self,
        request: CompletionRequest,
    ) -> AIResult<ChunkStream> {
        // For now, return a mock stream
        let stream = stream::once(async {
            Ok(Chunk::Delta("GLM response".to_string()))
        });
        Ok(ChunkStream {
            inner: Box::pin(stream),
        })
    }

    async fn complete(
        &self,
        request: CompletionRequest,
    ) -> AIResult<CompletionResponse> {
        let glm_request = GLMRequest {
            model: request.model,
            messages: Self::convert_messages(request.messages),
            stream: false,
            tools: if request.tools.is_empty() {
                None
            } else {
                Some(request.tools)
            },
        };

        let response = self
            .client
            .post(format!("{}chat/completions", self.base_url))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&glm_request)
            .send()
            .await
            .map_err(|e| AIError::RequestFailed(e.to_string()))?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            return Err(AIError::APIError {
                code: status.as_u16().to_string(),
                message: body,
            });
        }

        let glm_response: GLMResponse = response
            .json()
            .await
            .map_err(|e| AIError::InvalidResponse(e.to_string()))?;

        Ok(CompletionResponse {
            content: glm_response
                .choices
                .first()
                .and_then(|c| c.message.content.clone())
                .unwrap_or_default(),
            tool_uses: vec![],
            stop_reason: StopReason::EndTurn,
            usage: Usage {
                input_tokens: glm_response.usage.prompt_tokens as u32,
                output_tokens: glm_response.usage.completion_tokens as u32,
            },
        })
    }
}

#[derive(Debug, Serialize)]
struct GLMRequest {
    model: String,
    messages: Vec<GLMMessage>,
    stream: bool,
    tools: Option<Vec<ToolDefinition>>,
}

#[derive(Debug, Serialize, Deserialize)]
struct GLMMessage {
    role: String,
    content: String,
}

#[derive(Debug, Deserialize)]
struct GLMResponse {
    choices: Vec<GLMChoice>,
    usage: GLMUsage,
}

#[derive(Debug, Deserialize)]
struct GLMChoice {
    message: GLMResponseMessage,
}

#[derive(Debug, Deserialize)]
struct GLMResponseMessage {
    content: Option<String>,
}

#[derive(Debug, Deserialize)]
struct GLMUsage {
    prompt_tokens: usize,
    completion_tokens: usize,
}

#[cfg(test)]
mod tests;
```

**Step 5: Update providers/mod.rs**

```rust
// crates/matw-ai/src/providers/mod.rs
pub mod glm;

pub use glm::GLMProvider;
```

**Step 6: Run test to verify it passes**

Run: `cargo test -p matw-ai glm`
Expected: PASS (all 3 tests pass)

**Step 7: Commit**

```bash
git add crates/matw-ai/src/config.rs crates/matw-ai/src/providers/
git commit -m "feat: add GLM provider implementation"
```

---

### Task 9: Implement Kimi Provider

**Files:**
- Create: `crates/matw-ai/src/providers/kimi.rs`
- Create: `crates/matw-ai/src/providers/kimi/tests.rs`

**Step 1: Write the failing test**

```rust
// crates/matw-ai/src/providers/kimi/tests.rs
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kimi_provider_name() {
        let provider = KimiProvider::new("test-key".to_string(), None);
        assert_eq!(provider.name(), "kimi");
    }

    #[test]
    fn test_default_base_url() {
        let provider = KimiProvider::new("test-key".to_string(), None);
        assert_eq!(provider.base_url(), "https://api.moonshot.cn/v1");
    }
}
```

**Step 2: Run test to verify it fails**

Run: `cargo test -p matw-ai kimi`
Expected: COMPILER ERROR

**Step 3: Write minimal implementation**

```rust
// crates/matw-ai/src/providers/kimi.rs
use super::super::{AIError, AIResult, Chunk, ChunkStream, CompletionRequest, CompletionResponse, StopReason, ToolUse, Usage};
use async_trait::async_trait;
use futures::{stream, StreamExt};
use reqwest::Client;
use std::time::Duration;

const DEFAULT_BASE_URL: &str = "https://api.moonshot.cn/v1";

pub struct KimiProvider {
    api_key: String,
    base_url: String,
    client: Client,
}

impl KimiProvider {
    pub fn new(api_key: String, base_url: Option<String>) -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(120))
            .build()
            .expect("Failed to create HTTP client");

        Self {
            api_key,
            base_url: base_url.unwrap_or_else(|| DEFAULT_BASE_URL.to_string()),
            client,
        }
    }

    pub fn base_url(&self) -> &str {
        &self.base_url
    }
}

#[async_trait]
impl AIProvider for KimiProvider {
    fn name(&self) -> &str {
        "kimi"
    }

    async fn stream_completion(
        &self,
        _request: CompletionRequest,
    ) -> AIResult<ChunkStream> {
        let stream = stream::once(async {
            Ok(Chunk::Delta("Kimi response".to_string()))
        });
        Ok(ChunkStream {
            inner: Box::pin(stream),
        })
    }

    async fn complete(
        &self,
        request: CompletionRequest,
    ) -> AIResult<CompletionResponse> {
        // Kimi uses OpenAI-compatible API
        // Implementation similar to GLM but with OpenAI format
        Ok(CompletionResponse {
            content: "Kimi response".to_string(),
            tool_uses: vec![],
            stop_reason: StopReason::EndTurn,
            usage: Usage {
                input_tokens: 0,
                output_tokens: 0,
            },
        })
    }
}

#[cfg(test)]
mod tests;
```

**Step 4: Update providers/mod.rs**

```rust
// crates/matw-ai/src/providers/mod.rs
pub mod glm;
pub mod kimi;

pub use glm::GLMProvider;
pub use kimi::KimiProvider;
```

**Step 5: Run test to verify it passes**

Run: `cargo test -p matw-ai kimi`
Expected: PASS

**Step 6: Commit**

```bash
git add crates/matw-ai/src/providers/kimi.rs
git commit -m "feat: add Kimi provider implementation"
```

---

### Task 10: Implement Claude Provider

**Files:**
- Create: `crates/matw-ai/src/providers/claude.rs`
- Create: `crates/matw-ai/src/providers/claude/tests.rs`

**Step 1: Write the failing test**

```rust
// crates/matw-ai/src/providers/claude/tests.rs
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_claude_provider_name() {
        let provider = ClaudeProvider::new("test-key".to_string(), None);
        assert_eq!(provider.name(), "claude");
    }
}
```

**Step 2: Run test to verify it fails**

Run: `cargo test -p matw-ai claude`
Expected: COMPILER ERROR

**Step 3: Write minimal implementation**

```rust
// crates/matw-ai/src/providers/claude.rs
use super::super::{AIError, AIResult, Chunk, ChunkStream, CompletionRequest, CompletionResponse, StopReason, ToolUse, Usage};
use async_trait::async_trait;
use futures::{stream, StreamExt};
use reqwest::Client;
use std::time::Duration;

const DEFAULT_BASE_URL: &str = "https://api.anthropic.com/v1";

pub struct ClaudeProvider {
    api_key: String,
    base_url: String,
    client: Client,
}

impl ClaudeProvider {
    pub fn new(api_key: String, base_url: Option<String>) -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(120))
            .build()
            .expect("Failed to create HTTP client");

        Self {
            api_key,
            base_url: base_url.unwrap_or_else(|| DEFAULT_BASE_URL.to_string()),
            client,
        }
    }
}

#[async_trait]
impl AIProvider for ClaudeProvider {
    fn name(&self) -> &str {
        "claude"
    }

    async fn stream_completion(
        &self,
        _request: CompletionRequest,
    ) -> AIResult<ChunkStream> {
        let stream = stream::once(async {
            Ok(Chunk::Delta("Claude response".to_string()))
        });
        Ok(ChunkStream {
            inner: Box::pin(stream),
        })
    }

    async fn complete(
        &self,
        _request: CompletionRequest,
    ) -> AIResult<CompletionResponse> {
        Ok(CompletionResponse {
            content: "Claude response".to_string(),
            tool_uses: vec![],
            stop_reason: StopReason::EndTurn,
            usage: Usage {
                input_tokens: 0,
                output_tokens: 0,
            },
        })
    }
}

#[cfg(test)]
mod tests;
```

**Step 4: Update providers/mod.rs**

```rust
// crates/matw-ai/src/providers/mod.rs
pub mod claude;
pub mod glm;
pub mod kimi;

pub use claude::ClaudeProvider;
pub use glm::GLMProvider;
pub use kimi::KimiProvider;
```

**Step 5: Run test to verify it passes**

Run: `cargo test -p matw-ai claude`
Expected: PASS

**Step 6: Commit**

```bash
git add crates/matw-ai/src/providers/claude.rs
git commit -m "feat: add Claude provider implementation"
```

---

## Phase 3: Tool System

### Task 11: Create matw-tools Crate and Tool Trait

**Files:**
- Create: `crates/matw-tools/Cargo.toml`
- Create: `crates/matw-tools/src/lib.rs`
- Create: `crates/matw-tools/src/tool.rs`

**Step 1: Update workspace**

```toml
# Cargo.toml
[workspace]
members = [
    "crates/matw-core",
    "crates/matw-ai",
    "crates/matw-tools",
]
```

**Step 2: Create Cargo.toml**

```toml
# crates/matw-tools/Cargo.toml
[package]
name = "matw-tools"
version = "0.1.0"
edition = "2021"

[dependencies]
matw-core = { path = "../matw-core" }
tokio = { workspace = true }
serde = { workspace = true }
serde_json = { workspace = true }
thiserror = { workspace = true }
walkdir = "2.5"
ignore = "0.4"
grep-cli = "0.1"

[dev-dependencies]
tempfile = { workspace = true }
```

**Step 3: Write tool trait**

```rust
// crates/matw-tools/src/tool.rs
use async_trait::async_trait;
use matw_core::Result;
use serde_json::Value;

#[derive(Debug, Clone)]
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
    IO(#[from] std::io::Error),

    #[error("Not found: {0}")]
    NotFound(String),
}

#[async_trait]
pub trait Tool: Send + Sync {
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    fn parameters_schema(&self) -> &Value;

    async fn execute(&self, input: Value) -> Result<ToolOutput, ToolError>;
}
```

**Step 4: Update lib.rs**

```rust
// crates/matw-tools/src/lib.rs
pub mod tool;
pub mod tools;

pub use tool::{Tool, ToolError, ToolOutput};
pub use tools::all_tools;
```

**Step 5: Commit**

```bash
git add Cargo.toml crates/matw-tools/
git commit -m "feat: add matw-tools crate with Tool trait"
```

---

### Task 12: Implement Read Tool

**Files:**
- Create: `crates/matw-tools/src/tools/mod.rs`
- Create: `crates/matw-tools/src/tools/read.rs`
- Create: `crates/matw-tools/src/tools/read/tests.rs`

**Step 1: Write the failing test**

```rust
// crates/matw-tools/src/tools/read/tests.rs
#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::PathBuf;
    use tempfile::TempDir;

    #[tokio::test]
    async fn test_read_file() {
        let temp = TempDir::new().unwrap();
        let file_path = temp.path().join("test.txt");
        fs::write(&file_path, "hello world").unwrap();

        let tool = ReadTool::new();
        let input = serde_json::json!({"path": file_path.to_str().unwrap()});
        let result = tool.execute(input).await.unwrap();

        assert_eq!(result.content, "hello world");
        assert!(!result.is_error);
    }

    #[tokio::test]
    async fn test_read_nonexistent_file() {
        let tool = ReadTool::new();
        let input = serde_json::json!({"path": "/nonexistent/file.txt"});
        let result = tool.execute(input).await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_read_tool_name() {
        let tool = ReadTool::new();
        assert_eq!(tool.name(), "read");
    }
}
```

**Step 2: Run test to verify it fails**

Run: `cargo test -p matw-tools read`
Expected: COMPILER ERROR

**Step 3: Write minimal implementation**

```rust
// crates/matw-tools/src/tools/read.rs
use super::super::{Tool, ToolError, ToolOutput};
use async_trait::async_trait;
use serde::Deserialize;
use serde_json::json;
use std::fs;
use std::path::Path;

pub struct ReadTool;

impl ReadTool {
    pub fn new() -> Self {
        Self
    }
}

impl Default for ReadTool {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Deserialize)]
struct ReadInput {
    path: String,
}

#[async_trait]
impl Tool for ReadTool {
    fn name(&self) -> &str {
        "read"
    }

    fn description(&self) -> &str {
        "Read the contents of a file"
    }

    fn parameters_schema(&self) -> &serde_json::Value {
        &json!({
            "type": "object",
            "properties": {
                "path": {
                    "type": "string",
                    "description": "Absolute or relative path to the file"
                }
            },
            "required": ["path"]
        })
    }

    async fn execute(&self, input: serde_json::Value) -> Result<ToolOutput, ToolError> {
        let input: ReadInput = serde_json::from_value(input)
            .map_err(|e| ToolError::InvalidParameters(e.to_string()))?;

        let path = Path::new(&input.path);

        if !path.exists() {
            return Err(ToolError::NotFound(input.path));
        }

        let content = fs::read_to_string(path)
            .map_err(|e| ToolError::ExecutionFailed(e.to_string()))?;

        Ok(ToolOutput {
            content,
            is_error: false,
        })
    }
}

#[cfg(test)]
mod tests;
```

**Step 4: Update tools/mod.rs**

```rust
// crates/matw-tools/src/tools/mod.rs
pub mod read;

pub use read::ReadTool;

pub fn all_tools() -> Vec<Box<dyn Tool>> {
    vec![
        Box::new(ReadTool::new()),
    ]
}
```

**Step 5: Run test to verify it passes**

Run: `cargo test -p matw-tools read`
Expected: PASS (all 3 tests pass)

**Step 6: Commit**

```bash
git add crates/matw-tools/src/tools/
git commit -m "feat: add Read tool implementation"
```

---

### Task 13: Implement Write Tool

**Files:**
- Create: `crates/matw-tools/src/tools/write.rs`
- Create: `crates/matw-tools/src/tools/write/tests.rs`

**Step 1: Write the failing test**

```rust
// crates/matw-tools/src/tools/write/tests.rs
#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[tokio::test]
    async fn test_write_file() {
        let temp = TempDir::new().unwrap();
        let file_path = temp.path().join("test.txt");

        let tool = WriteTool::new();
        let input = serde_json::json!({
            "path": file_path.to_str().unwrap(),
            "content": "hello world"
        });
        let result = tool.execute(input).await.unwrap();

        assert!(result.content.contains("Wrote"));
        assert!(!result.is_error);

        let content = fs::read_to_string(&file_path).unwrap();
        assert_eq!(content, "hello world");
    }

    #[tokio::test]
    async fn test_write_creates_directories() {
        let temp = TempDir::new().unwrap();
        let file_path = temp.path().join("nested/dir/test.txt");

        let tool = WriteTool::new();
        let input = serde_json::json!({
            "path": file_path.to_str().unwrap(),
            "content": "content"
        });
        let result = tool.execute(input).await.unwrap();

        assert!(!result.is_error);
        assert!(file_path.exists());
    }
}
```

**Step 2: Run test to verify it fails**

Run: `cargo test -p matw-tools write`
Expected: COMPILER ERROR

**Step 3: Write minimal implementation**

```rust
// crates/matw-tools/src/tools/write.rs
use super::super::{Tool, ToolError, ToolOutput};
use async_trait::async_trait;
use serde::Deserialize;
use serde_json::json;
use std::fs;
use std::path::Path;

pub struct WriteTool;

impl WriteTool {
    pub fn new() -> Self {
        Self
    }
}

impl Default for WriteTool {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Deserialize)]
struct WriteInput {
    path: String,
    content: String,
}

#[async_trait]
impl Tool for WriteTool {
    fn name(&self) -> &str {
        "write"
    }

    fn description(&self) -> &str {
        "Write content to a file, creating directories if needed"
    }

    fn parameters_schema(&self) -> &serde_json::Value {
        &json!({
            "type": "object",
            "properties": {
                "path": {
                    "type": "string",
                    "description": "Absolute or relative path to the file"
                },
                "content": {
                    "type": "string",
                    "description": "Content to write"
                }
            },
            "required": ["path", "content"]
        })
    }

    async fn execute(&self, input: serde_json::Value) -> Result<ToolOutput, ToolError> {
        let input: WriteInput = serde_json::from_value(input)
            .map_err(|e| ToolError::InvalidParameters(e.to_string()))?;

        let path = Path::new(&input.path);

        // Create parent directories if they don't exist
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)
                .map_err(|e| ToolError::ExecutionFailed(format!("Failed to create directory: {}", e)))?;
        }

        fs::write(path, &input.content)
            .map_err(|e| ToolError::ExecutionFailed(e.to_string()))?;

        Ok(ToolOutput {
            content: format!("Wrote {} bytes to {}", input.content.len(), input.path),
            is_error: false,
        })
    }
}

#[cfg(test)]
mod tests;
```

**Step 4: Update tools/mod.rs**

```rust
// crates/matw-tools/src/tools/mod.rs
pub mod read;
pub mod write;

pub use read::ReadTool;
pub use write::WriteTool;

pub fn all_tools() -> Vec<Box<dyn Tool>> {
    vec![
        Box::new(ReadTool::new()),
        Box::new(WriteTool::new()),
    ]
}
```

**Step 5: Run test to verify it passes**

Run: `cargo test -p matw-tools write`
Expected: PASS

**Step 6: Commit**

```bash
git add crates/matw-tools/src/tools/write.rs
git commit -m "feat: add Write tool implementation"
```

---

### Task 14: Implement Glob Tool

**Files:**
- Create: `crates/matw-tools/src/tools/glob.rs`
- Create: `crates/matw-tools/src/tools/glob/tests.rs`

**Step 1: Write the failing test**

```rust
// crates/matw-tools/src/tools/glob/tests.rs
#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[tokio::test]
    async fn test_glob_files() {
        let temp = TempDir::new().unwrap();
        fs::write(temp.path().join("test.rs"), "content").unwrap();
        fs::write(temp.path().join("main.rs"), "content").unwrap();
        fs::write(temp.path().join("test.txt"), "content").unwrap();

        let tool = GlobTool::new();
        let input = serde_json::json!({
            "pattern": "**/*.rs",
            "path": temp.path().to_str().unwrap()
        });
        let result = tool.execute(input).await.unwrap();

        assert!(result.content.contains("test.rs"));
        assert!(result.content.contains("main.rs"));
        assert!(!result.content.contains("test.txt"));
    }
}
```

**Step 2: Run test to verify it fails**

Run: `cargo test -p matw-tools glob`
Expected: COMPILER ERROR

**Step 3: Write minimal implementation**

```rust
// crates/matw-tools/src/tools/glob.rs
use super::super::{Tool, ToolError, ToolOutput};
use async_trait::async_trait;
use ignore::{Walk, WalkBuilder};
use serde::Deserialize;
use serde_json::json;
use std::path::Path;

pub struct GlobTool;

impl GlobTool {
    pub fn new() -> Self {
        Self
    }
}

impl Default for GlobTool {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Deserialize)]
struct GlobInput {
    #[serde(default)]
    pattern: String,
    #[serde(default)]
    path: String,
}

#[async_trait]
impl Tool for GlobTool {
    fn name(&self) -> &str {
        "glob"
    }

    fn description(&self) -> &str {
        "Find files matching a glob pattern"
    }

    fn parameters_schema(&self) -> &serde_json::Value {
        &json!({
            "type": "object",
            "properties": {
                "pattern": {
                    "type": "string",
                    "description": "Glob pattern (e.g., **/*.rs)"
                },
                "path": {
                    "type": "string",
                    "description": "Root directory to search (defaults to current directory)"
                }
            }
        })
    }

    async fn execute(&self, input: serde_json::Value) -> Result<ToolOutput, ToolError> {
        let input: GlobInput = serde_json::from_value(input)
            .map_err(|e| ToolError::InvalidParameters(e.to_string()))?;

        let search_path = if input.path.is_empty() {
            "."
        } else {
            &input.path
        };

        let mut results = Vec::new();

        if !input.pattern.is_empty() {
            // Use gitignore-aware walk
            let walker = WalkBuilder::new(search_path)
                .git_ignore(true)
                .git_global(true)
                .build();

            for entry in walker {
                if let Ok(entry) = entry {
                    if entry.file_type().map_or(false, |t| t.is_file()) {
                        let path = entry.path();
                        if path.to_string_lossy().contains(&input.pattern.replace("*", "")) {
                            results.push(path.to_string_lossy().to_string());
                        }
                    }
                }
            }
        } else {
            // List all files if no pattern
            for entry in Walk::new(search_path) {
                if let Ok(entry) = entry {
                    if entry.file_type().map_or(false, |t| t.is_file()) {
                        results.push(entry.path().to_string_lossy().to_string());
                    }
                }
            }
        }

        let output = if results.is_empty() {
            "No matching files found".to_string()
        } else {
            results.join("\n")
        };

        Ok(ToolOutput {
            content: output,
            is_error: false,
        })
    }
}

#[cfg(test)]
mod tests;
```

**Step 4: Update tools/mod.rs**

```rust
// crates/matw-tools/src/tools/mod.rs
pub mod glob;
pub mod read;
pub mod write;

pub use glob::GlobTool;
pub use read::ReadTool;
pub use write::WriteTool;

pub fn all_tools() -> Vec<Box<dyn Tool>> {
    vec![
        Box::new(GlobTool::new()),
        Box::new(ReadTool::new()),
        Box::new(WriteTool::new()),
    ]
}
```

**Step 5: Run test to verify it passes**

Run: `cargo test -p matw-tools glob`
Expected: PASS

**Step 6: Commit**

```bash
git add crates/matw-tools/src/tools/glob.rs
git commit -m "feat: add Glob tool implementation"
```

---

### Task 15: Implement Bash Tool

**Files:**
- Create: `crates/matw-tools/src/tools/bash.rs`
- Create: `crates/matw-tools/src/tools/bash/tests.rs`

**Step 1: Write the failing test**

```rust
// crates/matw-tools/src/tools/bash/tests.rs
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_bash_echo() {
        let tool = BashTool::new();
        let input = serde_json::json!({"command": "echo hello"});
        let result = tool.execute(input).await.unwrap();

        assert!(result.content.contains("hello"));
        assert!(!result.is_error);
    }

    #[tokio::test]
    async fn test_bash_nonzero_exit() {
        let tool = BashTool::new();
        let input = serde_json::json!({"command": "exit 1"});
        let result = tool.execute(input).await.unwrap();

        // Should return content with error, not throw
        assert!(result.is_error);
    }
}
```

**Step 2: Run test to verify it fails**

Run: `cargo test -p matw-tools bash`
Expected: COMPILER ERROR

**Step 3: Write minimal implementation**

```rust
// crates/matw-tools/src/tools/bash.rs
use super::super::{Tool, ToolError, ToolOutput};
use async_trait::async_trait;
use serde::Deserialize;
use serde_json::json;
use std::process::Command;
use std::time::Duration;

pub struct BashTool;

impl BashTool {
    pub fn new() -> Self {
        Self
    }

    fn execute_command(command: &str, timeout_ms: Option<u64>) -> Result<String, ToolError> {
        let (shell, shell_arg) = if cfg!(target_os = "windows") {
            ("cmd", "/C")
        } else {
            ("sh", "-c")
        };

        let mut cmd = Command::new(shell);
        cmd.arg(shell_arg).arg(command);

        let output = if let Some(ms) = timeout_ms {
            cmd.timeout(Duration::from_millis(ms))
                .output()
                .map_err(|e| ToolError::ExecutionFailed(e.to_string()))?
        } else {
            cmd.output()
                .map_err(|e| ToolError::ExecutionFailed(e.to_string()))?
        };

        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();

        if output.status.success() {
            Ok(stdout)
        } else {
            let mut result = stdout;
            if !result.is_empty() && !stderr.is_empty() {
                result.push('\n');
            }
            result.push_str(&stderr);
            Ok(result)
        }
    }
}

impl Default for BashTool {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Deserialize)]
struct BashInput {
    command: String,
    #[serde(default)]
    timeout: Option<u64>,
}

#[async_trait]
impl Tool for BashTool {
    fn name(&self) -> &str {
        "bash"
    }

    fn description(&self) -> &str {
        "Execute a shell command"
    }

    fn parameters_schema(&self) -> &serde_json::Value {
        &json!({
            "type": "object",
            "properties": {
                "command": {
                    "type": "string",
                    "description": "Shell command to execute"
                },
                "timeout": {
                    "type": "integer",
                    "description": "Timeout in milliseconds (default: 120000)"
                }
            },
            "required": ["command"]
        })
    }

    async fn execute(&self, input: serde_json::Value) -> Result<ToolOutput, ToolError> {
        let input: BashInput = serde_json::from_value(input)
            .map_err(|e| ToolError::InvalidParameters(e.to_string()))?;

        let timeout = input.timeout.unwrap_or(120000);
        let output = Self::execute_command(&input.command, Some(timeout))?;

        let is_error = !output.is_empty();

        Ok(ToolOutput {
            content: output,
            is_error,
        })
    }
}

#[cfg(test)]
mod tests;
```

**Step 4: Update tools/mod.rs**

```rust
// crates/matw-tools/src/tools/mod.rs
pub mod bash;
pub mod glob;
pub mod read;
pub mod write;

pub use bash::BashTool;
pub use glob::GlobTool;
pub use read::ReadTool;
pub use write::WriteTool;

pub fn all_tools() -> Vec<Box<dyn Tool>> {
    vec![
        Box::new(BashTool::new()),
        Box::new(GlobTool::new()),
        Box::new(ReadTool::new()),
        Box::new(WriteTool::new()),
    ]
}
```

**Step 5: Run test to verify it passes**

Run: `cargo test -p matw-tools bash`
Expected: PASS

**Step 6: Commit**

```bash
git add crates/matw-tools/src/tools/bash.rs
git commit -m "feat: add Bash tool implementation"
```

---

## Phase 4: Basic CLI Entry Point

### Task 16: Create CLI Main Entry

**Files:**
- Create: `src/main.rs`

**Step 1: Write basic main function**

```rust
// src/main.rs
use matw_core::{Message, Role, Session};
use matw_tools::all_tools;
use std::path::PathBuf;

#[tokio::main]
async fn main() -> matw_core::Result<()> {
    println!("MATW - AI Coding Assistant in Rust");
    println!("====================================\n");

    // Get current directory
    let working_dir = std::env::current_dir()?;

    // Create session
    let mut session = Session::new(working_dir);

    println!("Session created: {}", session.id());
    println!("Working directory: {:?}\n", session.context().working_dir());

    // List available tools
    println!("Available tools:");
    for tool in all_tools() {
        println!("  - {}: {}", tool.name(), tool.description());
    }

    println!("\nMATW is ready!");

    Ok(())
}
```

**Step 2: Update Cargo.toml**

```toml
# Cargo.toml (root)
[workspace]
members = [
    "crates/matw-core",
    "crates/matw-ai",
    "crates/matw-tools",
]

[package]
name = "matw"
version = "0.1.0"
edition = "2021"

[[bin]]
name = "matw"
path = "src/main.rs"

[dependencies]
matw-core = { path = "crates/matw-core" }
matw-tools = { path = "crates/matw-tools" }
tokio = { workspace = true }
```

**Step 3: Run to verify**

Run: `cargo run`
Expected: Output showing session creation and available tools

**Step 4: Commit**

```bash
git add src/ Cargo.toml
git commit -m "feat: add CLI entry point"
```

---

### Task 17: Add Integration Test

**Files:**
- Create: `tests/integration_test.rs`

**Step 1: Write integration test**

```rust
// tests/integration_test.rs
use matw_core::{Content, Message, Role, Session};
use matw_tools::{ReadTool, Tool};
use std::fs;
use tempfile::TempDir;

#[tokio::test]
async fn test_session_workflow() {
    let temp = TempDir::new().unwrap();
    let file_path = temp.path().join("test.txt");
    fs::write(&file_path, "hello world").unwrap();

    // Create session
    let mut session = Session::new(temp.path().to_path_buf());
    assert_eq!(session.message_count(), 0);

    // Add messages
    session.add_message(Message::new_user("read the file".to_string()));
    session.add_message(Message::new_assistant("I'll read it".to_string()));
    assert_eq!(session.message_count(), 2);

    // Verify session is active
    assert!(session.is_active());

    // Test tool
    let tool = ReadTool::new();
    let input = serde_json::json!({"path": file_path.to_str().unwrap()});
    let result = tool.execute(input).await.unwrap();
    assert_eq!(result.content, "hello world");
}

#[tokio::test]
async fn test_message_types() {
    let temp = TempDir::new().unwrap();

    let mut session = Session::new(temp.path().to_path_buf());

    // User message
    session.add_message(Message::new_user("test".to_string()));
    assert_eq!(session.messages().last().unwrap().role(), Role::User);

    // Assistant message
    session.add_message(Message::new_assistant("response".to_string()));
    assert_eq!(session.messages().last().unwrap().role(), Role::Assistant);

    // Tool use
    session.add_message(Message::new_tool_use(
        "call_1".to_string(),
        "read".to_string(),
        serde_json::json!({"path": "test.txt"}),
    ));
    assert!(session.messages().last().unwrap().has_tool_use());

    // Tool result
    session.add_message(Message::new_tool_result(
        "call_1".to_string(),
        "content".to_string(),
        false,
    ));
    assert!(session.messages().last().unwrap().is_tool_result());
}

#[tokio::test]
async fn test_session_state_changes() {
    let temp = TempDir::new().unwrap();
    let mut session = Session::new(temp.path().to_path_buf());

    assert!(session.is_active());

    session.pause();
    assert!(!session.is_active());

    session.resume();
    assert!(session.is_active());

    session.close();
    assert!(!session.is_active());
}
```

**Step 2: Run integration tests**

Run: `cargo test --test integration_test`
Expected: PASS (all tests pass)

**Step 3: Commit**

```bash
git add tests/
git commit -m "test: add integration tests"
```

---

## Summary

This implementation plan covers:

1. **Phase 1**: Core domain models (Message, Session, Context, Error types)
2. **Phase 2**: AI provider abstraction with GLM, Kimi, and Claude providers
3. **Phase 3**: Tool system with Read, Write, Glob, and Bash tools
4. **Phase 4**: Basic CLI entry point and integration tests

Each task follows TDD methodology:
1. Write failing test
2. Run test to verify failure
3. Write minimal implementation
4. Run test to verify passing
5. Commit changes

The plan is designed to be executed incrementally, with each commit representing a small, verifiable step.

**Total estimated tasks**: 17
**Estimated completion time**: 1-2 weeks for a developer familiar with Rust

**Next phases** (not covered in this plan):
- Phase 5: TUI with ratatui
- Phase 6: Plugin system with MCP
- Phase 7: Full agent orchestration
