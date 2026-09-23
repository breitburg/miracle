//! One core view: the shown chat's messages, or a hint for a new chat,
//! above the composer. Both the main window and chat windows show one.

use std::time::SystemTime;

use adw::prelude::*;
use adw::subclass::prelude::*;
use gtk::glib;
use miracle_core::Action;

use crate::app_model::AppModel;
use crate::message_object::MessageObject;

/// Maximum width of the messages and of the composer row, in pixels.
const CONTENT_MAX_WIDTH: i32 = 640;

/// Horizontal padding of the messages and of the composer, in pixels.
/// style.css reads it as `--content-padding`, see [`provide_css_variables`].
const CONTENT_PADDING: i32 = 16;

/// Vertical padding of the messages and of the composer, in pixels.
/// style.css reads it as `--content-vertical-padding`.
const CONTENT_VERTICAL_PADDING: i32 = 12;

/// Padding inside a user message's bubble, in pixels. style.css reads them
/// as `--bubble-padding` and `--bubble-vertical-padding`.
const BUBBLE_PADDING: i32 = 16;
const BUBBLE_VERTICAL_PADDING: i32 = 12;

/// Gives Rust layout constants to style.css as CSS variables, so both
/// share one value.
pub fn provide_css_variables(display: &gtk::gdk::Display) {
    let provider = gtk::CssProvider::new();
    provider.load_from_string(&format!(
        ":root {{ --content-padding: {CONTENT_PADDING}px; \
         --content-vertical-padding: {CONTENT_VERTICAL_PADDING}px; \
         --bubble-padding: {BUBBLE_PADDING}px; \
         --bubble-vertical-padding: {BUBBLE_VERTICAL_PADDING}px; }}"
    ));
    gtk::style_context_add_provider_for_display(
        display,
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}

mod imp {
    use std::cell::{Cell, OnceCell, RefCell};

    use adw::subclass::prelude::*;
    use gtk::prelude::*;
    use gtk::{gdk, gio, glib};

    use crate::app_model::AppModel;
    use crate::message_object::{MessageObject, Role};

    #[derive(Default, gtk::CompositeTemplate, glib::Properties)]
    #[template(resource = "/dev/miracle/Miracle/ui/chat_view.ui")]
    #[properties(wrapper_type = super::ChatView)]
    pub struct ChatView {
        /// The shown chat's [`MessageObject`]s, oldest first.
        #[property(get)]
        pub(super) messages: RefCell<Option<gio::ListStore>>,
        /// For the window's header: the chat's title, or "New Chat".
        #[property(get)]
        pub(super) title: RefCell<String>,
        pub(super) model: OnceCell<AppModel>,
        pub(super) view_id: Cell<u64>,
        /// The chat that `messages` holds, to tell a switch from new messages.
        pub(super) shown_chat_id: Cell<Option<u64>>,
        pub(super) handlers: RefCell<Vec<glib::SignalHandlerId>>,
        #[template_child]
        pub(super) toast_overlay: TemplateChild<adw::ToastOverlay>,
        #[template_child]
        pub(super) stack: TemplateChild<gtk::Stack>,
        #[template_child]
        pub(super) message_clamp: TemplateChild<adw::ClampScrollable>,
        #[template_child]
        pub(super) message_list: TemplateChild<gtk::ListView>,
        #[template_child]
        pub(super) composer_clamp: TemplateChild<adw::Clamp>,
        #[template_child]
        pub(super) prompt: TemplateChild<gtk::TextView>,
        #[template_child]
        pub(super) send_button: TemplateChild<gtk::Button>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for ChatView {
        const NAME: &'static str = "MiracleChatView";
        type Type = super::ChatView;
        type ParentType = adw::Bin;

        fn class_init(klass: &mut Self::Class) {
            // The template refers to these types by name.
            MessageObject::ensure_type();
            klass.bind_template();
            klass.bind_template_callbacks();

            klass.install_action("chat.send", None, |view, _, _| view.send());
            klass.install_action(
                "message.copy",
                Some(glib::VariantTy::STRING),
                |view, _, text| {
                    if let Some(text) = text.and_then(|text| text.str()) {
                        view.copy_to_clipboard(text);
                    }
                },
            );
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    #[gtk::template_callbacks]
    impl ChatView {
        #[template_callback(function)]
        fn is_empty(text: &str) -> bool {
            text.is_empty()
        }

        /// The stack page that shows a message of this role.
        #[template_callback(function)]
        fn role_page(role: Role) -> &'static str {
            match role {
                Role::User => "user",
                Role::Assistant => "assistant",
            }
        }

        /// User actions sit under the bubble at the end; assistant actions
        /// at the start.
        #[template_callback(function)]
        fn actions_halign(role: Role) -> gtk::Align {
            match role {
                Role::User => gtk::Align::End,
                Role::Assistant => gtk::Align::Start,
            }
        }

        /// List items cannot reach the view, so the button passes its
        /// `ListItem`, and the row activates `message.copy` with the text;
        /// the action travels up to the view. The handler must be
        /// `not-swapped`: function callbacks skip the first value, which is
        /// then the button.
        #[template_callback(function)]
        fn copy_message(item: &gtk::ListItem) {
            let (Some(message), Some(child)) =
                (item.item().and_downcast::<MessageObject>(), item.child())
            else {
                return;
            };
            let _ = child.activate_action("message.copy", Some(&message.content().to_variant()));
        }
    }

    #[glib::derived_properties]
    impl ObjectImpl for ChatView {
        fn constructed(&self) {
            self.parent_constructed();
            let view = self.obj();
            self.messages
                .replace(Some(gio::ListStore::new::<MessageObject>()));

            // A threshold equal to the maximum turns off the clamps' gradual
            // tightening: the content takes all the width up to the maximum.
            self.message_clamp
                .set_maximum_size(super::CONTENT_MAX_WIDTH);
            self.message_clamp
                .set_tightening_threshold(super::CONTENT_MAX_WIDTH);
            self.composer_clamp
                .set_maximum_size(super::CONTENT_MAX_WIDTH);
            self.composer_clamp
                .set_tightening_threshold(super::CONTENT_MAX_WIDTH);
            self.prompt.set_left_margin(super::CONTENT_PADDING);
            self.prompt.set_right_margin(super::CONTENT_PADDING);
            self.prompt.set_top_margin(super::CONTENT_VERTICAL_PADDING);
            self.prompt
                .set_bottom_margin(super::CONTENT_VERTICAL_PADDING);
            self.send_button.set_margin_end(super::CONTENT_PADDING);
            self.send_button
                .set_margin_top(super::CONTENT_VERTICAL_PADDING);
            self.send_button
                .set_margin_bottom(super::CONTENT_VERTICAL_PADDING);

            // Edits go to the view's draft in the core; its changes (such as
            // clearing after a send) come back in `render`.
            self.prompt.buffer().connect_changed(glib::clone!(
                #[weak]
                view,
                move |buffer| {
                    let (Some(model), Some(state)) = (view.imp().model.get(), view.state()) else {
                        return;
                    };
                    let text = super::buffer_text(buffer);
                    if text != state.draft {
                        model.send(miracle_core::Action::EditDraft {
                            view_id: state.id,
                            text,
                        });
                    }
                }
            ));

            // Enter sends and Shift+Enter makes a new line, the same as on
            // macOS. The capture phase sees the key before the text view,
            // which would otherwise insert a new line for both.
            let keys = gtk::EventControllerKey::new();
            keys.set_propagation_phase(gtk::PropagationPhase::Capture);
            keys.connect_key_pressed(glib::clone!(
                #[weak]
                view,
                #[upgrade_or]
                glib::Propagation::Proceed,
                move |_, key, _, modifiers| {
                    let enter = matches!(
                        key,
                        gdk::Key::Return | gdk::Key::KP_Enter | gdk::Key::ISO_Enter
                    );
                    if enter && !modifiers.contains(gdk::ModifierType::SHIFT_MASK) {
                        view.send();
                        glib::Propagation::Stop
                    } else {
                        glib::Propagation::Proceed
                    }
                }
            ));
            self.prompt.add_controller(keys);
        }

        fn dispose(&self) {
            if let Some(model) = self.model.get() {
                for handler in self.handlers.take() {
                    model.disconnect(handler);
                }
            }
        }
    }
    impl WidgetImpl for ChatView {}
    impl BinImpl for ChatView {}
}

glib::wrapper! {
    pub struct ChatView(ObjectSubclass<imp::ChatView>)
        @extends adw::Bin, gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

impl Default for ChatView {
    fn default() -> Self {
        glib::Object::new()
    }
}

impl ChatView {
    /// Shows the core view `view_id` from `model` and follows its changes.
    /// Call once.
    pub fn bind(&self, model: &AppModel, view_id: u64) {
        let imp = self.imp();
        imp.model
            .set(model.clone())
            .expect("a chat view is bound once");
        imp.view_id.set(view_id);
        let render = glib::clone!(
            #[weak(rename_to = view)]
            self,
            move |_: &AppModel| view.render()
        );
        imp.handlers.replace(vec![
            model.connect_chats_changed(render.clone()),
            model.connect_views_changed(render),
        ]);
        self.render();
    }

    pub fn view_id(&self) -> u64 {
        self.imp().view_id.get()
    }

    fn state(&self) -> Option<miracle_core::ChatViewState> {
        self.imp().model.get()?.view(self.view_id())
    }

    /// Brings the widgets in line with the core: messages, page, title and
    /// draft.
    fn render(&self) {
        let imp = self.imp();
        let (Some(model), Some(state)) = (imp.model.get(), self.state()) else {
            return;
        };
        let chat = state.chat_id.and_then(|id| model.chat(id));
        let messages = self.messages().expect("created in constructed");

        let switched = imp.shown_chat_id.get() != state.chat_id;
        let chat_messages = chat.as_ref().map_or(&[][..], |chat| &chat.messages[..]);
        let shown = messages.n_items() as usize;
        if switched || chat_messages.len() < shown {
            let objects: Vec<_> = chat_messages.iter().map(MessageObject::new).collect();
            messages.splice(0, messages.n_items(), &objects);
        } else {
            // Messages are only ever appended.
            let objects: Vec<_> = chat_messages[shown..]
                .iter()
                .map(MessageObject::new)
                .collect();
            messages.extend_from_slice(&objects);
        }
        if switched || chat_messages.len() != shown {
            imp.shown_chat_id.set(state.chat_id);
            self.scroll_to_last_message();
        }

        imp.stack.set_visible_child_name(if chat.is_some() {
            "messages"
        } else {
            "new-chat"
        });

        let title = chat.map_or_else(|| "New Chat".to_owned(), |chat| chat.title);
        if *imp.title.borrow() != title {
            imp.title.replace(title);
            self.notify_title();
        }

        let buffer = imp.prompt.buffer();
        if buffer_text(&buffer) != state.draft {
            buffer.set_text(&state.draft);
        }
    }

    /// Sends the draft; the core clears it, which empties the text view.
    fn send(&self) {
        if let Some(model) = self.imp().model.get() {
            model.send(Action::SendMessage {
                view_id: self.view_id(),
                sent_at: SystemTime::now(),
            });
        }
    }

    fn copy_to_clipboard(&self, text: &str) {
        self.clipboard().set_text(text);
        self.imp()
            .toast_overlay
            .add_toast(adw::Toast::new("Copied to clipboard"));
    }

    /// Waits one main-loop cycle, so that the list has the new messages.
    fn scroll_to_last_message(&self) {
        glib::idle_add_local_once(glib::clone!(
            #[weak(rename_to = view)]
            self,
            move || {
                let list = &view.imp().message_list;
                let count = list.model().map_or(0, |model| model.n_items());
                if count > 0 {
                    list.scroll_to(count - 1, gtk::ListScrollFlags::NONE, None);
                }
            }
        ));
    }
}

/// All of `buffer`'s text.
fn buffer_text(buffer: &gtk::TextBuffer) -> String {
    let (start, end) = buffer.bounds();
    buffer.text(&start, &end, false).into()
}
