//! Business logic for Miracle, shared by every platform shell.
//!
//! The core follows a unidirectional data flow:
//!
//! ```text
//! shell ──Action──▶ Store::dispatch ──▶ reduce(State, Action) ──▶ State ──▶ shell renders
//! ```
//!
//! This crate is plain Rust: it knows nothing about FFI or any UI toolkit.
//! Rust shells (GNOME) depend on it directly; other languages reach it through
//! the `miracle_ffi` facade.
//!
//! # Extension points
//!
//! - **Ports.** When the core needs a platform capability (storage, network,
//!   clock), declare a trait in a `ports` module and take it in `Store::new`.
//!   Rust shells implement it directly; FFI shells implement it through a
//!   `#[uniffi::export(with_foreign)]` trait in `miracle_ffi`.
//! - **Observation.** When the core gains asynchronous work, add
//!   `Store::subscribe(Arc<dyn StateObserver>)`. Shells hop the callback onto
//!   their main thread (`@MainActor` in Swift, `glib::MainContext` in GTK).

mod logic;
mod store;
mod types;

pub use logic::{Period, SessionSection, SessionSummary, reduce};
pub use store::Store;
pub use types::{Action, Chat, Message, Model, Role, Session, SessionViewState, State};
