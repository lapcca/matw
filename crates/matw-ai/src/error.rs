use thiserror::Error;

pub type AIResult<T> = Result<T, AIError>;

#[derive(Debug, Error)]
pub enum AIError {
    #[error("API request failed: {0}")]
    RequestFailed(String),

    #[error("API returned error: {code} - {message}")]
    APIError { code: String, message: String },

    #[error("Invalid response format: {0}")]
    InvalidResponse(String),

    #[error("Provider not configured: {0}")]
    NotConfigured(String),

    #[error("Stream interrupted")]
    StreamInterrupted,

    #[error("Authentication failed")]
    AuthenticationFailed,

    #[error("Rate limit exceeded")]
    RateLimitExceeded,
}
