use bevy::prelude::*;

use crate::components::world::gameplay::GameplaySessionEntity;

pub fn despawn_gameplay_entities_system(
    mut commands: Commands,
    entities: Query<Entity, With<GameplaySessionEntity>>,
) {
    for entity in &entities {
        commands.entity(entity).despawn();
    }
}
