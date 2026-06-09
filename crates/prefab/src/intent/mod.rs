use bevy::prelude::*;

pub use ecs::components::characters::player::MovementTarget;

use ecs::components::characters::player::{
    LocalPlayerControlled, MovementIntent, MovementTarget as EcsMovementTarget,
};

pub type MovementIntentQuery<'world, 'state> = Query<'world, 'state, &'static mut MovementIntent>;

pub type LocalMovementIntentEntities<'world, 'state> =
    Query<'world, 'state, Entity, (With<LocalPlayerControlled>, With<MovementIntent>)>;

pub fn set_movement_target(movement_intent: &mut MovementIntent, target: EcsMovementTarget) {
    movement_intent.target = target;
}
