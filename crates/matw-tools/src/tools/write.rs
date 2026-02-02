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

    fn parameters_schema(&self) -> serde_json::Value {
        json!({
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
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_write_file() {
        let temp = tempfile::TempDir::new().unwrap();
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
        let temp = tempfile::TempDir::new().unwrap();
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
