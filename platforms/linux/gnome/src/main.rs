//! GNOME shell for Miracle: GTK 4 + libadwaita over `miracle_core`.
//!
//! Rust shells link the core directly, with no FFI layer in between.

mod app_model;
mod chat_view;
mod message_object;
mod period;
mod session_window;
mod window;

use adw::prelude::*;
use gtk::{gio, glib};

use crate::app_model::AppModel;
use crate::window::Window;

const APP_ID: &str = env!("APP_ID");

fn main() -> glib::ExitCode {
    let resources = gio::Resource::from_data(&glib::Bytes::from_static(include_bytes!(env!(
        "MIRACLE_GRESOURCE"
    ))))
    .expect("bundled GResource is valid");
    gio::resources_register(&resources);

    let app = adw::Application::builder()
        .application_id(APP_ID)
        // libadwaita loads style.css from here automatically.
        .resource_base_path("/dev/miracle/Miracle")
        .build();

    app.connect_startup(|_| {
        chat_view::provide_css_variables(&gtk::gdk::Display::default().expect("a display exists"));
    });
    // One model for every window.
    let model = AppModel::default();
    app.connect_activate(move |app| {
        let window = app
            .active_window()
            .unwrap_or_else(|| Window::new(app, &model).upcast());
        window.present();
    });
    app.set_accels_for_action("window.close", &["<Control>w"]);

    app.run()
}
