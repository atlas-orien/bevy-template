use bevy::prelude::*;

pub use ecs::components::base::MovementTarget;

use ecs::components::base::{MovementIntent, MovementTarget as EcsMovementTarget};

pub type MovementIntentQuery<'world, 'state> = Query<'world, 'state, &'static mut MovementIntent>;

pub fn set_movement_target(movement_intent: &mut MovementIntent, target: EcsMovementTarget) {
    movement_intent.target = target;
}
