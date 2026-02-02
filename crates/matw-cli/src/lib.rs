//! MATW CLI library
//!
//! Provides command-line interface and session management for MATW.

pub mod config;
pub mod session;

pub use config::Config;
pub use session::{detect_git_info, initialize_session, load_claude_md};
