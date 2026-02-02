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
mod tests {
    use super::*;

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
