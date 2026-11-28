use std::ops::Range;

use bevy_ecs::entity::Entity;
use bevy_ecs::message::Message;
use mcrs_voxel::ColumnPos;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TickHandoff {
    Copy,
    Take,
}

#[derive(Message, Clone, Debug, PartialEq, Eq)]
pub struct ColumnTicksAsk {
    pub column: ColumnPos,
    pub entity: Entity,
    pub rows: Range<i32>,
    pub handoff: TickHandoff,
}
