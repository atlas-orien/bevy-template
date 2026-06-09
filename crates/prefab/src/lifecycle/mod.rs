use bevy::prelude::*;
use ecs::components::world::gameplay::GameplaySessionEntity;

pub use ecs::systems::lifecycle::despawn_gameplay_entities_system as despawn_gameplay_prefabs_system;

pub type GameplaySessionEntities<'world, 'state> =
    Query<'world, 'state, Entity, With<GameplaySessionEntity>>;

pub fn despawn_gameplay_prefabs(
    commands: &mut Commands,
    entities: &GameplaySessionEntities<'_, '_>,
) {
    for entity in entities {
        commands.entity(entity).try_despawn();
    }
}
