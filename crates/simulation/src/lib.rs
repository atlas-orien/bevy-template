pub mod flow;
pub mod movement;
pub mod spawning;

pub use error::Result;

use bevy::prelude::*;

use self::flow::FlowPlugin;
use self::movement::MovementPlugin;
use self::spawning::SpawningPlugin;

pub struct SimulationPlugin;

impl Plugin for SimulationPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((FlowPlugin, SpawningPlugin, MovementPlugin));
    }
}
