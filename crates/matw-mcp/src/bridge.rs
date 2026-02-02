//! Bridge adapter to connect matw-tools to MCP

use super::protocol::ContentItem;
use super::server::{MCTool, MCPServer};
use matw_tools::Tool as MatwTool;
use std::sync::Arc;

pub struct ToolAdapter {
    tool: Arc<dyn MatwTool>,
}

impl ToolAdapter {
    pub fn new(tool: Arc<dyn MatwTool>) -> Self {
        Self { tool }
    }
}

#[async_trait::async_trait]
impl MCTool for ToolAdapter {
    fn name(&self) -> &str {
        self.tool.name()
    }

    fn description(&self) -> &str {
        self.tool.description()
    }

    fn input_schema(&self) -> serde_json::Value {
        self.tool.parameters_schema().clone()
    }

    async fn execute(&self, args: serde_json::Value) -> Result<Vec<ContentItem>, String> {
        let output = self.tool.execute(args).await
            .map_err(|e| e.to_string())?;

        Ok(vec![ContentItem::Text {
            text: output.content,
        }])
    }
}

pub async fn register_tools(server: &MCPServer, tools: Vec<Arc<dyn MatwTool>>) {
    for tool in tools {
        let adapter = Box::new(ToolAdapter::new(tool));
        server.register_tool(adapter).await;
    }
}
