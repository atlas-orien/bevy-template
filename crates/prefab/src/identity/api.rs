use bevy::prelude::*;

pub use ecs::components::world::gameplay::GameplayEntityId;

pub type GameplayEntityIdEntities<'world, 'state> =
    Query<'world, 'state, (Entity, &'static GameplayEntityId)>;

pub fn find_gameplay_entity(
    id: GameplayEntityId,
    entities: &GameplayEntityIdEntities<'_, '_>,
) -> Option<Entity> {
    entities
        .iter()
        .find_map(|(entity, entity_id)| (*entity_id == id).then_some(entity))
}
