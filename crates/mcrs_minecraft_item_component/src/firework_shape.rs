#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum FireworkShape {
    SmallBall,
    LargeBall,
    Star,
    Creeper,
    Burst,
}

impl FireworkShape {
    pub const ALL: [Self; 5] = [
        Self::SmallBall,
        Self::LargeBall,
        Self::Star,
        Self::Creeper,
        Self::Burst,
    ];

    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::SmallBall => "small_ball",
            Self::LargeBall => "large_ball",
            Self::Star => "star",
            Self::Creeper => "creeper",
            Self::Burst => "burst",
        }
    }
}
