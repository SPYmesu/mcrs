use bevy::prelude::*;

#[derive(Resource, Default, Deref, DerefMut)]
pub struct DebugOverlay(pub bool);

#[derive(Resource, Default, PartialEq)]
pub struct DebugModifier {
    pub used: bool,
}

#[derive(SystemSet, Clone, Debug, PartialEq, Eq, Hash)]
pub struct DebugOverlayToggle;
