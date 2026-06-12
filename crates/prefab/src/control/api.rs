use bevy::prelude::*;

pub use ecs::components::characters::DemoPlayerControlled;

pub type LocallyControlledQuery<'world, 'state> =
    Query<'world, 'state, Entity, With<DemoPlayerControlled>>;

pub fn find_locally_controlled_entity(
    controlled: &LocallyControlledQuery<'_, '_>,
) -> Option<Entity> {
    controlled.iter().next()
}
