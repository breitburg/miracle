use std::time::SystemTime;

use crate::Message;

/// A conversation, shown in the sidebar by its title.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Chat {
    pub id: u64,
    pub title: String,
    /// When the chat last changed. Shells group chats by this date.
    pub updated_at: SystemTime,
    /// Oldest first.
    pub messages: Vec<Message>,
}
