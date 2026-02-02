//! Application state management
//!
//! Manages the application state including session, input, messages, and UI state.

use matw_agent::Agent;
use matw_ai::AIProvider;
use matw_core::Message;
use matw_core::Session;
use matw_tools::Tool;
use std::sync::Arc;

/// Main application state
pub struct App<P: AIProvider> {
    /// Current session
    pub session: Session,
    /// Current input buffer
    pub input: String,
    /// All messages in the conversation
    pub messages: Vec<Message>,
    /// Whether the app should quit
    pub should_quit: bool,
    /// Available tools
    pub tools: Vec<Arc<dyn Tool>>,
    /// Current status message
    pub status: String,
    /// Optional agent for AI processing
    pub agent: Option<Agent<P>>,
}

impl<P: AIProvider> App<P> {
    /// Create a new application
    pub fn new(session: Session, tools: Vec<Arc<dyn Tool>>) -> Self {
        Self {
            session,
            input: String::new(),
            messages: Vec::new(),
            should_quit: false,
            tools,
            status: "Ready".to_string(),
            agent: None,
        }
    }

    /// Set the agent for AI processing
    pub fn with_agent(mut self, agent: Agent<P>) -> Self {
        self.agent = Some(agent);
        self
    }

    /// Handle character input
    pub fn handle_input(&mut self, c: char) {
        self.input.push(c);
    }

    /// Handle backspace
    pub fn handle_backspace(&mut self) {
        self.input.pop();
    }

    /// Submit the current input
    pub async fn submit_input(&mut self) {
        if self.input.is_empty() {
            return;
        }

        let msg = Message::new_user(self.input.clone());
        self.messages.push(msg.clone());
        self.session.add_message(msg);
        self.input.clear();
        self.status = "Processing...".to_string();

        // Run agent if available
        if let Some(ref agent) = self.agent {
            if let Err(e) = agent.process(&mut self.session).await {
                self.status = format!("Error: {}", e);
                self.messages.push(Message::new_assistant(format!("Error: {}", e)));
            } else {
                self.status = "Ready".to_string();
                // Update messages from session
                self.messages = self.session.messages().to_vec();
            }
        }
    }

    /// Quit the application
    pub fn quit(&mut self) {
        self.should_quit = true;
    }

    /// Get the input position (cursor position)
    pub fn cursor_position(&self) -> usize {
        self.input.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use matw_ai::providers::GLMProvider;
    use matw_tools::tools::ReadTool;
    use tempfile::TempDir;

    #[test]
    fn test_app_creation() {
        let temp = TempDir::new().unwrap();
        let session = Session::new(temp.path().to_path_buf());
        let tools: Vec<Arc<dyn Tool>> = vec![Arc::new(ReadTool::new())];

        let app: App<GLMProvider> = App::new(session, tools);

        assert_eq!(app.input, "");
        assert!(!app.should_quit);
        assert_eq!(app.status, "Ready");
        assert_eq!(app.messages.len(), 0);
    }

    #[test]
    fn test_handle_input() {
        let temp = TempDir::new().unwrap();
        let session = Session::new(temp.path().to_path_buf());
        let mut app: App<GLMProvider> = App::new(session, vec![]);

        app.handle_input('h');
        app.handle_input('i');
        app.handle_input('!');

        assert_eq!(app.input, "hi!");
    }

    #[test]
    fn test_handle_backspace() {
        let temp = TempDir::new().unwrap();
        let session = Session::new(temp.path().to_path_buf());
        let mut app: App<GLMProvider> = App::new(session, vec![]);

        app.input = "hello".to_string();
        app.handle_backspace();

        assert_eq!(app.input, "hell");
    }

    #[tokio::test]
    async fn test_submit_input() {
        let temp = TempDir::new().unwrap();
        let session = Session::new(temp.path().to_path_buf());
        let mut app: App<GLMProvider> = App::new(session, vec![]);

        app.input = "test message".to_string();
        app.submit_input().await;

        assert_eq!(app.input, "");
        assert_eq!(app.messages.len(), 1);
        assert_eq!(app.status, "Processing...");
        assert_eq!(app.session.message_count(), 1);
    }

    #[test]
    fn test_quit() {
        let temp = TempDir::new().unwrap();
        let session = Session::new(temp.path().to_path_buf());
        let mut app: App<GLMProvider> = App::new(session, vec![]);

        app.quit();

        assert!(app.should_quit);
    }

    #[test]
    fn test_cursor_position() {
        let temp = TempDir::new().unwrap();
        let session = Session::new(temp.path().to_path_buf());
        let mut app: App<GLMProvider> = App::new(session, vec![]);

        assert_eq!(app.cursor_position(), 0);

        app.input = "hello".to_string();
        assert_eq!(app.cursor_position(), 5);
    }
}
