use std::time::SystemTime;

use crate::Model;

/// Every user intent a shell can send to the core. The shell gives the
/// time, so [`crate::reduce`] stays pure.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Action {
    /// Opens a view on the session, or on a new session for `None`.
    OpenView { session_id: Option<u64> },
    /// Closes the view, when its window closes.
    CloseView { view_id: u64 },
    /// Shows the session in the view, or a new session for `None`. The
    /// draft stays.
    ShowSession {
        view_id: u64,
        session_id: Option<u64>,
    },
    /// Switches the session to `model`.
    SetModel { session_id: u64, model: Model },
    /// Replaces the view's unsent text.
    EditDraft { view_id: u64, text: String },
    /// Sends the view's draft as a user message and clears the draft. The
    /// session moves to the top; a new session is created by its first
    /// message.
    SendMessage { view_id: u64, sent_at: SystemTime },
}
