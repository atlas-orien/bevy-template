use bevy::prelude::*;

pub use ecs::components::characters::DemoPlayerControlledMarker;

pub type LocallyControlledQuery<'world, 'state> =
    Query<'world, 'state, Entity, With<DemoPlayerControlledMarker>>;

pub fn find_locally_controlled_entity(
    controlled: &LocallyControlledQuery<'_, '_>,
) -> Option<Entity> {
    controlled.iter().next()
}
