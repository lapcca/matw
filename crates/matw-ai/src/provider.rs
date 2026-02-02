use async_trait::async_trait;
use futures::Stream;
use matw_core::Message;
use pin_project::pin_project;
use serde::{Deserialize, Serialize};
use std::pin::Pin;
use crate::AIError;

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

impl ChunkStream {
    pub fn new(stream: Pin<Box<dyn Stream<Item = Result<Chunk, AIError>> + Send>>) -> Self {
        Self { inner: stream }
    }
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
