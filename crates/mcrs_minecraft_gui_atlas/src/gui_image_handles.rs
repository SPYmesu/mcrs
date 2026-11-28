use bevy::prelude::*;

#[derive(Resource)]
pub struct GuiImageHandles {
    pub(crate) atlas: Handle<Image>,
    pub(crate) glint: Handle<Image>,
}
