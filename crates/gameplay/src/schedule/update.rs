use bevy::prelude::*;

use super::sets::GameplayUpdateSet;
use crate::api::systems::{
    consume_gameplay_requests_system, forward_manager_requests_system,
    sync_gameplay_entities_system,
};

pub fn register_update_schedules(app: &mut App) {
    app.add_systems(
        Update,
        (
            forward_manager_requests_system.in_set(GameplayUpdateSet::ReceiveRuntimeRequests),
            consume_gameplay_requests_system.in_set(GameplayUpdateSet::ConsumeRuntimeRequests),
            sync_gameplay_entities_system.in_set(GameplayUpdateSet::SyncRuntimeUpdates),
        ),
    );
}
