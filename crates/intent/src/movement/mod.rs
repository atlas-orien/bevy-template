use bevy::prelude::*;
use ecs::components::characters::player::{MovementIntent, MovementTarget};
use error::{ErrorKind, GameError, Result};

pub struct MovementIntentPlugin;

impl Plugin for MovementIntentPlugin {
    fn build(&self, _app: &mut App) {}
}

pub type MovementIntentQuery<'world, 'state> = Query<'world, 'state, &'static mut MovementIntent>;

pub fn set_movement_intent(
    entity: Entity,
    target: MovementTarget,
    movement_intents: &mut MovementIntentQuery,
) -> Result<()> {
    let Ok(mut movement_intent) = movement_intents.get_mut(entity) else {
        return Err(GameError::from_kind(
            ErrorKind::Gameplay,
            "intent.movement.missing_entity",
            "entity cannot receive movement intent",
        ));
    };

    movement_intent.target = target;
    Ok(())
}
