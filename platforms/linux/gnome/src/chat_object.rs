//! GObject wrapper for a core `Chat`, so it can live in a `gio::ListModel`.

use gtk::{gio, glib};

use crate::message_object::MessageObject;

mod imp {
    use std::cell::OnceCell;

    use gtk::prelude::*;
    use gtk::subclass::prelude::*;
    use gtk::{gio, glib};

    #[derive(Default, glib::Properties)]
    #[properties(wrapper_type = super::ChatObject)]
    pub struct ChatObject {
        #[property(get, construct_only)]
        id: OnceCell<u64>,
        #[property(get, construct_only)]
        title: OnceCell<String>,
        /// [`super::MessageObject`]s, oldest first.
        #[property(get, construct_only)]
        messages: OnceCell<gio::ListStore>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for ChatObject {
        const NAME: &'static str = "MiracleChatObject";
        type Type = super::ChatObject;
    }

    #[glib::derived_properties]
    impl ObjectImpl for ChatObject {}
}

glib::wrapper! {
    pub struct ChatObject(ObjectSubclass<imp::ChatObject>);
}

impl ChatObject {
    pub fn new(chat: &miracle_core::Chat) -> Self {
        let messages = gio::ListStore::new::<MessageObject>();
        let objects: Vec<_> = chat.messages.iter().map(MessageObject::new).collect();
        messages.extend_from_slice(&objects);
        glib::Object::builder()
            .property("id", chat.id)
            .property("title", &chat.title)
            .property("messages", messages)
            .build()
    }
}

