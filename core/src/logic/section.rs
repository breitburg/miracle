use std::time::SystemTime;

use jiff::Timestamp;
use jiff::civil::Date;
use jiff::tz::TimeZone;

use crate::Session;

/// When the sessions of a sidebar section were last updated. Shells turn it
/// into a localized title.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Period {
    Today,
    Yesterday,
    /// 2 to 7 days ago.
    PreviousSevenDays,
    /// 8 to 30 days ago.
    PreviousThirtyDays,
    /// Earlier this year. `month` is 1 (January) to 12.
    Month {
        month: u8,
    },
    /// Before this year.
    Year {
        year: i16,
    },
}

/// Consecutive sessions updated in the same [`Period`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SessionSection {
    pub period: Period,
    /// Newest first.
    pub sessions: Vec<SessionSummary>,
}

/// What the sidebar shows of a session.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SessionSummary {
    pub id: u64,
    pub title: String,
}

impl From<&Session> for SessionSummary {
    fn from(session: &Session) -> Self {
        Self {
            id: session.id,
            title: session.title.clone(),
        }
    }
}

/// Groups `sessions` (newest first) by the local calendar day they were
/// last updated, as seen at `now`.
pub(crate) fn session_sections(sessions: &[Session], now: SystemTime) -> Vec<SessionSection> {
    sections_in(sessions, now, &TimeZone::system())
}

fn sections_in(sessions: &[Session], now: SystemTime, time_zone: &TimeZone) -> Vec<SessionSection> {
    let today = local_date(now, time_zone);
    let mut sections: Vec<SessionSection> = Vec::new();
    for session in sessions {
        let period = period(local_date(session.updated_at, time_zone), today);
        match sections.last_mut() {
            Some(section) if section.period == period => section.sessions.push(session.into()),
            _ => sections.push(SessionSection {
                period,
                sessions: vec![session.into()],
            }),
        }
    }
    sections
}

/// Calendar days, not 24-hour periods, so a day with a DST change still
/// counts as one.
fn period(date: Date, today: Date) -> Period {
    let days = (today - date).get_days();
    match days {
        ..=0 => Period::Today,
        1 => Period::Yesterday,
        ..=7 => Period::PreviousSevenDays,
        ..=30 => Period::PreviousThirtyDays,
        _ if date.year() == today.year() => Period::Month {
            month: date.month().unsigned_abs(),
        },
        _ => Period::Year { year: date.year() },
    }
}

fn local_date(time: SystemTime, time_zone: &TimeZone) -> Date {
    Timestamp::try_from(time)
        .unwrap_or(Timestamp::UNIX_EPOCH)
        .to_zoned(time_zone.clone())
        .date()
}

#[cfg(test)]
mod tests {
    use jiff::civil::date;

    use super::*;
    use crate::{Chat, Model};

    fn at(date: Date, hour: i8, time_zone: &TimeZone) -> SystemTime {
        date.at(hour, 0, 0, 0)
            .to_zoned(time_zone.clone())
            .expect("the time exists")
            .timestamp()
            .into()
    }

    fn session(id: u64, updated_at: SystemTime) -> Session {
        Session {
            id,
            title: format!("Session {id}"),
            model: Model::default(),
            updated_at,
            chat: Chat::default(),
        }
    }

    #[test]
    fn periods_follow_calendar_days() {
        let today = date(2026, 9, 23);
        assert_eq!(period(date(2026, 9, 24), today), Period::Today);
        assert_eq!(period(today, today), Period::Today);
        assert_eq!(period(date(2026, 9, 22), today), Period::Yesterday);
        assert_eq!(period(date(2026, 9, 16), today), Period::PreviousSevenDays);
        assert_eq!(period(date(2026, 9, 15), today), Period::PreviousThirtyDays);
        assert_eq!(period(date(2026, 8, 24), today), Period::PreviousThirtyDays);
        assert_eq!(period(date(2026, 8, 23), today), Period::Month { month: 8 });
        assert_eq!(
            period(date(2025, 12, 31), today),
            Period::Year { year: 2025 }
        );
        assert_eq!(
            period(date(2026, 12, 31), date(2027, 1, 2)),
            Period::PreviousSevenDays
        );
    }

    #[test]
    fn sections_group_consecutive_sessions_in_local_time() {
        let time_zone = TimeZone::get("Europe/Amsterdam").expect("the zone exists");
        // Clocks go back on 25 October 2026: a 25-hour day.
        let now = at(date(2026, 10, 26), 0, &time_zone);
        let sessions = [
            session(1, at(date(2026, 10, 26), 0, &time_zone)),
            session(2, at(date(2026, 10, 25), 0, &time_zone)),
            session(3, at(date(2026, 10, 24), 23, &time_zone)),
        ];
        let sections = sections_in(&sessions, now, &time_zone);
        let periods: Vec<_> = sections
            .iter()
            .map(|section| (section.period, section.sessions.len()))
            .collect();
        assert_eq!(
            periods,
            [
                (Period::Today, 1),
                (Period::Yesterday, 1),
                (Period::PreviousSevenDays, 1)
            ]
        );
    }
}
