use crate::{Action, Chat, Message, Role, Session, SessionViewState, State};

/// Applies `action` to `state`. Pure: all business rules live here.
pub fn reduce(mut state: State, action: Action) -> State {
    match action {
        Action::OpenView { session_id } => {
            let id = state.views.iter().map(|view| view.id).max().unwrap_or(0) + 1;
            state.views.push(SessionViewState {
                id,
                session_id: session_id.filter(|&session_id| has_session(&state, session_id)),
                draft: String::new(),
            });
        }
        Action::CloseView { view_id } => state.views.retain(|view| view.id != view_id),
        Action::ShowSession {
            view_id,
            session_id,
        } => {
            let exists = session_id.is_none_or(|session_id| has_session(&state, session_id));
            if let Some(view) = view_mut(&mut state, view_id)
                && exists
            {
                view.session_id = session_id;
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
            let session = match view.session_id {
                Some(session_id) => {
                    let Some(position) = state
                        .sessions
                        .iter()
                        .position(|session| session.id == session_id)
                    else {
                        return state;
                    };
                    let mut session = state.sessions.remove(position);
                    session.chat.messages.push(message);
                    session
                }
                // A new session: its first message creates it.
                None => Session {
                    id: state
                        .sessions
                        .iter()
                        .map(|session| session.id)
                        .max()
                        .unwrap_or(0)
                        + 1,
                    title: title_from(&message.content),
                    updated_at: sent_at,
                    chat: Chat {
                        messages: vec![message],
                    },
                },
            };
            let session_id = session.id;
            // The session is now the newest, so it moves to the top.
            state.sessions.insert(
                0,
                Session {
                    updated_at: sent_at,
                    ..session
                },
            );
            let view = view_mut(&mut state, view_id).expect("the view was found above");
            view.session_id = Some(session_id);
            view.draft.clear();
        }
    }
    state
}

fn has_session(state: &State, session_id: u64) -> bool {
    state
        .sessions
        .iter()
        .any(|session| session.id == session_id)
}

fn view_mut(state: &mut State, view_id: u64) -> Option<&mut SessionViewState> {
    state.views.iter_mut().find(|view| view.id == view_id)
}

/// A new session's title: the first line of its first message.
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

    fn session(id: u64, age_secs: u64) -> Session {
        Session {
            id,
            title: format!("Session {id}"),
            updated_at: at(1000 - age_secs),
            chat: Chat::default(),
        }
    }

    /// Two sessions and one view (id 1) on session 2.
    fn state() -> State {
        State {
            sessions: vec![session(1, 0), session(2, 10)],
            views: vec![SessionViewState {
                id: 1,
                session_id: Some(2),
                draft: String::new(),
            }],
        }
    }

    fn view(state: &State, view_id: u64) -> &SessionViewState {
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
    fn views_open_on_existing_sessions_and_close() {
        let s = reduce(
            state(),
            Action::OpenView {
                session_id: Some(1),
            },
        );
        assert_eq!(view(&s, 2).session_id, Some(1));
        let s = reduce(
            s,
            Action::OpenView {
                session_id: Some(9),
            },
        );
        assert_eq!(view(&s, 3).session_id, None);
        let s = reduce(s, Action::CloseView { view_id: 2 });
        assert_eq!(
            s.views.iter().map(|view| view.id).collect::<Vec<_>>(),
            [1, 3]
        );
    }

    #[test]
    fn show_session_switches_only_to_existing_sessions() {
        let s = reduce(
            state(),
            Action::ShowSession {
                view_id: 1,
                session_id: Some(1),
            },
        );
        assert_eq!(view(&s, 1).session_id, Some(1));
        assert_eq!(
            reduce(
                s.clone(),
                Action::ShowSession {
                    view_id: 1,
                    session_id: Some(9)
                }
            ),
            s
        );
        let s = reduce(
            s,
            Action::ShowSession {
                view_id: 1,
                session_id: None,
            },
        );
        assert_eq!(view(&s, 1).session_id, None);
    }

    #[test]
    fn first_message_of_a_new_session_creates_it() {
        let open = reduce(
            state(),
            Action::ShowSession {
                view_id: 1,
                session_id: None,
            },
        );
        let s = send(open, 1, "  Plan a trip\nto Lisbon ");
        let top = &s.sessions[0];
        assert_eq!((top.id, top.title.as_str()), (3, "Plan a trip"));
        assert_eq!(top.updated_at, at(2000));
        assert_eq!(
            top.chat.messages,
            [Message {
                role: Role::User,
                content: "Plan a trip\nto Lisbon".to_owned(),
            }]
        );
        assert_eq!(view(&s, 1).session_id, Some(3));
        assert_eq!(view(&s, 1).draft, "");
    }

    #[test]
    fn send_adds_trimmed_message_and_moves_session_to_top() {
        let s = send(state(), 1, "  Hello \n");
        let top = &s.sessions[0];
        assert_eq!(top.id, 2);
        assert_eq!(top.updated_at, at(2000));
        assert_eq!(
            top.chat.messages.last().map(|m| m.content.as_str()),
            Some("Hello")
        );
    }

    #[test]
    fn views_on_one_session_share_its_chat_but_not_drafts() {
        let s = reduce(
            state(),
            Action::OpenView {
                session_id: Some(2),
            },
        );
        let s = type_draft(s, 2, "Later");
        let s = send(s, 1, "Now");
        assert_eq!(view(&s, 2).session_id, Some(2));
        assert_eq!(view(&s, 2).draft, "Later");
        assert_eq!(s.sessions[0].chat.messages.len(), 1);
    }

    #[test]
    fn send_keeps_blank_drafts_and_ignores_unknown_views() {
        let blank = send(state(), 1, " \n ");
        assert_eq!(blank.sessions, state().sessions);
        assert_eq!(view(&blank, 1).draft, " \n ");
        assert_eq!(send(state(), 9, "Hello"), state());
    }
}
