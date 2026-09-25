use std::mem;
use std::sync::{Mutex, PoisonError};
use std::time::{Duration, SystemTime};

use crate::logic::session_sections;
use crate::{Action, Chat, Message, Model, Role, Session, SessionSection, State, reduce};

/// Owns the current [`State`] and applies [`Action`]s to it.
///
/// Thread-safe (`Send + Sync`), so any shell or FFI layer can share it.
#[derive(Debug)]
pub struct Store {
    state: Mutex<State>,
}

impl Store {
    /// A store with sample sessions, until the core can create them.
    pub fn new() -> Self {
        const HOUR: u64 = 60 * 60;
        const DAY: u64 = 24 * HOUR;
        let now = SystemTime::now();
        // Each chat alternates user and assistant messages, user first.
        let samples: [(&str, u64, &[&str]); 7] = [
            (
                "Welcome",
                0,
                &[
                    "Hi! What can you do?",
                    "I can help you write, plan, and learn. Ask me to draft an email, \
                     explain a concept, compare options, or break a big task into steps.",
                    "Can you remember things between chats?",
                    "Each chat keeps its own history, so I can refer back to anything \
                     earlier in this conversation. A new chat starts fresh.",
                    "Okay. How do I get the best answers?",
                    "Be specific about what you want and why. Tell me the audience, the \
                     length, and any constraints. If my first answer misses, say what \
                     to change and I will adjust.",
                    "Great, thanks!",
                    "You're welcome. Ask me anything whenever you're ready.",
                ],
            ),
            (
                "Project ideas",
                2 * HOUR,
                &[
                    "Give me an idea for a weekend project.",
                    "Build a small weather station with a Raspberry Pi and log the data.",
                    "Nice. What hardware do I need?",
                    "A Raspberry Pi Zero 2 W, a BME280 sensor for temperature, humidity, \
                     and pressure, a microSD card, jumper wires, and a 5 V power supply. \
                     A small weatherproof box helps if it goes outside.",
                    "How do I read the sensor?",
                    "Enable I²C with raspi-config, wire the BME280 to 3.3 V, GND, SDA, \
                     and SCL, then use a library such as bme280 in Python to read the \
                     values every minute.",
                    "And where do I store the readings?",
                    "Start with SQLite: one table with a timestamp and a column per \
                     reading. Later you can chart it with Grafana or a small web page.",
                    "Could I add a rain gauge later?",
                    "Yes. A tipping-bucket gauge closes a reed switch on each tip, so \
                     wire it to a GPIO pin and count the pulses. Each tip is usually \
                     about 0.28 mm of rain.",
                ],
            ),
            (
                "Weekend plans",
                DAY,
                &[
                    "Where can I go hiking near the city?",
                    "Try the river trail: it is 12 km long and mostly flat.",
                    "Is it good for kids?",
                    "Yes. The path is wide and well marked, and there are benches and a \
                     playground about halfway, near the old mill.",
                    "How long does it take?",
                    "Most families walk it in three to four hours with breaks. You can \
                     also turn back at the mill for a 6 km loop.",
                    "What should we bring?",
                    "Water, snacks, sunscreen, a light rain jacket, and comfortable \
                     shoes. The café at the mill is open on weekends if you want lunch.",
                ],
            ),
            (
                "Book recommendations",
                4 * DAY,
                &[
                    "Recommend a science fiction book.",
                    "Read \"The Left Hand of Darkness\" by Ursula K. Le Guin.",
                    "What is it about?",
                    "An envoy visits Gethen, a frozen planet whose people have no fixed \
                     sex, to invite it into an interstellar alliance. It is a story \
                     about trust, politics, and friendship across a deep cultural gap.",
                    "I liked it. Something similar?",
                    "Try \"The Dispossessed\", also by Le Guin, or \"A Memory Called \
                     Empire\" by Arkady Martine. Both focus on culture and politics \
                     more than on technology.",
                    "Anything shorter?",
                    "\"All Systems Red\" by Martha Wells is about 150 pages: a security \
                     robot that hacked its own governor and just wants to watch its \
                     shows in peace.",
                ],
            ),
            (
                "Trip to Lisbon",
                12 * DAY,
                &[
                    "What should I see in Lisbon?",
                    "Visit Alfama, ride tram 28, and see the sunset from a miradouro.",
                    "I have three days. Can you make a plan?",
                    "Day 1: Alfama, the castle, and tram 28 to Graça. Day 2: Belém for \
                     the tower, the Jerónimos Monastery, and pastéis de nata. Day 3: a \
                     day trip to Sintra for Pena Palace and the Moorish Castle.",
                    "Where should I stay?",
                    "Baixa or Chiado are central, flat by Lisbon standards, and close to \
                     the metro. Príncipe Real is quieter and a little more upscale.",
                    "Any food I must try?",
                    "Grilled sardines, bacalhau à brás, bifana sandwiches, and ginjinha, \
                     a sour cherry liqueur served in small cups near Rossio.",
                    "Do I need a car?",
                    "No. Use the metro, trams, and trains; the train to Sintra leaves \
                     from Rossio station and takes about 40 minutes.",
                ],
            ),
            (
                "Tax return",
                70 * DAY,
                &[
                    "Which documents do I need for my tax return?",
                    "You need your income statements, receipts for deductions, and your ID.",
                    "What counts as a deduction?",
                    "Common ones are work expenses, pension contributions, donations to \
                     registered charities, and some healthcare costs. The rules vary by \
                     country, so check your tax authority's list.",
                    "When is the deadline?",
                    "It depends on your country. Many have a spring deadline for the \
                     previous year, with an extension if you use a tax advisor.",
                    "Should I hire an advisor?",
                    "If you have only a salary, you can usually file yourself. If you \
                     are self-employed, own rental property, or have foreign income, \
                     an advisor often saves more than they cost.",
                ],
            ),
            (
                "Moving checklist",
                400 * DAY,
                &[
                    "Make a short moving checklist.",
                    "Book movers, pack by room, change your address, and read the meters.",
                    "What should I do first?",
                    "Book the movers six to eight weeks ahead, since good ones fill up \
                     fast, especially at the end of the month.",
                    "Any tips for packing?",
                    "Start with rooms you use least. Label each box with its room and \
                     contents, keep heavy items in small boxes, and pack one essentials \
                     box with chargers, toiletries, and a kettle.",
                    "Who needs my new address?",
                    "Your bank, employer, insurance, doctor, the tax office, and any \
                     subscriptions. Set up mail forwarding to catch the rest.",
                ],
            ),
        ];
        let mut sessions: Vec<_> = samples
            .into_iter()
            .zip(1..)
            .map(|((title, age, messages), id)| Session {
                id,
                title: title.to_owned(),
                model: Model::default(),
                updated_at: now - Duration::from_secs(age),
                chat: Chat {
                    messages: messages
                        .iter()
                        .zip([Role::User, Role::Assistant].into_iter().cycle())
                        .map(|(content, role)| Message {
                            role,
                            content: (*content).to_owned(),
                        })
                        .collect(),
                },
            })
            .collect();
        sessions.sort_by(|a, b| b.updated_at.cmp(&a.updated_at));
        Self {
            state: Mutex::new(State {
                sessions,
                ..Default::default()
            }),
        }
    }

