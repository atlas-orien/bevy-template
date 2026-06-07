pub mod flow;
pub mod rules;
pub mod spawning;

pub use error::Result;

use bevy::prelude::*;

use self::flow::FlowPlugin;
use self::rules::RulesPlugin;
use self::spawning::SpawningPlugin;

pub struct GameplayPlugin;

impl Plugin for GameplayPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((FlowPlugin, RulesPlugin, SpawningPlugin));
    }
}
