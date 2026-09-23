use std::mem;
use std::sync::{Mutex, PoisonError};
use std::time::{Duration, SystemTime};

use crate::section::chat_sections;
use crate::{Action, Chat, ChatSection, Message, Role, State, reduce};

/// Owns the current [`State`] and applies [`Action`]s to it.
///
/// Thread-safe (`Send + Sync`), so any shell or FFI layer can share it.
#[derive(Debug)]
pub struct Store {
    state: Mutex<State>,
}

impl Store {
    /// A store with sample chats, until the core can create them. The newest
    /// chat is open.
    pub fn new() -> Self {
        const HOUR: u64 = 60 * 60;
        const DAY: u64 = 24 * HOUR;
        let now = SystemTime::now();
        let mut chats: Vec<_> = [
            (
                "Welcome",
                0,
                "Hi! What can you do?",
                "I can help you write, plan, and learn. Ask me anything.",
            ),
            (
                "Project ideas",
                2 * HOUR,
                "Give me an idea for a weekend project.",
                "Build a small weather station with a Raspberry Pi and log the data.",
            ),
            (
                "Weekend plans",
                DAY,
                "Where can I go hiking near the city?",
                "Try the river trail: it is 12 km long and mostly flat.",
            ),
            (
                "Book recommendations",
                4 * DAY,
                "Recommend a science fiction book.",
                "Read \"The Left Hand of Darkness\" by Ursula K. Le Guin.",
            ),
            (
                "Trip to Lisbon",
                12 * DAY,
                "What should I see in Lisbon?",
                "Visit Alfama, ride tram 28, and see the sunset from a miradouro.",
            ),
            (
                "Tax return",
                70 * DAY,
                "Which documents do I need for my tax return?",
                "You need your income statements, receipts for deductions, and your ID.",
            ),
            (
                "Moving checklist",
                400 * DAY,
                "Make a short moving checklist.",
                "Book movers, pack by room, change your address, and read the meters.",
            ),
        ]
        .into_iter()
        .zip(1..)
        .map(|((title, age, prompt, reply), id)| Chat {
            id,
            title: title.to_owned(),
            updated_at: now - Duration::from_secs(age),
            messages: vec![
                Message {
                    role: Role::User,
                    content: prompt.to_owned(),
                },
                Message {
                    role: Role::Assistant,
                    content: reply.to_owned(),
                },
            ],
        })
        .collect();
        chats.sort_by(|a, b| b.updated_at.cmp(&a.updated_at));
        let selected_chat_id = chats.first().map(|chat| chat.id);
        Self {
            state: Mutex::new(State {
                chats,
                selected_chat_id,
                ..Default::default()
            }),
        }
    }

    /// A snapshot of the current state.
    pub fn state(&self) -> State {
        self.state
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .clone()
    }

    /// Applies `action` and returns the new state.
    pub fn dispatch(&self, action: Action) -> State {
        let mut state = self.state.lock().unwrap_or_else(PoisonError::into_inner);
        *state = reduce(mem::take(&mut state), action);
        state.clone()
    }

    /// The chats grouped for the sidebar, by the local calendar day they
    /// were last updated, as seen at `now`.
    pub fn chat_sections(&self, now: SystemTime) -> Vec<ChatSection> {
        let state = self.state.lock().unwrap_or_else(PoisonError::into_inner);
        chat_sections(&state.chats, now)
    }
}

impl Default for Store {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_chats_are_newest_first() {
        let chats = Store::new().state().chats;
        assert!(!chats.is_empty());
        assert!(chats.is_sorted_by(|a, b| a.updated_at >= b.updated_at));
    }
}
