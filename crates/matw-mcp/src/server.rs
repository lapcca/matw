//! MCP server implementation

use super::protocol::*;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct MCPServer {
    tools: Arc<RwLock<HashMap<String, Box<dyn MCTool>>>>,
}

#[async_trait::async_trait]
pub trait MCTool: Send + Sync {
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    fn input_schema(&self) -> serde_json::Value;
    async fn execute(&self, args: serde_json::Value) -> Result<Vec<ContentItem>, String>;
}

impl MCPServer {
    pub fn new() -> Self {
        Self {
            tools: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn register_tool(&self, tool: Box<dyn MCTool>) {
        let mut tools = self.tools.write().await;
        tools.insert(tool.name().to_string(), tool);
    }

    pub async fn handle_request(&self, request: JsonRpcRequest) -> JsonRpcResponse {
        let result = match request.method.as_str() {
            "tools/list" => self.list_tools().await,
            "tools/call" => self.call_tool(request.params).await,
            _ => Err(JsonRpcError {
                code: -32601,
                message: "Method not found".to_string(),
                data: None,
            }),
        };

        match result {
            Ok(result) => JsonRpcResponse {
                jsonrpc: "2.0".to_string(),
                id: request.id,
                result: Some(result),
                error: None,
            },
            Err(error) => JsonRpcResponse {
                jsonrpc: "2.0".to_string(),
                id: request.id,
                result: None,
                error: Some(error),
            },
        }
    }

    async fn list_tools(&self) -> Result<serde_json::Value, JsonRpcError> {
        let tools = self.tools.read().await;
        let tool_list: Vec<Tool> = tools.values().map(|t| {
            Tool {
                name: t.name().to_string(),
                description: t.description().to_string(),
                input_schema: t.input_schema(),
            }
        }).collect();

        Ok(serde_json::json!({ "tools": tool_list }))
    }

    async fn call_tool(&self, params: Option<serde_json::Value>) -> Result<serde_json::Value, JsonRpcError> {
        let params = params.ok_or_else(|| JsonRpcError {
            code: -32602,
            message: "Invalid params".to_string(),
            data: None,
        })?;

        let call: ToolCall = serde_json::from_value(params).map_err(|_| JsonRpcError {
            code: -32602,
            message: "Invalid tool call".to_string(),
            data: None,
        })?;

        let tools = self.tools.read().await;
        let tool = tools.get(&call.name).ok_or_else(|| JsonRpcError {
            code: -32602,
            message: format!("Tool not found: {}", call.name),
            data: None,
        })?;

        let content = tool.execute(call.arguments).await.map_err(|e| JsonRpcError {
            code: -32603,
            message: e,
            data: None,
        })?;

        Ok(serde_json::to_value(ToolResult {
            content,
            is_error: false,
        }).unwrap())
    }
}

impl Default for MCPServer {
    fn default() -> Self {
        Self::new()
    }
}
