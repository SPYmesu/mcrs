#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum SwingKind {
    None,
    #[default]
    Whack,
    Stab,
}

impl SwingKind {
    pub const ALL: [Self; 3] = [Self::None, Self::Whack, Self::Stab];

    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::Whack => "whack",
            Self::Stab => "stab",
        }
    }
}
