use crate::{Session, SessionViewState};

/// Everything a shell needs to render. Plain data, cheap to clone.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct State {
    /// Newest first, by [`Session::updated_at`].
    pub sessions: Vec<Session>,
    /// One per open window, in the order they opened.
    pub views: Vec<SessionViewState>,
}
