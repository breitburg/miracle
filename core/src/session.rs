use std::time::SystemTime;

use crate::Chat;

/// One unit of work, shown in the sidebar by its title.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Session {
    pub id: u64,
    pub title: String,
    /// When the session last changed. Shells group sessions by this date.
    pub updated_at: SystemTime,
    pub chat: Chat,
}
