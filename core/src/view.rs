/// What one window shows: a chat, or a new chat that its first message
/// creates, and the unsent text. Any number of views can show the same
/// chat; they share its messages and keep their own drafts.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ChatViewState {
    pub id: u64,
    /// The [`crate::Chat::id`] shown, or `None` for a new chat.
    pub chat_id: Option<u64>,
    /// The unsent text in the composer.
    pub draft: String,
}
