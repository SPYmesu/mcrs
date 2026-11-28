use bevy_ecs::entity::Entity;
use bevy_ecs::message::Message;
use mcrs_minecraft_region_disk::column_ticks::ColumnTicks;
use mcrs_voxel::ColumnPos;

#[derive(Message, Clone, Debug, PartialEq, Eq)]
pub struct ColumnTicksAnswer {
    pub entity: Entity,
    pub column: ColumnPos,
    pub ticks: ColumnTicks,
    pub changed: bool,
}
