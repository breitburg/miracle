//! GObject wrapper for a core `Message`, so it can live in a `gio::ListModel`.

use gtk::glib;
use miracle_core::Message;

/// GObject mirror of [`miracle_core::Role`], so properties and Blueprint
/// bindings can use it. The core stays free of GLib.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, glib::Enum)]
#[enum_type(name = "MiracleRole")]
pub enum Role {
    #[default]
    User,
    Assistant,
}

impl From<miracle_core::Role> for Role {
    fn from(role: miracle_core::Role) -> Self {
        match role {
            miracle_core::Role::User => Self::User,
            miracle_core::Role::Assistant => Self::Assistant,
        }
    }
}

mod imp {
    use std::cell::{Cell, OnceCell};

    use gtk::glib;
    use gtk::prelude::*;
    use gtk::subclass::prelude::*;

    use super::Role;

    #[derive(Default, glib::Properties)]
    #[properties(wrapper_type = super::MessageObject)]
    pub struct MessageObject {
        #[property(get, construct_only, builder(Role::default()))]
        role: Cell<Role>,
        #[property(get, construct_only)]
        content: OnceCell<String>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for MessageObject {
        const NAME: &'static str = "MiracleMessageObject";
        type Type = super::MessageObject;
    }

    #[glib::derived_properties]
    impl ObjectImpl for MessageObject {}
}

glib::wrapper! {
    pub struct MessageObject(ObjectSubclass<imp::MessageObject>);
}

impl MessageObject {
    pub fn new(message: &Message) -> Self {
        glib::Object::builder()
            .property("role", Role::from(message.role))
            .property("content", &message.content)
            .build()
    }
}
