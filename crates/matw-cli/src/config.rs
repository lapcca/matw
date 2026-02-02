use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Default configuration file location
fn default_config_path() -> PathBuf {
    dirs::home_dir()
        .expect("Unable to determine home directory")
        .join(".matw")
        .join("config.toml")
}

/// CLI configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// AI provider to use
    #[serde(default = "default_provider")]
    pub provider: String,

    /// API key for the provider
    #[serde(default)]
    pub api_key: Option<String>,

    /// Base URL for custom providers
    #[serde(default)]
    pub base_url: Option<String>,

    /// Model to use
    #[serde(default = "default_model")]
    pub model: String,

    /// Maximum tokens in response
    #[serde(default = "default_max_tokens")]
    pub max_tokens: usize,

    /// Temperature for generation
    #[serde(default = "default_temperature")]
    pub temperature: f32,
}

fn default_provider() -> String {
    "claude".to_string()
}

fn default_model() -> String {
    "claude-sonnet-4-20250514".to_string()
}

fn default_max_tokens() -> usize {
    8192
}

fn default_temperature() -> f32 {
    0.7
}

impl Default for Config {
    fn default() -> Self {
        Self {
            provider: default_provider(),
            api_key: None,
            base_url: None,
            model: default_model(),
            max_tokens: default_max_tokens(),
            temperature: default_temperature(),
        }
    }
}

impl Config {
    /// Load configuration from file, or return defaults
    pub fn load() -> Result<Self> {
        let path = default_config_path();

        if !path.exists() {
            return Ok(Self::default());
        }

        let content = std::fs::read_to_string(&path)?;
        let config: Self = toml::from_str(&content)?;

        Ok(config)
    }

    /// Save configuration to file
    pub fn save(&self) -> Result<()> {
        let path = default_config_path();

        // Create parent directory if needed
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let content = toml::to_string_pretty(self)?;
        std::fs::write(&path, content)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_default() {
        let config = Config::default();
        assert_eq!(config.provider, "claude");
        assert_eq!(config.model, "claude-sonnet-4-20250514");
        assert_eq!(config.max_tokens, 8192);
        assert_eq!(config.temperature, 0.7);
    }

    #[test]
    fn test_config_serialize() {
        let config = Config {
            provider: "glm".to_string(),
            api_key: Some("test-key".to_string()),
            base_url: Some("https://api.example.com".to_string()),
            model: "glm-4".to_string(),
            max_tokens: 4096,
            temperature: 0.5,
        };

        let toml_str = toml::to_string_pretty(&config).unwrap();
        assert!(toml_str.contains("provider = \"glm\""));
        assert!(toml_str.contains("api_key = \"test-key\""));
    }

    #[test]
    fn test_config_deserialize() {
        let toml_str = r#"
            provider = "kimi"
            model = "moonshot-v1-8k"
            max_tokens = 2048
            temperature = 0.8
        "#;

        let config: Config = toml::from_str(toml_str).unwrap();
        assert_eq!(config.provider, "kimi");
        assert_eq!(config.model, "moonshot-v1-8k");
        assert_eq!(config.max_tokens, 2048);
        assert_eq!(config.temperature, 0.8);
    }
}
