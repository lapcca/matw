use super::super::{AIError, AIResult, Chunk, ChunkStream, CompletionRequest, CompletionResponse, StopReason, Usage};
use async_trait::async_trait;
use futures::stream;
use matw_core::Message;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::Duration;

const DEFAULT_BASE_URL: &str = "https://open.bigmodel.cn/api/paas/v4/";

pub struct GLMProvider {
    api_key: String,
    base_url: String,
    client: Client,
}

impl GLMProvider {
    pub fn new(api_key: String, base_url: Option<String>) -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(120))
            .build()
            .expect("Failed to create HTTP client");

        Self {
            api_key,
            base_url: base_url.unwrap_or_else(|| DEFAULT_BASE_URL.to_string()),
            client,
        }
    }

    pub fn base_url(&self) -> &str {
        &self.base_url
    }

    fn convert_messages(messages: Vec<Message>) -> Vec<GLMMessage> {
        messages
            .into_iter()
            .map(|m| GLMMessage {
                role: match m.role() {
                    matw_core::Role::User => "user",
                    matw_core::Role::Assistant => "assistant",
                    matw_core::Role::System => "system",
                    matw_core::Role::Tool => "tool",
                }
                .to_string(),
                content: m.content().as_str().unwrap_or("").to_string(),
            })
            .collect()
    }
}

#[async_trait]
impl super::super::AIProvider for GLMProvider {
    fn name(&self) -> &str {
        "glm"
    }

    async fn stream_completion(
        &self,
        _request: CompletionRequest,
    ) -> AIResult<ChunkStream> {
        let stream = stream::once(async {
            Ok(Chunk::Delta("GLM response".to_string()))
        });
        Ok(ChunkStream::new(Box::pin(stream)))
    }

    async fn complete(
        &self,
        request: CompletionRequest,
    ) -> AIResult<CompletionResponse> {
        let glm_request = GLMRequest {
            model: request.model,
            messages: Self::convert_messages(request.messages),
            stream: false,
        };

        let response = self
            .client
            .post(format!("{}chat/completions", self.base_url))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&glm_request)
            .send()
            .await
            .map_err(|e| AIError::RequestFailed(e.to_string()))?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            return Err(AIError::APIError {
                code: status.as_u16().to_string(),
                message: body,
            });
        }

        let glm_response: GLMResponse = response
            .json()
            .await
            .map_err(|e| AIError::InvalidResponse(e.to_string()))?;

        Ok(CompletionResponse {
            content: glm_response
                .choices
                .first()
                .and_then(|c| c.message.content.clone())
                .unwrap_or_default(),
            tool_uses: vec![],
            stop_reason: StopReason::EndTurn,
            usage: Usage {
                input_tokens: glm_response.usage.prompt_tokens as u32,
                output_tokens: glm_response.usage.completion_tokens as u32,
            },
        })
    }
}

#[derive(Debug, Serialize)]
struct GLMRequest {
    model: String,
    messages: Vec<GLMMessage>,
    stream: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct GLMMessage {
    role: String,
    content: String,
}

#[derive(Debug, Deserialize)]
struct GLMResponse {
    choices: Vec<GLMChoice>,
    usage: GLMUsage,
}

#[derive(Debug, Deserialize)]
struct GLMChoice {
    message: GLMResponseMessage,
}

#[derive(Debug, Deserialize)]
struct GLMResponseMessage {
    content: Option<String>,
}

#[derive(Debug, Deserialize)]
struct GLMUsage {
    prompt_tokens: usize,
    completion_tokens: usize,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::provider::AIProvider;

    #[test]
    fn test_glm_provider_name() {
        let provider = GLMProvider::new("test-key".to_string(), None);
        assert_eq!(provider.name(), "glm");
    }

    #[test]
    fn test_default_base_url() {
        let provider = GLMProvider::new("test-key".to_string(), None);
        assert_eq!(provider.base_url(), "https://open.bigmodel.cn/api/paas/v4/");
    }

    #[test]
    fn test_custom_base_url() {
        let provider = GLMProvider::new("test-key".to_string(), Some("https://custom.api".to_string()));
        assert_eq!(provider.base_url(), "https://custom.api");
    }
}
