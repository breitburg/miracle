use crate::Chat;

/// Everything a shell needs to render. Plain data, cheap to clone.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct State {
    /// Newest first, by [`Chat::updated_at`].
    pub chats: Vec<Chat>,
    /// The [`Chat::id`] of the chat that is open, or `None` while the empty
    /// new chat is open.
    pub selected_chat_id: Option<u64>,
}
