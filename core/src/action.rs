use std::time::SystemTime;

/// Every user intent a shell can send to the core. The shell gives the
/// time, so [`crate::reduce`] stays pure.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Action {
    /// Opens a view on the chat, or on a new chat for `None`.
    OpenView { chat_id: Option<u64> },
    /// Closes the view, when its window closes.
    CloseView { view_id: u64 },
    /// Shows the chat in the view, or a new chat for `None`. The draft stays.
    ShowChat { view_id: u64, chat_id: Option<u64> },
    /// Replaces the view's unsent text.
    EditDraft { view_id: u64, text: String },
    /// Sends the view's draft as a user message and clears the draft. The
    /// chat moves to the top; a new chat is created by its first message.
    SendMessage { view_id: u64, sent_at: SystemTime },
}
