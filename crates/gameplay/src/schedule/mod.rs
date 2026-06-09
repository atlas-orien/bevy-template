use bevy::prelude::*;
use input::local::keyboard_movement_input_system;
use prefab::lifecycle::despawn_gameplay_prefabs_system;
use prefab::world_2d::characters::player::player_2d_movement_system;

use crate::state::AppState;

pub struct SchedulePlugin;

impl Plugin for SchedulePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (keyboard_movement_input_system, player_2d_movement_system)
                .run_if(in_state(AppState::Playing)),
        )
        .add_systems(OnExit(AppState::Playing), despawn_gameplay_prefabs_system);
    }
}
