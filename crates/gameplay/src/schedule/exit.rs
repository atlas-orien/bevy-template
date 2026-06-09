use bevy::prelude::*;
use prefab::lifecycle::despawn_gameplay_prefabs_system;

use crate::state::AppState;

pub fn register_exit_schedules(app: &mut App) {
    app.add_systems(OnExit(AppState::Playing), despawn_gameplay_prefabs_system);
}
