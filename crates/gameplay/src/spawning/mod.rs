pub mod defaults;
pub mod plan;
pub mod prefab;
pub mod systems;

use bevy::prelude::*;

use crate::state::AppState;
use systems::{spawn_gameplay_plan_system, spawn_requested_prefabs_system};

pub struct SpawningPlugin;

impl Plugin for SpawningPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::MainMenu), enter_main_menu)
            .add_systems(OnEnter(AppState::Playing), spawn_gameplay_plan_system)
            .add_systems(
                Update,
                spawn_requested_prefabs_system.run_if(in_state(AppState::Playing)),
            );
    }
}

fn enter_main_menu() {
    info!("Main menu gameplay ready.");
}
