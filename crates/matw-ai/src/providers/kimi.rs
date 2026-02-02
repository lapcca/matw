use super::super::{AIResult, Chunk, ChunkStream, CompletionRequest, CompletionResponse, StopReason, Usage};
use async_trait::async_trait;
use futures::stream;
use reqwest::Client;
use std::time::Duration;

const DEFAULT_BASE_URL: &str = "https://api.moonshot.cn/v1";

pub struct KimiProvider {
    api_key: String,
    base_url: String,
    _client: Client,
}

impl KimiProvider {
    pub fn new(api_key: String, base_url: Option<String>) -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(120))
            .build()
            .expect("Failed to create HTTP client");

        Self {
            api_key,
            base_url: base_url.unwrap_or_else(|| DEFAULT_BASE_URL.to_string()),
            _client: client,
        }
    }

    pub fn base_url(&self) -> &str {
        &self.base_url
    }
}

#[async_trait]
impl super::super::AIProvider for KimiProvider {
    fn name(&self) -> &str {
        "kimi"
    }

    async fn stream_completion(
        &self,
        _request: CompletionRequest,
    ) -> AIResult<ChunkStream> {
        let stream = stream::once(async {
            Ok(Chunk::Delta("Kimi response".to_string()))
        });
        Ok(ChunkStream::new(Box::pin(stream)))
    }

    async fn complete(
        &self,
        _request: CompletionRequest,
    ) -> AIResult<CompletionResponse> {
        // Kimi uses OpenAI-compatible API
        // Implementation similar to GLM but with OpenAI format
        Ok(CompletionResponse {
            content: "Kimi response".to_string(),
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
    fn test_kimi_provider_name() {
        let provider = KimiProvider::new("test-key".to_string(), None);
        assert_eq!(provider.name(), "kimi");
    }

    #[test]
    fn test_default_base_url() {
        let provider = KimiProvider::new("test-key".to_string(), None);
        assert_eq!(provider.base_url(), "https://api.moonshot.cn/v1");
    }
}
