//! A window with one session and nothing else, opened from the sidebar. It
//! owns its core view and closes it with the window.

use adw::prelude::*;
use gtk::glib;
use miracle_core::Action;

use crate::app_model::AppModel;
use crate::chat_view::ChatView;

pub fn open(app: &adw::Application, model: &AppModel, session_id: u64) {
    let chat_view = ChatView::default();
    chat_view.bind(model, model.open_view(Some(session_id)));

    let title = adw::WindowTitle::new("", "");
    chat_view
        .bind_property("title", &title, "title")
        .sync_create()
        .build();
    let header = adw::HeaderBar::new();
    header.set_title_widget(Some(&title));

    let toolbar_view = adw::ToolbarView::new();
    toolbar_view.add_top_bar(&header);
    toolbar_view.set_content(Some(&chat_view));
    // As in the main window: the flat bar needs the view background.
    toolbar_view.add_css_class("view");

    let window = adw::ApplicationWindow::builder()
        .application(app)
        .default_width(640)
        .default_height(720)
        .width_request(360)
        .height_request(294)
        .content(&toolbar_view)
        .build();
    let view_id = chat_view.view_id();
    let model = model.clone();
    window.connect_close_request(glib::clone!(
        #[weak]
        model,
        #[upgrade_or]
        glib::Propagation::Proceed,
        move |_| {
            model.send(Action::CloseView { view_id });
            glib::Propagation::Proceed
        }
    ));
    window.present();
}
