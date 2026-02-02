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
