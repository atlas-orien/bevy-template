use bevy::prelude::*;

use crate::components::world::gameplay::GameplaySessionEntityMarker;

pub fn despawn_gameplay_entities_system(
    mut commands: Commands,
    entities: Query<Entity, With<GameplaySessionEntityMarker>>,
) {
    for entity in &entities {
        commands.entity(entity).despawn();
    }
}
