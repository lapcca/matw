// Tools will be implemented incrementally following TDD
pub mod read;
pub mod write;

pub use read::ReadTool;
pub use write::WriteTool;

pub fn all_tools() -> Vec<Box<dyn crate::Tool>> {
    vec![
        Box::new(ReadTool::new()),
        Box::new(WriteTool::new()),
    ]
}
