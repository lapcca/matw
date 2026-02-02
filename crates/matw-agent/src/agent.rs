//! Agent orchestration

use matw_ai::{AIProvider, CompletionRequest, ToolDefinition};
use matw_core::{Message, Role, Session};
use matw_tools::Tool;
use std::sync::Arc;

pub struct Agent<P: AIProvider> {
    provider: P,
    tools: Vec<Arc<dyn Tool>>,
    max_iterations: usize,
}

impl<P: AIProvider> Agent<P> {
    pub fn new(provider: P, tools: Vec<Arc<dyn Tool>>) -> Self {
        Self {
            provider,
            tools,
            max_iterations: 10,
        }
    }

    pub async fn process(&self, session: &mut Session) -> Result<(), AgentError> {
        let mut iteration = 0;

        loop {
            // Check max iterations
            if iteration >= self.max_iterations {
                return Err(AgentError::MaxIterationsReached);
            }

            // Get last user message
            let _last_user_msg = session.messages()
                .iter()
                .rev()
                .find(|m| m.role() == Role::User)
                .ok_or(AgentError::NoUserMessage)?;

            // Prepare completion request
            let tool_defs: Vec<_> = self.tools.iter()
                .map(|t| ToolDefinition {
                    name: t.name().to_string(),
                    description: t.description().to_string(),
                    parameters: t.parameters_schema().clone(),
                })
                .collect();

            let request = CompletionRequest {
                messages: session.messages().to_vec(),
                tools: tool_defs,
                model: "default".to_string(),
                max_tokens: Some(4096),
                temperature: Some(0.7),
                system_prompt: Some(self.get_system_prompt()),
            };

            // Get AI response
            let response = self.provider.complete(request).await
                .map_err(|e| AgentError::AIProvider(e.to_string()))?;

            // Add assistant message
            session.add_message(Message::new_assistant(response.content.clone()));

            // Check for tool uses
            if !response.tool_uses.is_empty() {
                for tool_use in response.tool_uses {
                    // Add tool use message
                    session.add_message(Message::new_tool_use(
                        tool_use.id.clone(),
                        tool_use.name.clone(),
                        tool_use.input.clone(),
                    ));

                    // Execute tool
                    let tool = self.tools.iter()
                        .find(|t| t.name() == tool_use.name)
                        .ok_or_else(|| AgentError::ToolNotFound(tool_use.name.clone()))?;

                    let output = tool.execute(tool_use.input).await
                        .map_err(|e| AgentError::ToolExecution(e.to_string()))?;

                    // Add tool result message
                    session.add_message(Message::new_tool_result(
                        tool_use.id,
                        output.content,
                        output.is_error,
                    ));
                }

                iteration += 1;
                continue;
            }

            // No more tool uses, done
            break;
        }

        Ok(())
    }

    fn get_system_prompt(&self) -> String {
        "You are a helpful AI coding assistant with access to tools.".to_string()
    }
}

#[derive(Debug, thiserror::Error)]
pub enum AgentError {
    #[error("Max iterations reached")]
    MaxIterationsReached,

    #[error("No user message found")]
    NoUserMessage,

    #[error("AI provider error: {0}")]
    AIProvider(String),

    #[error("Tool not found: {0}")]
    ToolNotFound(String),

    #[error("Tool execution error: {0}")]
    ToolExecution(String),
}
