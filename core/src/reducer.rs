use crate::{Action, Chat, ChatViewState, Message, Role, State};

/// Applies `action` to `state`. Pure: all business rules live here.
pub fn reduce(mut state: State, action: Action) -> State {
    match action {
        Action::OpenView { chat_id } => {
            let id = state.views.iter().map(|view| view.id).max().unwrap_or(0) + 1;
            state.views.push(ChatViewState {
                id,
                chat_id: chat_id.filter(|&chat_id| has_chat(&state, chat_id)),
                draft: String::new(),
            });
        }
        Action::CloseView { view_id } => state.views.retain(|view| view.id != view_id),
        Action::ShowChat { view_id, chat_id } => {
            let exists = chat_id.is_none_or(|chat_id| has_chat(&state, chat_id));
            if let Some(view) = view_mut(&mut state, view_id)
                && exists
            {
                view.chat_id = chat_id;
            }
        }
        Action::EditDraft { view_id, text } => {
            if let Some(view) = view_mut(&mut state, view_id) {
                view.draft = text;
            }
        }
        Action::SendMessage { view_id, sent_at } => {
            let Some(view) = state.views.iter().find(|view| view.id == view_id) else {
                return state;
            };
            let content = view.draft.trim().to_owned();
            if content.is_empty() {
                return state;
            }
            let message = Message {
                role: Role::User,
                content,
            };
            let chat = match view.chat_id {
                Some(chat_id) => {
                    let Some(position) = state.chats.iter().position(|chat| chat.id == chat_id)
                    else {
                        return state;
                    };
                    let mut chat = state.chats.remove(position);
                    chat.messages.push(message);
                    chat
                }
                // A new chat: its first message creates it.
                None => Chat {
                    id: state.chats.iter().map(|chat| chat.id).max().unwrap_or(0) + 1,
                    title: title_from(&message.content),
                    updated_at: sent_at,
                    messages: vec![message],
                },
            };
            let chat_id = chat.id;
            // The chat is now the newest, so it moves to the top.
            state.chats.insert(
                0,
                Chat {
                    updated_at: sent_at,
                    ..chat
                },
            );
            let view = view_mut(&mut state, view_id).expect("the view was found above");
            view.chat_id = Some(chat_id);
            view.draft.clear();
        }
    }
    state
}

fn has_chat(state: &State, chat_id: u64) -> bool {
    state.chats.iter().any(|chat| chat.id == chat_id)
}

fn view_mut(state: &mut State, view_id: u64) -> Option<&mut ChatViewState> {
    state.views.iter_mut().find(|view| view.id == view_id)
}

/// A new chat's title: the first line of its first message.
fn title_from(content: &str) -> String {
    content.lines().next().unwrap_or_default().trim().to_owned()
}

#[cfg(test)]
mod tests {
    use std::time::{Duration, SystemTime};

    use super::*;

    fn at(secs: u64) -> SystemTime {
        SystemTime::UNIX_EPOCH + Duration::from_secs(secs)
    }

    fn chat(id: u64, age_secs: u64) -> Chat {
        Chat {
            id,
            title: format!("Chat {id}"),
            updated_at: at(1000 - age_secs),
            messages: Vec::new(),
        }
    }

    /// Two chats and one view (id 1) on chat 2.
    fn state() -> State {
        State {
            chats: vec![chat(1, 0), chat(2, 10)],
            views: vec![ChatViewState {
                id: 1,
                chat_id: Some(2),
                draft: String::new(),
            }],
        }
    }

    fn view(state: &State, view_id: u64) -> &ChatViewState {
        state
            .views
            .iter()
            .find(|view| view.id == view_id)
            .expect("the view exists")
    }

    fn type_draft(state: State, view_id: u64, text: &str) -> State {
        reduce(
            state,
            Action::EditDraft {
                view_id,
                text: text.to_owned(),
            },
        )
    }

    /// Types `content` into the view's composer and sends it.
    fn send(state: State, view_id: u64, content: &str) -> State {
        reduce(
            type_draft(state, view_id, content),
            Action::SendMessage {
                view_id,
                sent_at: at(2000),
            },
        )
    }

    #[test]
    fn views_open_on_existing_chats_and_close() {
        let s = reduce(state(), Action::OpenView { chat_id: Some(1) });
        assert_eq!(view(&s, 2).chat_id, Some(1));
        let s = reduce(s, Action::OpenView { chat_id: Some(9) });
        assert_eq!(view(&s, 3).chat_id, None);
        let s = reduce(s, Action::CloseView { view_id: 2 });
        assert_eq!(
            s.views.iter().map(|view| view.id).collect::<Vec<_>>(),
            [1, 3]
        );
    }

    #[test]
    fn show_chat_switches_only_to_existing_chats() {
        let s = reduce(
            state(),
            Action::ShowChat {
                view_id: 1,
                chat_id: Some(1),
            },
        );
        assert_eq!(view(&s, 1).chat_id, Some(1));
        assert_eq!(
            reduce(
                s.clone(),
                Action::ShowChat {
                    view_id: 1,
                    chat_id: Some(9)
                }
            ),
            s
        );
        let s = reduce(
            s,
            Action::ShowChat {
                view_id: 1,
                chat_id: None,
            },
        );
        assert_eq!(view(&s, 1).chat_id, None);
    }

    #[test]
    fn first_message_of_a_new_chat_creates_it() {
        let open = reduce(
            state(),
            Action::ShowChat {
                view_id: 1,
                chat_id: None,
            },
        );
        let s = send(open, 1, "  Plan a trip\nto Lisbon ");
        let top = &s.chats[0];
        assert_eq!((top.id, top.title.as_str()), (3, "Plan a trip"));
        assert_eq!(top.updated_at, at(2000));
        assert_eq!(
            top.messages,
            [Message {
                role: Role::User,
                content: "Plan a trip\nto Lisbon".to_owned(),
            }]
        );
        assert_eq!(view(&s, 1).chat_id, Some(3));
        assert_eq!(view(&s, 1).draft, "");
    }

    #[test]
    fn send_adds_trimmed_message_and_moves_chat_to_top() {
        let s = send(state(), 1, "  Hello \n");
        let top = &s.chats[0];
        assert_eq!(top.id, 2);
        assert_eq!(top.updated_at, at(2000));
        assert_eq!(
            top.messages.last().map(|m| m.content.as_str()),
            Some("Hello")
        );
    }

    #[test]
    fn views_on_one_chat_share_messages_but_not_drafts() {
        let s = reduce(state(), Action::OpenView { chat_id: Some(2) });
        let s = type_draft(s, 2, "Later");
        let s = send(s, 1, "Now");
        assert_eq!(view(&s, 2).chat_id, Some(2));
        assert_eq!(view(&s, 2).draft, "Later");
        assert_eq!(s.chats[0].messages.len(), 1);
    }

    #[test]
    fn send_keeps_blank_drafts_and_ignores_unknown_views() {
        let blank = send(state(), 1, " \n ");
        assert_eq!(blank.chats, state().chats);
        assert_eq!(view(&blank, 1).draft, " \n ");
        assert_eq!(send(state(), 9, "Hello"), state());
    }
}
