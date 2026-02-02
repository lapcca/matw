use super::super::{Tool, ToolError, ToolOutput};
use async_trait::async_trait;
use serde::Deserialize;
use serde_json::json;

pub struct BashTool;

impl BashTool {
    pub fn new() -> Self {
        Self
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
    timeout_ms: Option<u64>,
}

#[async_trait]
impl Tool for BashTool {
    fn name(&self) -> &str {
        "bash"
    }

    fn description(&self) -> &str {
        "Execute shell commands with optional timeout"
    }

    fn parameters_schema(&self) -> serde_json::Value {
        json!({
            "type": "object",
            "properties": {
                "command": {
                    "type": "string",
                    "description": "Shell command to execute"
                },
                "timeout_ms": {
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

        let timeout_ms = input.timeout_ms.unwrap_or(120000);

        // Execute command using tokio
        let output = tokio::time::timeout(
            std::time::Duration::from_millis(timeout_ms),
            execute_command(&input.command)
        )
        .await
        .map_err(|_| ToolError::ExecutionFailed("Command timed out".to_string()))??;

        Ok(ToolOutput {
            content: output,
            is_error: false,
        })
    }
}

async fn execute_command(command: &str) -> Result<String, ToolError> {
    use tokio::process::Command;

    // On Unix-like systems, use sh -c
    #[cfg(unix)]
    let result = Command::new("sh")
        .arg("-c")
        .arg(command)
        .output()
        .await;

    // On Windows, use cmd /C
    #[cfg(windows)]
    let result = Command::new("cmd")
        .arg("/C")
        .arg(command)
        .output()
        .await;

    let output = result.map_err(|e| ToolError::ExecutionFailed(e.to_string()))?;

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();

    if !output.status.success() {
        let code = output.status.code().unwrap_or(-1);
        return Err(ToolError::ExecutionFailed(format!(
            "Command failed with exit code {}: {}",
            code,
            if stderr.is_empty() { &stdout } else { &stderr }
        )));
    }

    Ok(if stderr.is_empty() {
        stdout
    } else {
        format!("{}\n{}", stdout, stderr)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_bash_echo_command() {
        let tool = BashTool::new();
        let input = json!({
            "command": "echo hello world"
        });
        let result = tool.execute(input).await.unwrap();

        assert_eq!(result.content.trim(), "hello world");
        assert!(!result.is_error);
    }

    #[tokio::test]
    async fn test_bash_failed_command() {
        let tool = BashTool::new();
        let input = json!({
            "command": "exit 1"
        });
        let result = tool.execute(input).await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_bash_timeout() {
        let tool = BashTool::new();
        let input = json!({
            "command": "sleep 10",
            "timeout_ms": 100
        });
        let result = tool.execute(input).await;

        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("timed out"));
    }
}
