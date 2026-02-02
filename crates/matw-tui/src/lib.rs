//! MATW TUI - Terminal User Interface for MATW
//!
//! Provides a ratatui-based terminal UI for the MATW AI coding assistant.

pub mod app;
pub mod ui;
pub mod event;
pub mod runner;

pub use app::App;
pub use event::{Event, EventHandler};
pub use runner::run;
pub use ui::UI;
