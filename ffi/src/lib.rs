//! UniFFI facade over `miracle_core`, for every non-Rust shell.
//!
//! The core stays FFI-free: its plain types are re-declared here as UniFFI
//! "remote" types. The field and variant lists must match the core exactly;
//! the compiler rejects any drift.

use std::sync::Arc;
use std::time::SystemTime;

use miracle_core::{
    Action, Chat, Message, Period, Role, Session, SessionSection, SessionSummary, SessionViewState,
    State,
};

uniffi::setup_scaffolding!();

#[uniffi::remote(Record)]
pub struct Message {
    pub role: Role,
    pub content: String,
}

#[uniffi::remote(Enum)]
pub enum Role {
    User,
    Assistant,
}

#[uniffi::remote(Record)]
pub struct Chat {
    pub messages: Vec<Message>,
}

#[uniffi::remote(Record)]
pub struct Session {
    pub id: u64,
    pub title: String,
    pub updated_at: SystemTime,
    pub chat: Chat,
}

#[uniffi::remote(Record)]
pub struct SessionViewState {
    pub id: u64,
    pub session_id: Option<u64>,
    pub draft: String,
}

#[uniffi::remote(Record)]
pub struct State {
    pub sessions: Vec<Session>,
    pub views: Vec<SessionViewState>,
}

#[uniffi::remote(Enum)]
pub enum Action {
    OpenView {
        session_id: Option<u64>,
    },
    CloseView {
        view_id: u64,
    },
    ShowSession {
        view_id: u64,
        session_id: Option<u64>,
    },
    EditDraft {
        view_id: u64,
        text: String,
    },
    SendMessage {
        view_id: u64,
        sent_at: SystemTime,
    },
}

#[uniffi::remote(Enum)]
pub enum Period {
    Today,
    Yesterday,
    PreviousSevenDays,
    PreviousThirtyDays,
    Month { month: u8 },
    Year { year: i16 },
}

#[uniffi::remote(Record)]
pub struct SessionSection {
    pub period: Period,
    pub sessions: Vec<SessionSummary>,
}

#[uniffi::remote(Record)]
pub struct SessionSummary {
    pub id: u64,
    pub title: String,
}

/// Foreign handle to the core [`miracle_core::Store`].
#[derive(uniffi::Object)]
pub struct Store(miracle_core::Store);

#[uniffi::export]
impl Store {
    #[uniffi::constructor]
    pub fn new() -> Arc<Self> {
        Arc::new(Self(miracle_core::Store::new()))
    }

    /// A snapshot of the current state.
    pub fn state(&self) -> State {
        self.0.state()
    }

    /// Applies `action` and returns the new state.
    pub fn dispatch(&self, action: Action) -> State {
        self.0.dispatch(action)
    }

    /// Opens a view on the session (a new session for `None`) and returns
    /// its id.
    pub fn open_view(&self, session_id: Option<u64>) -> u64 {
        self.0.open_view(session_id)
    }

    /// The sessions grouped for the sidebar, as seen at `now`.
    pub fn session_sections(&self, now: SystemTime) -> Vec<SessionSection> {
        self.0.session_sections(now)
    }
}
