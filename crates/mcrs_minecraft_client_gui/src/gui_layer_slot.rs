use bevy::prelude::Entity;

use mcrs_minecraft_gui_scene::GuiLayerOrder;

#[derive(Clone, Copy)]
pub(crate) struct LayerSlot {
    pub(crate) order: GuiLayerOrder,
    pub(crate) entity: Entity,
}
