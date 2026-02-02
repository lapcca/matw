// Tools will be implemented incrementally following TDD
pub mod bash;
pub mod glob;
pub mod read;
pub mod write;

pub use bash::BashTool;
pub use glob::GlobTool;
pub use read::ReadTool;
pub use write::WriteTool;

/// Get all available tools
pub fn all_tools() -> Vec<Box<dyn crate::Tool>> {
    vec![
        Box::new(BashTool::new()),
        Box::new(GlobTool::new()),
        Box::new(ReadTool::new()),
        Box::new(WriteTool::new()),
    ]
}
