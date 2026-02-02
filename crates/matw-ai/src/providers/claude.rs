use super::super::{AIResult, Chunk, ChunkStream, CompletionRequest, CompletionResponse, StopReason, Usage};
use async_trait::async_trait;
use futures::stream;
use reqwest::Client;
use std::time::Duration;

const DEFAULT_BASE_URL: &str = "https://api.anthropic.com/v1";

pub struct ClaudeProvider {
    _api_key: String,
    _base_url: String,
    _client: Client,
}

impl ClaudeProvider {
    pub fn new(api_key: String, base_url: Option<String>) -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(120))
            .build()
            .expect("Failed to create HTTP client");

        Self {
            _api_key: api_key,
            _base_url: base_url.unwrap_or_else(|| DEFAULT_BASE_URL.to_string()),
            _client: client,
        }
    }
}

#[async_trait]
impl super::super::AIProvider for ClaudeProvider {
    fn name(&self) -> &str {
        "claude"
    }

    async fn stream_completion(
        &self,
        _request: CompletionRequest,
    ) -> AIResult<ChunkStream> {
        let stream = stream::once(async {
            Ok(Chunk::Delta("Claude response".to_string()))
        });
        Ok(ChunkStream::new(Box::pin(stream)))
    }

    async fn complete(
        &self,
        _request: CompletionRequest,
    ) -> AIResult<CompletionResponse> {
        Ok(CompletionResponse {
            content: "Claude response".to_string(),
            tool_uses: vec![],
            stop_reason: StopReason::EndTurn,
            usage: Usage {
                input_tokens: 0,
                output_tokens: 0,
            },
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::provider::AIProvider;

    #[test]
    fn test_claude_provider_name() {
        let provider = ClaudeProvider::new("test-key".to_string(), None);
        assert_eq!(provider.name(), "claude");
    }
}
