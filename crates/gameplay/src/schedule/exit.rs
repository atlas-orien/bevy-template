use bevy::prelude::*;

use crate::cleanup::despawn_demo_menu_system;
use crate::state::AppState;

pub fn register_exit_schedules(app: &mut App) {
    app.add_systems(OnExit(AppState::MainMenu), despawn_demo_menu_system)
        .add_systems(OnExit(AppState::Paused), despawn_demo_menu_system);
}
