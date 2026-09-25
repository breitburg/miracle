//! Pure functions over the types: the reducer and the sidebar sections.

mod reducer;
mod section;

pub use reducer::reduce;
pub(crate) use section::session_sections;
pub use section::{Period, SessionSection, SessionSummary};
