#[cfg(all(feature = "test-support", not(target_family = "wasm")))]
pub mod gui_test_assets;
mod in_game_gradient;
mod irect;
mod item_decorations;
mod item_icons;
mod outlined_box;
mod slot_icon;
mod sprite_draw;
mod text_draw;
mod texture_tile;
mod tooltip;
mod tooltip_layer;

pub use in_game_gradient::{DIM_BOTTOM, DIM_TOP};
pub use irect::{irect, point};
pub use item_decorations::{CountLabel, Decorated, decorations};
pub use item_icons::{ItemIcons, build_icon_meshes, item_icons_changed};
pub use outlined_box::{BLACK, draw_outlined_box};
pub use slot_icon::SlotIcon;
pub use sprite_draw::draw_sprite;
pub use text_draw::{
    Shadow, draw_centered_text, draw_glyphs, draw_str, draw_str_without_shadow, draw_text,
    draw_text_without_shadow, draw_tinted_text, shadow_color,
};
pub use texture_tile::tile_texture;
pub use tooltip::{LineGap, Tooltip};
pub use tooltip_layer::{TooltipLayer, draw_tooltip};
