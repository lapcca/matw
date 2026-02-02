use matw_core::{Content, Message, Role, Session, SessionState};
use tempfile::TempDir;

#[test]
fn test_session_lifecycle() {
    let temp_dir = TempDir::new().unwrap();
    let working_dir = temp_dir.path().to_path_buf();

    // Create session
    let mut session = Session::new(working_dir.clone());
    assert_eq!(session.state(), SessionState::Active);
    assert_eq!(session.message_count(), 0);
    assert!(session.context().working_dir() == working_dir);

    // Add user message
    let user_msg = Message::new_user("Hello, how are you?".to_string());
    session.add_message(user_msg);
    assert_eq!(session.message_count(), 1);

    // Add assistant message
    let assistant_msg = Message::new_assistant("I'm doing well!".to_string());
    session.add_message(assistant_msg);
    assert_eq!(session.message_count(), 2);

    // Pause session
    session.pause();
    assert_eq!(session.state(), SessionState::Paused);
    assert!(!session.is_active());

    // Resume session
    session.resume();
    assert_eq!(session.state(), SessionState::Active);
    assert!(session.is_active());

    // Close session
    session.close();
    assert_eq!(session.state(), SessionState::Closed);
    assert!(!session.is_active());
}

#[test]
fn test_message_content_types() {
    // Text content
    let text_msg = Message::new_user("Simple text message".to_string());
    assert!(matches!(text_msg.content(), Content::Text(_)));

    // Tool use content
    let tool_msg = Message::new_tool_use(
        "tool-123".to_string(),
        "read".to_string(),
        serde_json::json!({"path": "/tmp/file.txt"}),
    );
    assert!(matches!(tool_msg.content(), Content::ToolUse { .. }));

    // Tool result content
    let result_msg = Message::new_tool_result(
        "tool-123".to_string(),
        "File content here".to_string(),
        false,
    );
    assert!(matches!(result_msg.content(), Content::ToolResult { .. }));
}

#[test]
fn test_multi_turn_conversation() {
    let temp_dir = TempDir::new().unwrap();
    let mut session = Session::new(temp_dir.path().to_path_buf());

    // Turn 1: User asks question
    session.add_message(Message::new_user("What files are in this directory?".to_string()));

    // Turn 1: Assistant uses glob tool
    session.add_message(Message::new_tool_use(
        "tool-1".to_string(),
        "glob".to_string(),
        serde_json::json!({"pattern": "*"}),
    ));

    // Turn 1: Tool returns result
    session.add_message(Message::new_tool_result(
        "tool-1".to_string(),
        "file1.rs\nfile2.rs".to_string(),
        false,
    ));

    // Turn 1: Assistant responds
    session.add_message(Message::new_assistant(
        "I found 2 files: file1.rs and file2.rs".to_string()
    ));

    // Turn 2: User follows up
    session.add_message(Message::new_user("Read file1.rs".to_string()));

    assert_eq!(session.message_count(), 5);

    // Verify message order
    let messages = session.messages();
    assert!(matches!(messages[0].role(), Role::User));
    assert!(matches!(messages[1].role(), Role::Assistant));
    assert!(matches!(messages[2].role(), Role::Tool));
    assert!(matches!(messages[3].role(), Role::Assistant));
    assert!(matches!(messages[4].role(), Role::User));
}

#[test]
fn test_context_with_git_info() {
    use matw_core::{Context, GitInfo};
    use std::path::PathBuf;

    let temp_dir = TempDir::new().unwrap();
    let git_root = temp_dir.path();

    // Initialize git repo
    std::process::Command::new("git")
        .args(["-C", git_root.to_str().unwrap(), "init"])
        .output()
        .unwrap();

    std::process::Command::new("git")
        .args([
            "-C",
            git_root.to_str().unwrap(),
            "-c",
            "user.name=Test",
            "-c",
            "user.email=test@example.com",
            "commit",
            "--allow-empty",
            "-m",
            "Initial",
        ])
        .output()
        .unwrap();

    // Create context with git info
    let git_info = GitInfo {
        branch: "main".to_string(),
        commit: "abc123".to_string(),
        root: git_root.to_path_buf(),
    };

    let context = Context::with_details(git_root.to_path_buf(), Some(git_info), None);
    let session = Session::with_context(context);

    let retrieved_info = session.context().git_info();
    assert!(retrieved_info.is_some());
    let info = retrieved_info.unwrap();
    assert!(info.branch == "main" || info.branch == "master");
    assert!(!info.commit.is_empty());
    assert_eq!(info.root, git_root);
}

#[test]
fn test_message_serialization() {
    let msg = Message::new_user("Test message".to_string());
    let json = serde_json::to_string(&msg).unwrap();
    let deserialized: Message = serde_json::from_str(&json).unwrap();

    assert_eq!(msg.id(), deserialized.id());
    assert_eq!(msg.role(), deserialized.role());
    assert_eq!(msg.content(), deserialized.content());
}
