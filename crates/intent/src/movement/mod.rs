use bevy::prelude::*;
use error::{ErrorKind, GameError, Result};
use prefab::intent::set_movement_target;
pub use prefab::intent::{MovementIntentQuery, MovementTarget};

pub struct MovementIntentPlugin;

impl Plugin for MovementIntentPlugin {
    fn build(&self, _app: &mut App) {}
}

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

    set_movement_target(&mut movement_intent, target);
    Ok(())
}
