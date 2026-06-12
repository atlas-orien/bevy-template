use bevy::prelude::*;
use intent::movement::{MovementIntentQuery, MovementTarget, set_movement_intent};
use prefab::control::{LocallyControlledQuery, find_locally_controlled_entity};

use crate::api::LocalUserInputMessage;
use crate::state::AppState;

pub fn apply_demo_local_user_input_system(
    mut local_inputs: MessageReader<LocalUserInputMessage>,
    controlled: LocallyControlledQuery,
    mut movement_intents: MovementIntentQuery,
    state: Res<State<AppState>>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    for input in local_inputs.read() {
        match *input {
            LocalUserInputMessage::Move(direction) => {
                if state.get() != &AppState::Playing {
                    continue;
                }

                let Some(entity) = find_locally_controlled_entity(&controlled) else {
                    continue;
                };
                let target = if direction == Vec2::ZERO {
                    MovementTarget::None
                } else {
                    MovementTarget::Direction(direction)
                };

                if let Err(error) = set_movement_intent(entity, target, &mut movement_intents) {
                    warn!("failed to apply demo local movement input: {error}");
                }
            }
            LocalUserInputMessage::TogglePause => match state.get() {
                AppState::Playing => next_state.set(AppState::Paused),
                AppState::Paused => next_state.set(AppState::Playing),
                _ => {}
            },
        }
    }
}
