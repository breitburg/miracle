/// A model that a [`crate::Session`] uses.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Model {
    #[default]
    Opus5_5,
    Sonnet5,
    Haiku4_5,
}

impl Model {
    /// Every model, in the order shells list them.
    pub const ALL: [Self; 3] = [Self::Opus5_5, Self::Sonnet5, Self::Haiku4_5];

    /// The API id.
    pub fn id(self) -> &'static str {
        match self {
            Self::Opus5_5 => "claude-opus-5-5",
            Self::Sonnet5 => "claude-sonnet-5",
            Self::Haiku4_5 => "claude-haiku-4-5",
        }
    }
}
