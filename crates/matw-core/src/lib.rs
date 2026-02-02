// MATW Core - Core domain models
// Modules will be populated incrementally following TDD

pub mod context;
pub mod session;
pub mod message;
pub mod role;
pub mod content;
pub mod error;

pub use context::{Context, GitInfo};
pub use session::{Session, SessionState};
pub use role::Role;
pub use content::Content;
pub use message::Message;
pub use error::{MatwError, Result};
