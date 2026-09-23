use crate::{Action, Chat, Message, Role, State};

/// Applies `action` to `state`. Pure: all business rules live here.
pub fn reduce(mut state: State, action: Action) -> State {
    match action {
        Action::OpenNewChat => state.selected_chat_id = None,
        Action::SelectChat { id } => {
            if state.chats.iter().any(|chat| chat.id == id) {
                state.selected_chat_id = Some(id);
            }
        }
        Action::EditDraft { text } => state.draft = text,
        Action::SendMessage { sent_at } => {
            let content = state.draft.trim().to_owned();
            if content.is_empty() {
                return state;
            }
            let message = Message {
                role: Role::User,
                content,
            };
            match state.selected_chat_id {
                // The new chat is open: its first message creates it.
                None => {
                    let id = state.chats.iter().map(|chat| chat.id).max().unwrap_or(0) + 1;
                    state.chats.insert(
                        0,
                        Chat {
                            id,
                            title: title_from(&message.content),
                            updated_at: sent_at,
                            messages: vec![message],
                        },
                    );
                    state.selected_chat_id = Some(id);
                }
                Some(id) => {
                    let Some(position) = state.chats.iter().position(|chat| chat.id == id) else {
                        return state;
                    };
                    // The chat is now the newest, so it moves to the top.
                    let mut chat = state.chats.remove(position);
                    chat.messages.push(message);
                    chat.updated_at = sent_at;
                    state.chats.insert(0, chat);
                }
            }
            state.draft.clear();
        }
    }
    state
}

/// A new chat's title: the first line of its first message.
fn title_from(content: &str) -> String {
    content.lines().next().unwrap_or_default().trim().to_owned()
}

#[cfg(test)]
mod tests {
    use std::time::{Duration, SystemTime};

    use super::*;

    fn chat(id: u64, age_secs: u64) -> Chat {
        Chat {
            id,
            title: format!("Chat {id}"),
            updated_at: SystemTime::UNIX_EPOCH + Duration::from_secs(1000 - age_secs),
            messages: Vec::new(),
        }
    }

    fn state() -> State {
        State {
            chats: vec![chat(1, 0), chat(2, 10)],
            selected_chat_id: Some(2),
            ..Default::default()
        }
    }

    fn type_draft(state: State, text: &str) -> State {
        reduce(
            state,
            Action::EditDraft {
                text: text.to_owned(),
            },
        )
    }

    /// Types `content` into the composer and sends it.
    fn send(state: State, content: &str) -> State {
        reduce(
            type_draft(state, content),
            Action::SendMessage {
                sent_at: SystemTime::UNIX_EPOCH + Duration::from_secs(2000),
            },
        )
    }

    #[test]
    fn open_new_chat_clears_the_selection() {
        let s = reduce(state(), Action::OpenNewChat);
        assert_eq!(s.selected_chat_id, None);
        assert_eq!(s.chats, state().chats);
    }

    #[test]
    fn first_message_of_a_new_chat_creates_it() {
        let open = reduce(state(), Action::OpenNewChat);
        let s = send(open, "  Plan a trip\nto Lisbon ");
        let top = &s.chats[0];
        assert_eq!(top.id, 3);
        assert_eq!(top.title, "Plan a trip");
        assert_eq!(
            top.updated_at,
            SystemTime::UNIX_EPOCH + Duration::from_secs(2000)
        );
        assert_eq!(
            top.messages,
            [Message {
                role: Role::User,
                content: "Plan a trip\nto Lisbon".to_owned(),
            }]
        );
        assert_eq!(s.chats.len(), 3);
        assert_eq!(s.selected_chat_id, Some(3));
        assert_eq!(s.draft, "");
    }

    #[test]
    fn first_chat_in_an_empty_state_gets_id_one() {
        let s = send(State::default(), "Hello");
        assert_eq!(s.chats[0].id, 1);
    }

    #[test]
    fn selects_only_existing_chats() {
        let s = reduce(state(), Action::SelectChat { id: 1 });
        assert_eq!(s.selected_chat_id, Some(1));
        assert_eq!(reduce(s.clone(), Action::SelectChat { id: 9 }), s);
    }

    #[test]
    fn send_adds_trimmed_user_message_and_moves_chat_to_top() {
        let s = send(state(), "  Hello \n");
        let top = &s.chats[0];
        assert_eq!(top.id, 2);
        assert_eq!(
            top.updated_at,
            SystemTime::UNIX_EPOCH + Duration::from_secs(2000)
        );
        assert_eq!(
            top.messages,
            [Message {
                role: Role::User,
                content: "Hello".to_owned(),
            }]
        );
    }

    #[test]
    fn send_keeps_blank_drafts_and_drafts_for_unknown_chats() {
        let blank = send(state(), " \n ");
        assert_eq!(blank.chats, state().chats);
        assert_eq!(blank.draft, " \n ");
        let open = reduce(state(), Action::OpenNewChat);
        assert_eq!(send(open.clone(), " \n ").chats, open.chats);
        let unknown = State {
            selected_chat_id: Some(9),
            ..state()
        };
        let s = send(unknown.clone(), "Hello");
        assert_eq!(s.chats, unknown.chats);
        assert_eq!(s.draft, "Hello");
    }

    #[test]
    fn draft_survives_switching_chats() {
        let typed = type_draft(state(), "Hi");
        assert_eq!(reduce(typed, Action::SelectChat { id: 1 }).draft, "Hi");
    }
}
