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

    fn parameters_schema(&self) -> serde_json::Value {
        json!({
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
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_read_file() {
        let temp = tempfile::TempDir::new().unwrap();
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
