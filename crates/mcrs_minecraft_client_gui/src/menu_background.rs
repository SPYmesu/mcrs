use bevy::math::IVec2;
use mcrs_minecraft_menu_geometry::{BackgroundPart, MenuGeometry};
use mcrs_minecraft_menu_key::menu_keys::MenuType;

use mcrs_minecraft_gui_atlas::StitchRequest;

pub(crate) fn container_backgrounds() -> impl Iterator<Item = StitchRequest> {
    MenuType::ALL
        .iter()
        .filter_map(|kind| MenuGeometry::of(*kind))
        .chain([MenuGeometry::INVENTORY])
        .flat_map(|geometry| geometry.background())
        .filter_map(background_request)
}

pub(crate) fn background_request(part: BackgroundPart) -> Option<StitchRequest> {
    let extent = IVec2::new(
        part.source.x.saturating_add(part.source.width),
        part.source.y.saturating_add(part.source.height),
    )
    .max(IVec2::ZERO)
    .as_uvec2();
    StitchRequest::background(part.texture, extent)
}

#[cfg(test)]
mod tests {
    use super::*;
    use mcrs_minecraft_gui_atlas::{BLANK, GUI_TEXTURES, GuiAtlasParts};

    use crate::menu_test_app::{ATLAS, stitch_requests};
    use mcrs_minecraft_gui_primitives::gui_test_assets::corpus_font;

    #[test]
    fn the_gui_atlas_holds_every_region_and_the_digit_widths() {
        let GuiAtlasParts {
            layout: atlas,
            atlas: image,
            glint,
        } = &*ATLAS;
        for name in GUI_TEXTURES
            .iter()
            .filter(|n| **n != "misc/enchanted_glint_item")
        {
            assert!(atlas.find_region(name).is_some(), "{name}");
        }
        for request in stitch_requests().iter() {
            let region = request.region();
            assert!(atlas.find_region(region).is_some(), "{region}");
            if let StitchRequest::Background { extent, .. } = request {
                assert_eq!(atlas.region(region).size().as_uvec2(), extent, "{region}");
            }
        }
        assert_eq!(
            atlas.region("container/generic_54").size(),
            IVec2::new(176, 222)
        );
        assert_eq!(
            atlas
                .region("container/creative_inventory/tab_items")
                .size(),
            IVec2::new(195, 136)
        );
        assert_eq!(atlas.region("hud/hotbar").size(), IVec2::new(182, 22));
        assert_eq!(
            atlas.region("container/inventory").size(),
            IVec2::new(176, 166)
        );
        assert_eq!(
            atlas.region("container/gamemode_switcher").size(),
            IVec2::new(125, 75)
        );
        assert_eq!(
            atlas.region("gamemode_switcher/slot").size(),
            IVec2::new(26, 26)
        );
        assert_eq!(corpus_font().advance('7', false), 6.0);
        assert_eq!(glint.width(), 128);
        assert!(image.height() <= 2048);
        let blank = atlas.region(BLANK);
        let start = (blank.min.y as usize * image.width() as usize + blank.min.x as usize) * 4;
        let pixels = image.data.as_deref().unwrap();
        assert_eq!(&pixels[start..start + 4], &[255, 255, 255, 255]);
    }
}
