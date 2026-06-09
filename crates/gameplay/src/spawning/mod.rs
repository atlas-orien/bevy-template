pub mod defaults;
pub mod plan;
pub mod systems;

use bevy::prelude::*;

use crate::state::AppState;
use systems::spawn_gameplay_plan_system;

pub struct SpawningPlugin;

impl Plugin for SpawningPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::MainMenu), enter_main_menu)
            .add_systems(OnEnter(AppState::Playing), spawn_gameplay_plan_system);
    }
}

fn enter_main_menu() {
    info!("Main menu gameplay ready.");
}
