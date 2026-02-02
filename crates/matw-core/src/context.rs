use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitInfo {
    pub branch: String,
    pub commit: String,
    pub root: PathBuf,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Context {
    working_dir: PathBuf,
    git_info: Option<GitInfo>,
    environment: HashMap<String, String>,
    claude_md: Option<String>,
}

impl Context {
    pub fn new(working_dir: PathBuf) -> Self {
        Self {
            working_dir,
            git_info: None,
            environment: HashMap::new(),
            claude_md: None,
        }
    }

    pub fn with_details(working_dir: PathBuf, git_info: Option<GitInfo>, claude_md: Option<String>) -> Self {
        Self {
            working_dir,
            git_info,
            environment: HashMap::new(),
            claude_md,
        }
    }

    pub fn working_dir(&self) -> &Path {
        &self.working_dir
    }

    pub fn git_info(&self) -> Option<&GitInfo> {
        self.git_info.as_ref()
    }

    pub fn set_git_info(&mut self, info: GitInfo) {
        self.git_info = Some(info);
    }

    pub fn environment(&self) -> &HashMap<String, String> {
        &self.environment
    }

    pub fn set_env(&mut self, key: String, value: String) {
        self.environment.insert(key, value);
    }

    pub fn set_environment(&mut self, env: HashMap<String, String>) {
        self.environment = env;
    }

    pub fn claude_md(&self) -> Option<&String> {
        self.claude_md.as_ref()
    }

    pub fn set_claude_md(&mut self, content: String) {
        self.claude_md = Some(content);
    }
}
