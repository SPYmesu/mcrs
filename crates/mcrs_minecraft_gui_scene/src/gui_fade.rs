use bevy::prelude::*;

use crate::gui_quad::GuiQuad;

#[derive(Resource, Clone, Copy, Debug, PartialEq, Eq)]
pub struct GuiFade(u32);

impl GuiFade {
    pub const OPAQUE: Self = Self(255);

    #[must_use]
    pub const fn new(alpha: u32) -> Self {
        Self(alpha)
    }

    pub fn apply(self, quad: &mut GuiQuad) {
        if self.0 >= 255 {
            return;
        }
        let alpha = self.0;
        let scale = |color: &mut u32| {
            let faded = (*color >> 24) * alpha / 255;
            *color = (*color & 0x00FF_FFFF) | (faded << 24);
        };
        match quad {
            GuiQuad::Sprite { color, .. }
            | GuiQuad::Fill { color, .. }
            | GuiQuad::Texels { color, .. }
            | GuiQuad::Glyph { color, .. } => scale(color),
            GuiQuad::FillGradient { top, bottom, .. } => {
                scale(top);
                scale(bottom);
            }
            GuiQuad::Icon { .. }
            | GuiQuad::Figure { .. }
            | GuiQuad::Highlight { .. }
            | GuiQuad::Clip { .. }
            | GuiQuad::EndClip => {}
        }
    }
}

impl Default for GuiFade {
    fn default() -> Self {
        Self::OPAQUE
    }
}
