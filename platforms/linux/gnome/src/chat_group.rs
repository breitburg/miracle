//! View-side grouping of chats by date, like the Swift `ChatGroup`.

use gtk::glib;

/// The sidebar section for a chat updated at `date`: "Today", "Yesterday",
/// "Previous 7 Days", "Previous 30 Days", then the month for this year and
/// the year before that.
pub fn title(date: &glib::DateTime, now: &glib::DateTime) -> String {
    let days = days_between(date, now);
    match days {
        ..=0 => "Today".to_owned(),
        1 => "Yesterday".to_owned(),
        ..=7 => "Previous 7 Days".to_owned(),
        ..=30 => "Previous 30 Days".to_owned(),
        _ if date.year() == now.year() => {
            date.format("%B").expect("month name is valid UTF-8").into()
        }
        _ => date.year().to_string(),
    }
}

/// Calendar days from `from` to `to`, in the local time zone.
fn days_between(from: &glib::DateTime, to: &glib::DateTime) -> i64 {
    let hours = start_of_day(to).difference(&start_of_day(from)).as_hours();
    // Round, because a day with a DST change has 23 or 25 hours.
    (hours + 12).div_euclid(24)
}

fn start_of_day(date: &glib::DateTime) -> glib::DateTime {
    glib::DateTime::from_local(date.year(), date.month(), date.day_of_month(), 0, 0, 0.0)
        .expect("midnight of an existing date is valid")
}
