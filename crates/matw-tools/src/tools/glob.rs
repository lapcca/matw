use super::super::{Tool, ToolError, ToolOutput};
use async_trait::async_trait;
use ignore::{Walk, WalkBuilder};
use serde::Deserialize;
use serde_json::json;
use glob::Pattern;

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

    fn parameters_schema(&self) -> serde_json::Value {
        json!({
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
            // Compile glob pattern
            let pattern = Pattern::new(&input.pattern)
                .map_err(|e| ToolError::InvalidParameters(format!("Invalid glob pattern: {}", e)))?;

            // Use gitignore-aware walk
            let walker = WalkBuilder::new(search_path)
                .git_ignore(true)
                .git_global(true)
                .build();

            for entry in walker {
                if let Ok(entry) = entry {
                    if entry.file_type().map_or(false, |t| t.is_file()) {
                        let path = entry.path();
                        // Match against the pattern relative to search path
                        let relative_path = path.strip_prefix(search_path)
                            .unwrap_or(path)
                            .to_string_lossy();

                        if pattern.matches(&relative_path) {
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
mod tests {
    use super::*;
    use std::fs;

    #[tokio::test]
    async fn test_glob_files() {
        let temp = tempfile::TempDir::new().unwrap();
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
