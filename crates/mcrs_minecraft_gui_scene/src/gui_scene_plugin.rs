use bevy::prelude::*;

use crate::gui_config::GuiConfig;
use crate::gui_fade::GuiFade;
use crate::gui_phase::GuiPhase;
use crate::gui_pointer::GuiPointerPlugin;

pub struct GuiScenePlugin {
    pub config: GuiConfig,
}

impl Plugin for GuiScenePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(self.config)
            .init_resource::<GuiFade>()
            .add_plugins(GuiPointerPlugin)
            .configure_sets(PostUpdate, (GuiPhase::Draw, GuiPhase::Batch).chain());
    }
}
