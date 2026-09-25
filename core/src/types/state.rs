use crate::{Session, SessionViewState};

/// Everything a shell needs to render. Plain data, cheap to clone.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct State {
    /// Newest first, by [`Session::updated_at`].
    pub sessions: Vec<Session>,
    /// One per open window, in the order they opened.
    pub views: Vec<SessionViewState>,
}

impl State {
    pub fn session(&self, id: u64) -> Option<&Session> {
        self.sessions.iter().find(|session| session.id == id)
    }

    pub fn session_mut(&mut self, id: u64) -> Option<&mut Session> {
        self.sessions.iter_mut().find(|session| session.id == id)
    }

    pub fn view(&self, id: u64) -> Option<&SessionViewState> {
        self.views.iter().find(|view| view.id == id)
    }

    pub fn view_mut(&mut self, id: u64) -> Option<&mut SessionViewState> {
        self.views.iter_mut().find(|view| view.id == id)
    }
}
