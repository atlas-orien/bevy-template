use bevy::prelude::*;
use prefab::world_2d::characters::player::player_2d_movement_system;

use super::sets::GameplayUpdateSet;
use crate::api::systems::{
    consume_gameplay_requests_system, forward_manager_requests_system,
    sync_gameplay_entities_system,
};
use crate::state::AppState;

pub fn register_update_schedules(app: &mut App) {
    app.add_systems(
        Update,
        (
            forward_manager_requests_system.in_set(GameplayUpdateSet::ReceiveRuntimeRequests),
            consume_gameplay_requests_system.in_set(GameplayUpdateSet::ConsumeRuntimeRequests),
            sync_gameplay_entities_system.in_set(GameplayUpdateSet::SyncRuntimeUpdates),
            player_2d_movement_system
                .run_if(in_state(AppState::Playing))
                .in_set(GameplayUpdateSet::GameplayRules),
        ),
    );
}
