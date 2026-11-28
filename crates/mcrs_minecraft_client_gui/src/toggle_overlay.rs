use bevy::prelude::*;
use mcrs_minecraft_gui_scene::{DebugModifier, DebugOverlay};

pub fn toggle_overlay(
    keys: Res<ButtonInput<KeyCode>>,
    mut modifier: ResMut<DebugModifier>,
    mut overlay: ResMut<DebugOverlay>,
) {
    if keys.just_released(KeyCode::F3) && !std::mem::take(&mut modifier.used) {
        **overlay = !**overlay;
    }
}
