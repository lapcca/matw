//! MATW Agent - Agent orchestration for MATW
//!
//! Provides agent loop and orchestration for AI interactions.

pub mod agent;
pub mod streaming;

pub use agent::{Agent, AgentError};
pub use streaming::process_streaming;
