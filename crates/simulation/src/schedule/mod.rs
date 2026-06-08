use bevy::prelude::*;
use ecs::systems::movement::movement_system;

use crate::state::AppState;

pub struct SchedulePlugin;

impl Plugin for SchedulePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, movement_system.run_if(in_state(AppState::Playing)));
    }
}
