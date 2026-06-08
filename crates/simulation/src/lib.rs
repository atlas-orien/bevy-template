pub mod cleanup;
pub mod schedule;
pub mod spawning;
pub mod state;

pub use error::Result;

use bevy::prelude::*;

use self::cleanup::CleanupPlugin;
use self::schedule::SchedulePlugin;
use self::spawning::SpawningPlugin;
use self::state::StatePlugin;

pub struct SimulationPlugin;

impl Plugin for SimulationPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((StatePlugin, SchedulePlugin, SpawningPlugin, CleanupPlugin));
    }
}