    /// A snapshot of the current state.
    pub fn state(&self) -> State {
        self.state
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .clone()
    }

    /// Applies `action` and returns the new state.
    pub fn dispatch(&self, action: Action) -> State {
        let mut state = self.state.lock().unwrap_or_else(PoisonError::into_inner);
        *state = reduce(mem::take(&mut state), action);
        state.clone()
    }

    /// Opens a view ([`Action::OpenView`]) and returns its id, for the
    /// window that shows it.
    pub fn open_view(&self, session_id: Option<u64>) -> u64 {
        let mut state = self.state.lock().unwrap_or_else(PoisonError::into_inner);
        *state = reduce(mem::take(&mut state), Action::OpenView { session_id });
        state.views.last().expect("the view was just opened").id
    }

    /// The sessions grouped for the sidebar, by the local calendar day they
    /// were last updated, as seen at `now`.
    pub fn session_sections(&self, now: SystemTime) -> Vec<SessionSection> {
        let state = self.state.lock().unwrap_or_else(PoisonError::into_inner);
        session_sections(&state.sessions, now)
    }
}

impl Default for Store {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_sessions_are_newest_first() {
        let sessions = Store::new().state().sessions;
        assert!(!sessions.is_empty());
        assert!(sessions.is_sorted_by(|a, b| a.updated_at >= b.updated_at));
    }
}
