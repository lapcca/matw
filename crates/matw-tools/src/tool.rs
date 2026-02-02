use async_trait::async_trait;
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
