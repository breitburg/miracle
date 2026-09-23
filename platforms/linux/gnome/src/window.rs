//! The main window: sessions in the sidebar, next to its own core view.

use adw::prelude::*;
use adw::subclass::prelude::*;
use gtk::{gio, glib};
use miracle_core::Action;

use crate::app_model::AppModel;
use crate::period;
use crate::session_window;

mod imp {
    use std::cell::{Cell, OnceCell, RefCell};

    use adw::subclass::prelude::*;
    use gtk::prelude::*;
    use gtk::{gdk, gio, glib};

    use crate::app_model::AppModel;
    use crate::chat_view::ChatView;

    #[derive(Default, gtk::CompositeTemplate, glib::Properties)]
    #[template(resource = "/dev/miracle/Miracle/ui/window.ui")]
    #[properties(wrapper_type = super::Window)]
    pub struct Window {
        /// The model every window shares.
        #[property(get, construct_only)]
        pub(super) model: OnceCell<AppModel>,
        /// The session of each sidebar item, in sidebar order.
        pub(super) items: RefCell<Vec<(adw::SidebarItem, u64)>>,
        /// The session whose context menu is open.
        pub(super) menu_session_id: Cell<Option<u64>>,
        #[template_child]
        pub(super) split_view: TemplateChild<adw::OverlaySplitView>,
        #[template_child]
        pub(super) sidebar: TemplateChild<adw::Sidebar>,
        #[template_child]
        pub(super) chat_view: TemplateChild<ChatView>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for Window {
        const NAME: &'static str = "MiracleWindow";
        type Type = super::Window;
        type ParentType = adw::ApplicationWindow;

        fn class_init(klass: &mut Self::Class) {
            // The template refers to this type by name.
            ChatView::ensure_type();
            klass.bind_template();

            klass.install_action("session.new", None, |window, _, _| {
                window.model().send(super::Action::ShowSession {
                    view_id: window.imp().chat_view.view_id(),
                    session_id: None,
                });
            });
            klass.add_binding_action(gdk::Key::n, gdk::ModifierType::CONTROL_MASK, "session.new");
            klass.install_action("session.open-in-new-window", None, |window, _, _| {
                if let Some(session_id) = window.imp().menu_session_id.get() {
                    window.open_in_new_window(session_id);
                }
            });
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    #[glib::derived_properties]
    impl ObjectImpl for Window {
        fn constructed(&self) {
            self.parent_constructed();
            let window = self.obj();
            let model = window.model();

            // The main window's view lives as long as the app, on the
            // newest session.
            let view_id = model.open_view(model.sessions().first().map(|session| session.id));
            self.chat_view.bind(&model, view_id);
            window.connect_close_request(glib::clone!(
                #[weak]
                model,
                #[upgrade_or]
                glib::Propagation::Proceed,
                move |_| {
                    model.send(super::Action::CloseView { view_id });
                    glib::Propagation::Proceed
                }
            ));

            model.connect_sessions_changed(glib::clone!(
                #[weak]
                window,
                move |_| window.render_sidebar()
            ));
            model.connect_views_changed(glib::clone!(
                #[weak]
                window,
                move |_| window.select_shown_session()
            ));

            self.sidebar.connect_activated(glib::clone!(
                #[weak]
                window,
                move |_, index| window.activate_sidebar_item(index)
            ));
            // Right-click on a session: "Open in New Window". The menu is shared
            // by all items; `setup-menu` says which item it is for.
            let menu = gio::Menu::new();
            menu.append(
                Some("Open in New Window"),
                Some("session.open-in-new-window"),
            );
            self.sidebar.set_property("menu-model", &menu);
            self.sidebar.connect_closure(
                "setup-menu",
                false,
                glib::closure_local!(
                    #[weak]
                    window,
                    move |_: &adw::Sidebar, item: Option<&adw::SidebarItem>| {
                        let session_id = item.and_then(|item| window.session_id_of(item));
                        window.imp().menu_session_id.set(session_id);
                    }
                ),
            );

            window.render_sidebar();
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
    pub fn new(app: &adw::Application, model: &AppModel) -> Self {
        glib::Object::builder()
            .property("application", app)
            .property("model", model)
            .build()
    }

    /// Rebuilds the sidebar: one section per period, newest first.
    /// `AdwSidebar` has no model for sections, so we recreate them.
    fn render_sidebar(&self) {
        let imp = self.imp();
        imp.sidebar.remove_all();
        let mut items = Vec::new();
        for session_section in self.model().sections() {
            let section = adw::SidebarSection::new();
            section.set_title(Some(&period::title(session_section.period)));
            imp.sidebar.append(section.clone());
            for session in session_section.sessions {
                let item = adw::SidebarItem::new(&session.title);
                section.append(item.clone());
                items.push((item, session.id));
            }
        }
        imp.items.replace(items);
        self.select_shown_session();
    }

    /// Selects the session the main view shows; nothing for a new session.
    fn select_shown_session(&self) {
        let imp = self.imp();
        let shown = self
            .model()
            .view(imp.chat_view.view_id())
            .and_then(|view| view.session_id);
        let position = imp
            .items
            .borrow()
            .iter()
            .position(|(_, session_id)| Some(*session_id) == shown);
        imp.sidebar
            .set_selected(position.map_or(gtk::INVALID_LIST_POSITION, |position| position as u32));
    }

    fn session_id_of(&self, item: &adw::SidebarItem) -> Option<u64> {
        self.imp()
            .items
            .borrow()
            .iter()
            .find(|(candidate, _)| candidate == item)
            .map(|(_, session_id)| *session_id)
    }

    fn activate_sidebar_item(&self, index: u32) {
        let imp = self.imp();
        let session_id = imp.items.borrow().get(index as usize).map(|(_, id)| *id);
        if let Some(session_id) = session_id {
            self.model().send(Action::ShowSession {
                view_id: imp.chat_view.view_id(),
                session_id: Some(session_id),
            });
        }
        // A collapsed sidebar covers the session, so close it after a choice.
        if imp.split_view.is_collapsed() {
            imp.split_view.set_show_sidebar(false);
        }
    }

    fn open_in_new_window(&self, session_id: u64) {
        let app = self
            .application()
            .and_downcast::<adw::Application>()
            .expect("the window belongs to the app");
        session_window::open(&app, &self.model(), session_id);
    }
}
