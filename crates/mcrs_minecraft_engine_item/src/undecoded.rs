use alloc::boxed::Box;

use bevy_ecs::component::Component;
use mcrs_minecraft_item_component::PatchEntry;

#[derive(Component, Clone, Debug, PartialEq, Hash)]
pub(crate) struct Undecoded(pub(crate) Box<[PatchEntry]>);
