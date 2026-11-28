use bevy_ecs::schedule::SystemSet;

#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ColumnTicksExchange {
    Ask,
    Answer,
    Use,
}
