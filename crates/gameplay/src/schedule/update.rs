use bevy::prelude::*;

use super::sets::GameplayUpdateSet;
use crate::api::systems::{
    consume_gameplay_requests_system, forward_manager_requests_system,
    sync_gameplay_entities_system,
};
use crate::control::apply_demo_local_user_input_system;
use crate::interaction::{handle_demo_ui_interactions_system, handle_demo_ui_navigation_system};
use crate::state::AppState;

pub fn register_update_schedules(app: &mut App) {
    app.add_systems(
        Update,
        (
            forward_manager_requests_system.in_set(GameplayUpdateSet::ReceiveRuntimeRequests),
            consume_gameplay_requests_system.in_set(GameplayUpdateSet::ConsumeRuntimeRequests),
            (
                apply_demo_local_user_input_system,
                prefab::movement::movement_system.run_if(in_state(AppState::Playing)),
                handle_demo_ui_interactions_system,
                handle_demo_ui_navigation_system,
            )
                .in_set(GameplayUpdateSet::GameplayRules),
            sync_gameplay_entities_system.in_set(GameplayUpdateSet::SyncRuntimeUpdates),
        ),
    );
}
