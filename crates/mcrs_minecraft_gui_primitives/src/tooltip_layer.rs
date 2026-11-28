use crate::tooltip::{self, Tooltip};
use bevy::prelude::*;
use mcrs_minecraft_font::Font;
use mcrs_minecraft_gui_atlas::GuiAtlasLayout;
use mcrs_minecraft_gui_scene::GuiBand;
use mcrs_minecraft_gui_scene::GuiLayer;
use mcrs_minecraft_gui_scene::GuiSize;

#[derive(Component, Default)]
#[require(GuiLayer = GuiLayer::new(GuiBand::Overlay, 0))]
pub struct TooltipLayer;

pub fn draw_tooltip(
    atlas: Option<Res<GuiAtlasLayout>>,
    font: Option<Res<Font>>,
    size: Res<GuiSize>,
    tooltips: Query<(&GuiLayer, &Tooltip), Without<TooltipLayer>>,
    mut layer: Single<&mut GuiLayer, With<TooltipLayer>>,
) {
    let out = layer.redraw();
    let (Some(atlas), Some(font)) = (atlas, font) else {
        return;
    };
    let first = tooltips
        .iter()
        .filter(|(_, tooltip)| tooltip.lines().is_some())
        .min_by_key(|(layer, _)| layer.order());
    let Some((_, request)) = first else {
        return;
    };
    tooltip::draw(request, *size, &atlas, &font, out);
}
