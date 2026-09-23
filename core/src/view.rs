/// What one window shows: a session, or a new session that its first
/// message creates, and the unsent text. Any number of views can show the
/// same session; they share its chat and keep their own drafts.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct SessionViewState {
    pub id: u64,
    /// The [`crate::Session::id`] shown, or `None` for a new session.
    pub session_id: Option<u64>,
    /// The unsent text in the composer.
    pub draft: String,
}
