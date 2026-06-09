use bevy::prelude::*;

#[derive(Message, Debug, Clone, Copy)]
pub struct SpawnedEvent {
    pub entity: Entity,
}

#[derive(Message, Debug, Clone, Copy)]
pub struct DiedEvent {
    pub entity: Entity,
}
