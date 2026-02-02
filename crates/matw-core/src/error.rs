use std::io;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum MatwError {
    #[error("AI provider error: {0}")]
    AI(String),

    #[error("Tool execution error: {0}")]
    Tool(String),

    #[error("IO error: {0}")]
    IO(#[from] io::Error),

    #[error("Plugin error: {0}")]
    Plugin(String),

    #[error("Session not found: {0}")]
    SessionNotFound(String),

    #[error("Configuration error: {0}")]
    Config(String),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("UTF-8 error: {0}")]
    Utf8(#[from] std::str::Utf8Error),
}

pub type Result<T> = std::result::Result<T, MatwError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let err = MatwError::Config("missing api key".to_string());
        assert_eq!(err.to_string(), "Configuration error: missing api key");
    }

    #[test]
    fn test_error_from_io() {
        let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "file not found");
        let err: MatwError = io_err.into();
        assert!(matches!(err, MatwError::IO(_)));
    }

    #[test]
    fn test_result_type_alias() {
        fn returns_ok() -> Result<String> {
            Ok("success".to_string())
        }
        assert!(returns_ok().is_ok());
    }
}
