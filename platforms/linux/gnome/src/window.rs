use std::time::SystemTime;

use adw::prelude::*;
use adw::subclass::prelude::*;
use gtk::{gio, glib};
use miracle_core::Action;

use crate::chat_group;
use crate::chat_object::ChatObject;

/// Sidebar index of the first chat: "New Chat" is at index 0.
const FIRST_CHAT_INDEX: u32 = 1;

/// Maximum width of the messages and of the composer row, in pixels.
const CONTENT_MAX_WIDTH: i32 = 640;

/// Horizontal padding of the messages and of the composer, in pixels.
/// style.css reads it as `--content-padding`, see [`provide_css_variables`].
const CONTENT_PADDING: i32 = 16;

/// Vertical padding of the messages and of the composer, in pixels.
/// style.css reads it as `--content-vertical-padding`.
const CONTENT_VERTICAL_PADDING: i32 = 12;

/// Gives Rust layout constants to style.css as CSS variables, so both
/// share one value.
pub fn provide_css_variables(display: &gtk::gdk::Display) {
    let provider = gtk::CssProvider::new();
    provider.load_from_string(&format!(
        ":root {{ --content-padding: {CONTENT_PADDING}px; \
         --content-vertical-padding: {CONTENT_VERTICAL_PADDING}px; }}"
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

    /// Rebuilds the sidebar: one section per date group, newest first.
    /// `AdwSidebar` has no model for sections, so we recreate them.
    fn render_sidebar(&self) {
        let sidebar = &self.imp().sidebar;
        sidebar.remove_all();

        // "New Chat" comes first, in a section without a title.
        let actions = adw::SidebarSection::new();
        let new_chat = adw::SidebarItem::new("New Chat");
        new_chat.set_icon_name(Some("chat-message-new-symbolic"));
        actions.append(new_chat);
        sidebar.append(actions);

        let now = glib::DateTime::now_local().expect("local time is available");
        let selected = self.model().selected_chat();
        // While the new chat is open, "New Chat" is the selected item.
        if selected.is_none() {
            sidebar.set_selected(0);
        }
        let mut current: Option<(String, adw::SidebarSection)> = None;
        for (index, chat) in self.model().chats().iter::<ChatObject>().enumerate() {
            let chat = chat.expect("chats do not change while we read them");
            let title = chat_group::title(&chat.updated_at(), &now);
            if current
                .as_ref()
                .is_none_or(|(current_title, _)| *current_title != title)
            {
                let section = adw::SidebarSection::new();
                section.set_title(Some(&title));
                sidebar.append(section.clone());
                current = Some((title, section));
            }
            let (_, section) = current.as_ref().expect("a section exists");
            section.append(adw::SidebarItem::new(&chat.title()));
            if selected.as_ref() == Some(&chat) {
                sidebar.set_selected(index as u32 + FIRST_CHAT_INDEX);
            }
        }
    }

    /// Chats follow "New Chat" in the same order as `chats`, so a chat's
    /// sidebar index is its position in `chats` plus [`FIRST_CHAT_INDEX`].
    fn activate_sidebar_item(&self, index: u32) {
        match index.checked_sub(FIRST_CHAT_INDEX) {
            None => self.model().send(Action::OpenNewChat),
            Some(position) => {
                if let Some(chat) = self
                    .model()
                    .chats()
                    .item(position)
                    .and_downcast::<ChatObject>()
                {
                    self.model().send(Action::SelectChat { id: chat.id() });
                }
            }
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

    fn send_prompt(&self) {
        let buffer = self.imp().prompt.buffer();
        let (start, end) = buffer.bounds();
        self.model().send(Action::SendMessage {
            content: buffer.text(&start, &end, false).into(),
            sent_at: SystemTime::now(),
        });
        buffer.set_text("");
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
