//! MATW MCP - Model Context Protocol plugin system
//!
//! Provides MCP (Model Context Protocol) implementation for MATW.

pub mod bridge;
pub mod protocol;
pub mod server;

pub use bridge::{register_tools, ToolAdapter};
pub use server::{MCPServer, MCTool};
