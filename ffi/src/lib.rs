//! UniFFI facade over `miracle_core`, for every non-Rust shell.
//!
//! The core stays FFI-free: its plain types are re-declared here as UniFFI
//! "remote" types. The field and variant lists must match the core exactly;
//! the compiler rejects any drift.

use std::sync::Arc;
use std::time::SystemTime;

use miracle_core::{Action, Chat, ChatSection, ChatSummary, Message, Period, Role, State};

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
    pub id: u64,
    pub title: String,
    pub updated_at: SystemTime,
    pub messages: Vec<Message>,
}

#[uniffi::remote(Record)]
pub struct State {
    pub chats: Vec<Chat>,
    pub selected_chat_id: Option<u64>,
    pub draft: String,
}

#[uniffi::remote(Enum)]
pub enum Action {
    OpenNewChat,
    SelectChat {
        id: u64,
    },
    EditDraft {
        text: String,
    },
    SendMessage {
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
pub struct ChatSection {
    pub period: Period,
    pub chats: Vec<ChatSummary>,
}

#[uniffi::remote(Record)]
pub struct ChatSummary {
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

    /// The chats grouped for the sidebar, as seen at `now`.
    pub fn chat_sections(&self, now: SystemTime) -> Vec<ChatSection> {
        self.0.chat_sections(now)
    }
}
