use bevy::prelude::Component;
use mcrs_minecraft_item_icon::{IconSources, ItemIcon, StackOverlay, resolve};
use mcrs_minecraft_model::item_model_asset::DisplayContext;
use mcrs_minecraft_model::item_model_context::ItemModelContext;
use mcrs_minecraft_model::stack_view::StackView;

const GUI: ItemModelContext = ItemModelContext::new(DisplayContext::Gui);

#[derive(Component, Clone, Copy, Debug, PartialEq, Eq)]
pub struct SlotIcon {
    pub icon: ItemIcon,
    pub overlay: StackOverlay,
}

impl SlotIcon {
    #[must_use]
    pub fn of(view: StackView<'_>, sources: &IconSources<'_>) -> Self {
        Self {
            icon: resolve(view, sources, &GUI),
            overlay: StackOverlay::of(view),
        }
    }
}
