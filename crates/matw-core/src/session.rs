use crate::{context::Context, message::Message};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SessionState {
    Active,
    Paused,
    Closed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    id: Uuid,
    messages: Vec<Message>,
    context: Context,
    state: SessionState,
}

impl Session {
    pub fn new(working_dir: PathBuf) -> Self {
        Self {
            id: Uuid::new_v4(),
            messages: Vec::new(),
            context: Context::new(working_dir),
            state: SessionState::Active,
        }
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn context(&self) -> &Context {
        &self.context
    }

    pub fn context_mut(&mut self) -> &mut Context {
        &mut self.context
    }

    pub fn messages(&self) -> &[Message] {
        &self.messages
    }

    pub fn message_count(&self) -> usize {
        self.messages.len()
    }

    pub fn state(&self) -> SessionState {
        self.state
    }

    pub fn is_active(&self) -> bool {
        self.state == SessionState::Active
    }

    pub fn add_message(&mut self, message: Message) {
        self.messages.push(message);
    }

    pub fn close(&mut self) {
        self.state = SessionState::Closed;
    }

    pub fn pause(&mut self) {
        self.state = SessionState::Paused;
    }

    pub fn resume(&mut self) {
        self.state = SessionState::Active;
    }

    pub fn to_ai_request(&self) -> Vec<&Message> {
        self.messages.iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Message;

    #[test]
    fn test_new_session() {
        let session = Session::new(PathBuf::from("/tmp"));
        assert_eq!(session.message_count(), 0);
        assert!(session.is_active());
    }

    #[test]
    fn test_add_message() {
        let mut session = Session::new(PathBuf::from("/tmp"));
        session.add_message(Message::new_user("hello".to_string()));
        assert_eq!(session.message_count(), 1);
    }

    #[test]
    fn test_context_working_dir() {
        let session = Session::new(PathBuf::from("/home/user/project"));
        assert_eq!(session.context().working_dir(), PathBuf::from("/home/user/project"));
    }

    #[test]
    fn test_session_id_is_unique() {
        let s1 = Session::new(PathBuf::from("/tmp"));
        let s2 = Session::new(PathBuf::from("/tmp"));
        assert_ne!(s1.id(), s2.id());
    }

    #[test]
    fn test_close_session() {
        let mut session = Session::new(PathBuf::from("/tmp"));
        session.close();
        assert!(!session.is_active());
    }
}
