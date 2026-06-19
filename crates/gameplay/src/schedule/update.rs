use bevy::prelude::*;

use super::sets::GameplayUpdateSet;
use crate::api::systems::{
    consume_gameplay_requests_system, forward_manager_requests_system,
    sync_gameplay_entities_system,
};
use crate::control::{apply_demo_local_user_input_system, handle_demo_sensor_triggered_system};
use crate::interaction::{handle_demo_ui_interactions_system, handle_demo_ui_navigation_system};
use crate::state::{AppState, PauseState};

pub fn register_update_schedules(app: &mut App) {
    app.add_systems(
        Update,
        (
            forward_manager_requests_system.in_set(GameplayUpdateSet::ReceiveRuntimeRequests),
            consume_gameplay_requests_system.in_set(GameplayUpdateSet::ConsumeRuntimeRequests),
            apply_demo_local_user_input_system
                .run_if(in_state(AppState::Playing))
                .in_set(GameplayUpdateSet::ApplyLocalInput),
            prefab::movement::movement_system
                .run_if(in_state(PauseState::Running))
                .in_set(GameplayUpdateSet::Movement),
            (
                handle_demo_sensor_triggered_system.run_if(in_state(PauseState::Running)),
                (
                    handle_demo_ui_interactions_system,
                    handle_demo_ui_navigation_system,
                )
                    .run_if(
                        in_state(AppState::MainMenu)
                            .or(in_state(AppState::Playing))
                            .or(in_state(AppState::GameOver)),
                    ),
            )
                .in_set(GameplayUpdateSet::GameplayRules),
            sync_gameplay_entities_system.in_set(GameplayUpdateSet::SyncRuntimeUpdates),
        ),
    );
}
