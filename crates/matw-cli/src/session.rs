use anyhow::Result;
use matw_core::{Context, GitInfo, Session};
use std::path::{Path, PathBuf};
use std::process::Command;

/// Detect Git repository information
pub fn detect_git_info(dir: &Path) -> Option<GitInfo> {
    let output = Command::new("git")
        .args(["-C", dir.to_str()?, "rev-parse", "--git-dir"])
        .output()
        .ok()?;

    if !output.status.success() {
        return None;
    }

    let _git_dir = String::from_utf8_lossy(&output.stdout).trim().to_string();

    // Get current branch
    let branch = Command::new("git")
        .args(["-C", dir.to_str()?, "rev-parse", "--abbrev-ref", "HEAD"])
        .output()
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .map(|s| s.trim().to_string())
        .unwrap_or_else(|| "HEAD".to_string());

    // Get current commit
    let commit = Command::new("git")
        .args(["-C", dir.to_str()?, "rev-parse", "HEAD"])
        .output()
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .map(|s| s.trim().to_string())
        .unwrap_or_default();

    // Get git root
    let root = Command::new("git")
        .args(["-C", dir.to_str()?, "rev-parse", "--show-toplevel"])
        .output()
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .map(|s| PathBuf::from(s.trim()))
        .unwrap_or_else(|| dir.to_path_buf());

    Some(GitInfo {
        branch,
        commit,
        root,
    })
}

/// Load CLAUDE.md content if it exists
pub fn load_claude_md(git_root: &Path) -> Option<String> {
    let claude_md_path = git_root.join("CLAUDE.md");
    std::fs::read_to_string(claude_md_path).ok()
}

/// Initialize a new session with context
pub fn initialize_session(working_dir: PathBuf) -> Result<Session> {
    let git_info = detect_git_info(&working_dir);

    let claude_md = if let Some(ref git) = git_info {
        load_claude_md(&git.root)
    } else {
        None
    };

    let mut context = Context::with_details(working_dir.clone(), git_info, claude_md);
    context.set_environment(std::env::vars().collect());

    let session = Session::with_context(context);
    Ok(session)
}

#[cfg(test)]
mod tests {
    use super::*;
    use matw_core::SessionState;

    #[test]
    fn test_initialize_session() {
        let temp = std::env::temp_dir();
        let session = initialize_session(temp.clone()).unwrap();

        assert_eq!(session.state(), SessionState::Active);
        assert_eq!(session.context().working_dir(), temp);
    }

    #[test]
    fn test_initialize_session_with_git() {
        let temp = std::env::temp_dir().join("matw-test-git");
        std::fs::create_dir_all(&temp).unwrap();

        // Initialize git repo
        Command::new("git")
            .args(["-C", temp.to_str().unwrap(), "init"])
            .output()
            .unwrap();

        Command::new("git")
            .args([
                "-C",
                temp.to_str().unwrap(),
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

        let session = initialize_session(temp.clone()).unwrap();

        let git_info = session.context().git_info();
        assert!(git_info.is_some());
        let git_info = git_info.unwrap();
        // Git default branch is "main" on newer versions, "main" or "master" are both valid
        assert!(git_info.branch == "main" || git_info.branch == "master");

        // Cleanup
        std::fs::remove_dir_all(temp).ok();
    }
}
