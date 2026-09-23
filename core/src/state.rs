use crate::{Chat, ChatViewState};

/// Everything a shell needs to render. Plain data, cheap to clone.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct State {
    /// Newest first, by [`Chat::updated_at`].
    pub chats: Vec<Chat>,
    /// One per open window, in the order they opened.
    pub views: Vec<ChatViewState>,
}
