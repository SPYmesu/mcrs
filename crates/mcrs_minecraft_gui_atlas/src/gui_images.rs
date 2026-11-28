use bevy::asset::AssetId;
use bevy::math::UVec2;
use bevy::prelude::*;

#[derive(Resource, Clone, Copy)]
pub struct GuiImages {
    atlas: AssetId<Image>,
    glint: AssetId<Image>,
    glyph_origin: UVec2,
}

impl GuiImages {
    #[must_use]
    pub const fn new(atlas: AssetId<Image>, glint: AssetId<Image>, glyph_origin: UVec2) -> Self {
        Self {
            atlas,
            glint,
            glyph_origin,
        }
    }

    #[must_use]
    pub const fn atlas(&self) -> AssetId<Image> {
        self.atlas
    }

    #[must_use]
    pub const fn glint(&self) -> AssetId<Image> {
        self.glint
    }

    #[must_use]
    pub const fn glyph_origin(&self) -> UVec2 {
        self.glyph_origin
    }
}
