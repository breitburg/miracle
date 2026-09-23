use std::time::SystemTime;

use adw::prelude::*;
use adw::subclass::prelude::*;
use gtk::{gio, glib};
use miracle_core::Action;

use crate::chat_object::ChatObject;
use crate::period;

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
    use std::cell::RefCell;

    use adw::subclass::prelude::*;
    use gtk::prelude::*;
    use gtk::{gdk, glib};

    use crate::app_model::AppModel;
    use crate::chat_object::ChatObject;
    use crate::message_object::{MessageObject, Role};

    #[derive(Default, gtk::CompositeTemplate, glib::Properties)]
    #[template(resource = "/dev/miracle/Miracle/ui/window.ui")]
    #[properties(wrapper_type = super::Window)]
    pub struct Window {
        #[property(get)]
        model: RefCell<AppModel>,
        #[template_child]
        pub(super) toast_overlay: TemplateChild<adw::ToastOverlay>,
        #[template_child]
        pub(super) split_view: TemplateChild<adw::OverlaySplitView>,
        #[template_child]
        pub(super) sidebar: TemplateChild<adw::Sidebar>,
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
    impl ObjectSubclass for Window {
        const NAME: &'static str = "MiracleWindow";
        type Type = super::Window;
        type ParentType = adw::ApplicationWindow;

        fn class_init(klass: &mut Self::Class) {
            // The template refers to these types by name.
            AppModel::ensure_type();
            ChatObject::ensure_type();
            MessageObject::ensure_type();
            klass.bind_template();
            klass.bind_template_callbacks();

            klass.install_action("chat.new", None, |window, _, _| {
                window.model().send(Action::OpenNewChat);
            });
            klass.add_binding_action(gdk::Key::n, gdk::ModifierType::CONTROL_MASK, "chat.new");
            klass.install_action("chat.send", None, |window, _, _| window.send_prompt());
            klass.install_action(
                "message.copy",
                Some(glib::VariantTy::STRING),
                |window, _, text| {
                    if let Some(text) = text.and_then(|text| text.str()) {
                        window.copy_to_clipboard(text);
                    }
                },
            );
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    #[gtk::template_callbacks]
    impl Window {
        #[template_callback(function)]
        fn is_empty(text: &str) -> bool {
            text.is_empty()
        }

        /// The page of the chat pane: the messages, or the hint while the
        /// new chat is open.
        #[template_callback(function)]
        fn chat_page(chat: Option<&ChatObject>) -> &'static str {
            if chat.is_some() {
                "messages"
            } else {
                "new-chat"
            }
        }

        /// The open chat's messages, or none while the new chat is open.
        #[template_callback(function)]
        fn chat_messages(chat: Option<&ChatObject>) -> Option<gtk::gio::ListStore> {
            chat.map(ChatObject::messages)
        }

        /// The header title: the open chat's, or "New Chat" while the new
        /// chat is open.
        #[template_callback(function)]
        fn chat_title(chat: Option<&ChatObject>) -> String {
            chat.map_or_else(|| "New Chat".to_owned(), ChatObject::title)
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

        /// List items cannot reach the window, so the button passes its
        /// `ListItem`, and the row activates `message.copy` with the text;
        /// the action travels up to the window. The handler must be
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
    impl ObjectImpl for Window {
        fn constructed(&self) {
            self.parent_constructed();
            let window = self.obj();
            let model = window.model();

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

            model.chats().connect_items_changed(glib::clone!(
                #[weak]
                window,
                move |_, _, _, _| window.render_sidebar()
            ));
            model.connect_selected_chat_notify(glib::clone!(
                #[weak]
                window,
                move |_| window.scroll_to_last_message()
            ));
            // The core owns the draft: edits go to it, and its changes (such
            // as clearing after a send) come back to the text view.
            self.prompt.buffer().connect_changed(glib::clone!(
                #[weak]
                model,
                move |buffer| {
                    let text = super::buffer_text(buffer);
                    if text != model.draft() {
                        model.send(Action::EditDraft { text });
                    }
                }
            ));
            let prompt = self.prompt.get();
            model.connect_draft_notify(glib::clone!(
                #[weak]
                prompt,
                move |model| {
                    let buffer = prompt.buffer();
                    let draft = model.draft();
                    if super::buffer_text(&buffer) != draft {
                        buffer.set_text(&draft);
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
                window,
                #[upgrade_or]
                glib::Propagation::Proceed,
                move |_, key, _, modifiers| {
                    let enter = matches!(
                        key,
                        gdk::Key::Return | gdk::Key::KP_Enter | gdk::Key::ISO_Enter
                    );
                    if enter && !modifiers.contains(gdk::ModifierType::SHIFT_MASK) {
                        window.send_prompt();
                        glib::Propagation::Stop
                    } else {
                        glib::Propagation::Proceed
                    }
                }
            ));
            self.prompt.add_controller(keys);

            self.sidebar.connect_activated(glib::clone!(
                #[weak]
                window,
                move |_, index| window.activate_sidebar_item(index)
            ));

            window.render_sidebar();
            window.scroll_to_last_message();
        }
    }
    impl WidgetImpl for Window {}
    impl WindowImpl for Window {}
    impl ApplicationWindowImpl for Window {}
    impl AdwApplicationWindowImpl for Window {}
}

glib::wrapper! {
    pub struct Window(ObjectSubclass<imp::Window>)
        @extends adw::ApplicationWindow, gtk::ApplicationWindow, gtk::Window, gtk::Widget,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable,
                    gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

impl Window {
    pub fn new(app: &adw::Application) -> Self {
        glib::Object::builder().property("application", app).build()
    }

    /// Rebuilds the sidebar: one section per period, newest first.
    /// `AdwSidebar` has no model for sections, so we recreate them.
    fn render_sidebar(&self) {
        let sidebar = &self.imp().sidebar;
        sidebar.remove_all();

        // Nothing is selected while the new chat is open.
        let selected = self.model().selected_chat().map(|chat| chat.id());
        sidebar.set_selected(gtk::INVALID_LIST_POSITION);
        let mut index = 0;
        for chat_section in self.model().sections() {
            let section = adw::SidebarSection::new();
            section.set_title(Some(&period::title(chat_section.period)));
            sidebar.append(section.clone());
            for chat in chat_section.chats {
                section.append(adw::SidebarItem::new(&chat.title));
                if selected == Some(chat.id) {
                    sidebar.set_selected(index);
                }
                index += 1;
            }
        }
    }

    /// Sidebar items follow the order of `chats`, so an item's index is the
    /// chat's position.
    fn activate_sidebar_item(&self, index: u32) {
        if let Some(chat) = self.model().chats().item(index).and_downcast::<ChatObject>() {
            self.model().send(Action::SelectChat { id: chat.id() });
        }
        // A collapsed sidebar covers the chat, so close it after a choice.
        let split_view = &self.imp().split_view;
        if split_view.is_collapsed() {
            split_view.set_show_sidebar(false);
        }
    }

    fn copy_to_clipboard(&self, text: &str) {
        self.clipboard().set_text(text);
        self.imp()
            .toast_overlay
            .add_toast(adw::Toast::new("Copied to clipboard"));
    }

    /// Sends the draft; the core clears it, which empties the text view.
    fn send_prompt(&self) {
        self.model().send(Action::SendMessage {
            sent_at: SystemTime::now(),
        });
    }

    /// Waits one main-loop cycle, so that the list has the new messages.
    fn scroll_to_last_message(&self) {
        glib::idle_add_local_once(glib::clone!(
            #[weak(rename_to = window)]
            self,
            move || {
                let list = &window.imp().message_list;
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
