//! MATW TUI - Terminal User Interface for MATW
//!
//! Provides a ratatui-based terminal UI for the MATW AI coding assistant.

pub mod app;
pub mod ui;
pub mod event;

pub use app::App;
pub use event::{Event, EventHandler};
pub use ui::UI;
