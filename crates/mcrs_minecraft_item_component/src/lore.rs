use alloc::borrow::Cow;
use core::hash::Hash;
use core::hash::Hasher;

use mcrs_minecraft_text::Text;

use crate::Neutral;

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "bevy", derive(bevy_ecs::component::Component))]
pub struct Lore {
    pub lines: Cow<'static, [Text]>,
}

impl Lore {
    pub const MAX_LINES: usize = 256;

    #[must_use]
    pub fn new(lines: impl Into<Cow<'static, [Text]>>) -> Option<Self> {
        let lines: Cow<'static, [Text]> = lines.into();
        (lines.len() <= Self::MAX_LINES).then_some(Self { lines })
    }
}

impl Hash for Lore {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_usize(self.lines.len());
    }
}

impl Neutral for Lore {
    const NEUTRAL: &'static Self = &Self {
        lines: Cow::Borrowed(&[]),
    };
}
