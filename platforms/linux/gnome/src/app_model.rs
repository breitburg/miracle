//! Thin GObject adapter over the Rust `Store`, the GTK counterpart of the
//! Swift `AppModel`. One model serves every window.
//!
//! Holds no business rules: it sends actions to the core, keeps the
//! resulting state and tells widgets what changed, through the
//! `chats-changed` and `views-changed` signals.

use std::time::SystemTime;

use gtk::glib;
use gtk::glib::closure_local;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use miracle_core::{Action, Chat, ChatSection, ChatViewState, State};

mod imp {
    use std::cell::RefCell;
    use std::sync::OnceLock;

    use gtk::glib;
    use gtk::glib::subclass::Signal;
    use gtk::subclass::prelude::*;
    use miracle_core::{State, Store};

    #[derive(Default)]
    pub struct AppModel {
        pub(super) store: Store,
        /// The state the widgets last heard about.
        pub(super) state: RefCell<State>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for AppModel {
        const NAME: &'static str = "MiracleAppModel";
        type Type = super::AppModel;
    }

    impl ObjectImpl for AppModel {
        fn signals() -> &'static [Signal] {
            static SIGNALS: OnceLock<Vec<Signal>> = OnceLock::new();
            SIGNALS.get_or_init(|| {
                vec![
                    // The chats or their order changed.
                    Signal::builder("chats-changed").build(),
                    // A view opened, closed, or changed its chat or draft.
                    Signal::builder("views-changed").build(),
                ]
            })
        }

        fn constructed(&self) {
            self.parent_constructed();
            self.state.replace(self.store.state());
        }
    }
}

glib::wrapper! {
    pub struct AppModel(ObjectSubclass<imp::AppModel>);
}

impl Default for AppModel {
    fn default() -> Self {
        glib::Object::new()
    }
}

impl AppModel {
    pub fn send(&self, action: Action) {
        let state = self.imp().store.dispatch(action);
        self.apply(state);
    }

    /// Opens a view on the chat (a new chat for `None`) and returns its id,
    /// for the window that shows it.
    pub fn open_view(&self, chat_id: Option<u64>) -> u64 {
        let id = self.imp().store.open_view(chat_id);
        self.apply(self.imp().store.state());
        id
    }

    /// Newest first.
    pub fn chats(&self) -> Vec<Chat> {
        self.imp().state.borrow().chats.clone()
    }

    pub fn chat(&self, id: u64) -> Option<Chat> {
        let state = self.imp().state.borrow();
        state.chats.iter().find(|chat| chat.id == id).cloned()
    }

    pub fn view(&self, id: u64) -> Option<ChatViewState> {
        let state = self.imp().state.borrow();
        state.views.iter().find(|view| view.id == id).cloned()
    }

    /// The chats grouped for the sidebar, as of now.
    pub fn sections(&self) -> Vec<ChatSection> {
        self.imp().store.chat_sections(SystemTime::now())
    }

    pub fn connect_chats_changed<F: Fn(&Self) + 'static>(&self, f: F) -> glib::SignalHandlerId {
        self.connect_closure(
            "chats-changed",
            false,
            closure_local!(move |model: &AppModel| f(model)),
        )
    }

    pub fn connect_views_changed<F: Fn(&Self) + 'static>(&self, f: F) -> glib::SignalHandlerId {
        self.connect_closure(
            "views-changed",
            false,
            closure_local!(move |model: &AppModel| f(model)),
        )
    }

    /// Keeps `state` and signals what changed, after releasing the borrow,
    /// so handlers can read the model.
    fn apply(&self, state: State) {
        let old = self.imp().state.replace(state);
        let (chats_changed, views_changed) = {
            let new = self.imp().state.borrow();
            (old.chats != new.chats, old.views != new.views)
        };
        if chats_changed {
            self.emit_by_name::<()>("chats-changed", &[]);
        }
        if views_changed {
            self.emit_by_name::<()>("views-changed", &[]);
        }
    }
}
