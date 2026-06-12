//! 消费本机语义输入，把移动和暂停落到受控实体与 PauseState。

use bevy::prelude::*;
use intent::movement::{MovementIntentQuery, MovementTarget, set_movement_intent};
use prefab::control::{LocallyControlledQuery, find_locally_controlled_entity};

use crate::api::LocalUserInputMessage;
use crate::state::PauseState;

pub fn apply_demo_local_user_input_system(
    mut local_inputs: MessageReader<LocalUserInputMessage>,
    controlled: LocallyControlledQuery,
    mut movement_intents: MovementIntentQuery,
    pause_state: Res<State<PauseState>>,
    mut next_pause_state: ResMut<NextState<PauseState>>,
) {
    for input in local_inputs.read() {
        match *input {
            LocalUserInputMessage::Move(direction) => {
                if pause_state.get() != &PauseState::Running {
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
            LocalUserInputMessage::TogglePause => {
                next_pause_state.set(match pause_state.get() {
                    PauseState::Running => PauseState::Paused,
                    PauseState::Paused => PauseState::Running,
                });
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use bevy::state::app::StatesPlugin;

    use super::*;
    use crate::state::AppState;

    fn input_app(pause_state: PauseState) -> App {
        let mut app = App::new();
        app.add_plugins(StatesPlugin)
            .init_state::<AppState>()
            .add_sub_state::<PauseState>()
            .add_message::<LocalUserInputMessage>()
            .add_systems(Update, apply_demo_local_user_input_system);
        app.world_mut()
            .resource_mut::<NextState<AppState>>()
            .set(AppState::Playing);
        app.update();
        app.world_mut()
            .resource_mut::<NextState<PauseState>>()
            .set(pause_state);
        app.update();
        app
    }

    #[test]
    fn toggle_pause_switches_running_to_paused() {
        let mut app = input_app(PauseState::Running);
        app.world_mut()
            .write_message(LocalUserInputMessage::TogglePause);

        app.update();
        app.update();

        assert_eq!(
            *app.world().resource::<State<PauseState>>().get(),
            PauseState::Paused
        );
    }

    #[test]
    fn toggle_pause_switches_paused_to_running() {
        let mut app = input_app(PauseState::Paused);
        app.world_mut()
            .write_message(LocalUserInputMessage::TogglePause);

        app.update();
        app.update();

        assert_eq!(
            *app.world().resource::<State<PauseState>>().get(),
            PauseState::Running
        );
    }
}
