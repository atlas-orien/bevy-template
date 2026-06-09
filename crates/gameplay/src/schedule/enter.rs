use bevy::prelude::*;

use crate::spawning::initial::spawn_initial_gameplay_plan_system;
use crate::state::AppState;

pub fn register_enter_schedules(app: &mut App) {
    app.add_systems(OnEnter(AppState::MainMenu), enter_main_menu)
        .add_systems(
            OnEnter(AppState::Playing),
            spawn_initial_gameplay_plan_system,
        );
}

fn enter_main_menu() {
    info!("Main menu gameplay ready.");
}
