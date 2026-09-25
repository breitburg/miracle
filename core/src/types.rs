//! Plain data: the state a shell renders and the actions it sends.

mod action;
mod chat;
mod message;
mod model;
mod session;
mod state;
mod view;

pub use action::Action;
pub use chat::Chat;
pub use message::{Message, Role};
pub use model::Model;
pub use session::Session;
pub use state::State;
pub use view::SessionViewState;
