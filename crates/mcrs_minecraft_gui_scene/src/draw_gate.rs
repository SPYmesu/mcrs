use bevy::prelude::*;
use mcrs_minecraft_font::Font;
use mcrs_minecraft_gui_atlas::GuiAtlasLayout;

use crate::gui_size::GuiSize;
use crate::layer_due::layer_due;

pub fn gui_frame_changed() -> impl SystemCondition<()> {
    resource_changed::<GuiSize>
        .or_else(resource_exists_and_changed::<GuiAtlasLayout>)
        .or_else(resource_exists_and_changed::<Font>)
}

pub fn changed_or_removed<C: Component>() -> impl SystemCondition<()> {
    any_match_filter::<Changed<C>>.or_else(any_component_removed::<C>)
}

pub fn layer_stale<Kind: Component>() -> impl SystemCondition<()> {
    any_match_filter::<Added<Kind>>
        .or_else(gui_frame_changed())
        .or_else(layer_due::<Kind>)
}
