use crate::Message;

/// The conversation of a [`crate::Session`].
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Chat {
    /// Oldest first.
    pub messages: Vec<Message>,
}
