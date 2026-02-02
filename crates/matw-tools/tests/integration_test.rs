use matw_tools::{Tool, tools::{ReadTool, WriteTool, GlobTool, BashTool}};
use tempfile::TempDir;
use std::fs;

#[tokio::test]
async fn test_write_and_read_file() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.txt");

    // Write file
    let write_tool = WriteTool::new();
    let write_input = serde_json::json!({
        "path": file_path.to_str().unwrap(),
        "content": "Hello, World!"
    });
    let write_result = write_tool.execute(write_input).await.unwrap();
    assert!(!write_result.is_error);
    assert!(file_path.exists());

    // Read file
    let read_tool = ReadTool::new();
    let read_input = serde_json::json!({
        "path": file_path.to_str().unwrap()
    });
    let read_result = read_tool.execute(read_input).await.unwrap();
    assert!(!read_result.is_error);
    assert_eq!(read_result.content, "Hello, World!");
}

#[tokio::test]
async fn test_nested_directory_creation() {
    let temp_dir = TempDir::new().unwrap();
    let nested_path = temp_dir.path().join("a/b/c/d/file.txt");

    let write_tool = WriteTool::new();
    let input = serde_json::json!({
        "path": nested_path.to_str().unwrap(),
        "content": "nested content"
    });
    let result = write_tool.execute(input).await.unwrap();

    assert!(!result.is_error);
    assert!(nested_path.exists());
    assert_eq!(fs::read_to_string(&nested_path).unwrap(), "nested content");
}

#[tokio::test]
async fn test_glob_with_real_files() {
    let temp_dir = TempDir::new().unwrap();

    // Create test files
    fs::write(temp_dir.path().join("main.rs"), "fn main() {}").unwrap();
    fs::write(temp_dir.path().join("lib.rs"), "pub fn hello() {}").unwrap();
    fs::write(temp_dir.path().join("test.txt"), "text file").unwrap();
    fs::write(temp_dir.path().join("readme.md"), "# README").unwrap();

    // Test Rust file glob
    let glob_tool = GlobTool::new();
    let input = serde_json::json!({
        "pattern": "*.rs",
        "path": temp_dir.path().to_str().unwrap()
    });
    let result = glob_tool.execute(input).await.unwrap();

    assert!(!result.is_error);
    assert!(result.content.contains("main.rs"));
    assert!(result.content.contains("lib.rs"));
    assert!(!result.content.contains("test.txt"));
    assert!(!result.content.contains("readme.md"));
}

#[tokio::test]
async fn test_bash_command_chain() {
    let bash_tool = BashTool::new();

    // Test command chaining with pipe
    let input = serde_json::json!({
        "command": "echo \"hello world\" | tr a-z A-Z"
    });
    let result = bash_tool.execute(input).await.unwrap();

    assert!(!result.is_error);
    assert!(result.content.contains("HELLO WORLD"));
}

#[tokio::test]
async fn test_tool_error_handling() {
    let temp_dir = TempDir::new().unwrap();
    let nonexistent = temp_dir.path().join("does_not_exist.txt");

    // Try to read non-existent file
    let read_tool = ReadTool::new();
    let input = serde_json::json!({
        "path": nonexistent.to_str().unwrap()
    });
    let result = read_tool.execute(input).await;

    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("Not found"));
}

#[tokio::test]
async fn test_glob_recursive_pattern() {
    let temp_dir = TempDir::new().unwrap();

    // Create nested structure
    fs::write(temp_dir.path().join("file.rs"), "root").unwrap();
    fs::create_dir_all(temp_dir.path().join("src")).unwrap();
    fs::write(temp_dir.path().join("src/file.rs"), "src").unwrap();
    fs::create_dir_all(temp_dir.path().join("src/utils")).unwrap();
    fs::write(temp_dir.path().join("src/utils/file.rs"), "src/utils").unwrap();

    let glob_tool = GlobTool::new();
    let input = serde_json::json!({
        "pattern": "**/*.rs",
        "path": temp_dir.path().to_str().unwrap()
    });
    let result = glob_tool.execute(input).await.unwrap();

    assert!(!result.is_error);
    // Should find all three .rs files
    let lines: Vec<&str> = result.content.lines().collect();
    assert_eq!(lines.len(), 3);
}

#[tokio::test]
async fn test_bash_timeout_enforcement() {
    let bash_tool = BashTool::new();

    // Command that sleeps longer than timeout
    let input = serde_json::json!({
        "command": "sleep 5",
        "timeout_ms": 100
    });
    let result = bash_tool.execute(input).await;

    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("timed out"));
}

#[tokio::test]
async fn test_workflow_edit_file() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("config.toml");

    // Write initial content
    let write_tool = WriteTool::new();
    write_tool.execute(serde_json::json!({
        "path": file_path.to_str().unwrap(),
        "content": "name = \"old\"\nversion = \"1.0\""
    })).await.unwrap();

    // Read it back
    let read_tool = ReadTool::new();
    let read_result = read_tool.execute(serde_json::json!({
        "path": file_path.to_str().unwrap()
    })).await.unwrap();

    assert_eq!(read_result.content, "name = \"old\"\nversion = \"1.0\"");

    // Update (write new content)
    write_tool.execute(serde_json::json!({
        "path": file_path.to_str().unwrap(),
        "content": "name = \"new\"\nversion = \"2.0\""
    })).await.unwrap();

    // Verify update
    let updated = read_tool.execute(serde_json::json!({
        "path": file_path.to_str().unwrap()
    })).await.unwrap();

    assert_eq!(updated.content, "name = \"new\"\nversion = \"2.0\"");
}
