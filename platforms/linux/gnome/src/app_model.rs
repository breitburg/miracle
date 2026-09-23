//! Thin GObject adapter over the Rust `Store`, the GTK counterpart of the
//! Swift `AppModel`.
//!
//! Holds no business rules: it sends actions to the core and republishes the
//! resulting state as GObject properties, so widgets can bind to them.

use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use miracle_core::{Action, State};

use crate::chat_object::ChatObject;

mod imp {
    use std::cell::RefCell;

    use gtk::prelude::*;
    use gtk::subclass::prelude::*;
    use gtk::{gio, glib};

    use crate::chat_object::ChatObject;

    #[derive(glib::Properties)]
    #[properties(wrapper_type = super::AppModel)]
    pub struct AppModel {
        /// [`ChatObject`]s, newest first.
        #[property(get)]
        pub(super) chats: gio::ListStore,
        /// The open chat, one of `chats`.
        #[property(get, nullable)]
        pub(super) selected_chat: RefCell<Option<ChatObject>>,
        /// The core chats that `chats` shows, to skip rebuilds.
        pub(super) rendered_chats: RefCell<Vec<miracle_core::Chat>>,
        pub(super) store: miracle_core::Store,
    }

    impl Default for AppModel {
        fn default() -> Self {
            Self {
                chats: gio::ListStore::new::<ChatObject>(),
                selected_chat: RefCell::default(),
                rendered_chats: RefCell::default(),
                store: miracle_core::Store::new(),
            }
        }
    }

    #[glib::object_subclass]
    impl ObjectSubclass for AppModel {
        const NAME: &'static str = "MiracleAppModel";
        type Type = super::AppModel;
    }

    #[glib::derived_properties]
    impl ObjectImpl for AppModel {
        fn constructed(&self) {
            self.parent_constructed();
            self.obj().render(&self.store.state());
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
        self.render(&state);
    }

    fn render(&self, state: &State) {
        let imp = self.imp();
        let chats_changed = *imp.rendered_chats.borrow() != state.chats;
        let objects: Vec<ChatObject> = if chats_changed {
            state.chats.iter().map(ChatObject::new).collect()
        } else {
            imp.chats
                .iter()
                .map(|chat| chat.expect("chats do not change"))
                .collect()
        };

        // Select first, so that listeners of `chats` see the new selection.
        let selected = objects
            .iter()
            .find(|chat| Some(chat.id()) == state.selected_chat_id)
            .cloned();
        if *imp.selected_chat.borrow() != selected {
            imp.selected_chat.replace(selected);
            self.notify_selected_chat();
        }

        if chats_changed {
            imp.rendered_chats.replace(state.chats.clone());
            imp.chats.splice(0, imp.chats.n_items(), &objects);
        }
    }
}
