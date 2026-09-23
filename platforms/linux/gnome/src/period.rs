//! Sidebar section titles for the core's [`Period`]s.

use gtk::glib;
use miracle_core::Period;

/// "Today", "Yesterday", "Previous 7 Days", "Previous 30 Days", a month name
/// or a year.
pub fn title(period: Period) -> String {
    match period {
        Period::Today => "Today".to_owned(),
        Period::Yesterday => "Yesterday".to_owned(),
        Period::PreviousSevenDays => "Previous 7 Days".to_owned(),
        Period::PreviousThirtyDays => "Previous 30 Days".to_owned(),
        Period::Month { month } => glib::DateTime::from_local(2000, month.into(), 1, 0, 0, 0.0)
            .and_then(|date| date.format("%B"))
            .expect("the first of a month is a valid date")
            .into(),
        Period::Year { year } => year.to_string(),
    }
}
