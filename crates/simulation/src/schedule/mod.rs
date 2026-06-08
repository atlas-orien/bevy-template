use bevy::prelude::*;
use input::local::keyboard_movement_input_system;
use prefab::runtime::PrefabRuntimePlugin;

use crate::state::AppState;

pub struct SchedulePlugin;

impl Plugin for SchedulePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PrefabRuntimePlugin::new(AppState::Playing))
            .add_systems(
                Update,
                keyboard_movement_input_system.run_if(in_state(AppState::Playing)),
            );
    }
}
