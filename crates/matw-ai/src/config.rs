use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Clone, Deserialize)]
pub struct AIConfig {
    pub default_provider: String,
    pub providers: HashMap<String, ProviderConfig>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ProviderConfig {
    #[serde(flatten)]
    pub config: ProviderTypeConfig,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum ProviderTypeConfig {
    Claude {
        api_key: String,
        base_url: Option<String>,
        model: String,
    },
    OpenAI {
        api_key: String,
        base_url: Option<String>,
        model: String,
    },
    Ollama {
        base_url: Option<String>,
        model: String,
    },
    GLM {
        api_key: String,
        base_url: Option<String>,
        model: String,
    },
    Kimi {
        api_key: String,
        base_url: Option<String>,
        model: String,
    },
}
