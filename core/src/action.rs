use std::time::SystemTime;

/// Every user intent a shell can send to the core.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Action {
    /// Opens an empty new chat. Its first message creates it.
    OpenNewChat,
    /// Opens the chat with this [`crate::Chat::id`].
    SelectChat { id: u64 },
    /// Replaces the composer text.
    EditDraft { text: String },
    /// Sends the draft as a user message to the selected chat, or creates a
    /// chat with it if the new chat is open, and clears the draft. The
    /// shell gives the time, so [`crate::reduce`] stays pure.
    SendMessage { sent_at: SystemTime },
}
