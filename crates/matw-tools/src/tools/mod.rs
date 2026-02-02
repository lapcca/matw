// Tools will be implemented incrementally following TDD
pub mod read;

pub use read::ReadTool;

pub fn all_tools() -> Vec<Box<dyn crate::Tool>> {
    vec![
        Box::new(ReadTool::new()),
    ]
}
