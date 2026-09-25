//! Thin GObject adapter over the Rust `Store`, the GTK counterpart of the
//! Swift `AppModel`. One model serves every window.
//!
//! Holds no business rules: it sends actions to the core, keeps the
//! resulting state and tells widgets what changed, through the
//! `sessions-changed` and `views-changed` signals.

use std::time::SystemTime;

use gtk::glib;
use gtk::glib::closure_local;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use miracle_core::{Action, Model, Session, SessionSection, SessionViewState, State};

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
                    // The sessions or their order changed.
                    Signal::builder("sessions-changed").build(),
                    // A view opened, closed, or changed its session or draft.
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

    /// Opens a view on the session (a new session for `None`) and returns
    /// its id, for the window that shows it.
    pub fn open_view(&self, session_id: Option<u64>) -> u64 {
        let id = self.imp().store.open_view(session_id);
        self.apply(self.imp().store.state());
        id
    }

    /// Newest first.
    pub fn sessions(&self) -> Vec<Session> {
        self.imp().state.borrow().sessions.clone()
    }

    pub fn session(&self, id: u64) -> Option<Session> {
        self.imp().state.borrow().session(id).cloned()
    }

    pub fn view(&self, id: u64) -> Option<SessionViewState> {
        self.imp().state.borrow().view(id).cloned()
    }

    /// The session the view shows; `None` for a new session. Clones
    /// nothing, so it is cheap on every keystroke.
    pub fn shown_session_id(&self, view_id: u64) -> Option<u64> {
        self.imp().state.borrow().view(view_id)?.session_id
    }

    /// The session's model, without a clone of its chat.
    pub fn session_model(&self, id: u64) -> Option<Model> {
        Some(self.imp().state.borrow().session(id)?.model)
    }

    /// The sessions grouped for the sidebar, as of now.
    pub fn sections(&self) -> Vec<SessionSection> {
        self.imp().store.session_sections(SystemTime::now())
    }

    pub fn connect_sessions_changed<F: Fn(&Self) + 'static>(&self, f: F) -> glib::SignalHandlerId {
        self.connect_closure(
            "sessions-changed",
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
        let (sessions_changed, views_changed) = {
            let new = self.imp().state.borrow();
            (old.sessions != new.sessions, old.views != new.views)
        };
        if sessions_changed {
            self.emit_by_name::<()>("sessions-changed", &[]);
        }
        if views_changed {
            self.emit_by_name::<()>("views-changed", &[]);
        }
    }
}
