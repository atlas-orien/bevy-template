pub mod api;
pub mod cleanup;
pub mod lifecycle;
pub mod schedule;
pub mod spawning;
pub mod state;

pub use error::Result;

use bevy::prelude::*;
use intent::IntentPlugin;
use prefab::PrefabPlugin;

use self::api::GameplayApiPlugin;
use self::cleanup::CleanupPlugin;
use self::lifecycle::LifecyclePlugin;
use self::schedule::SchedulePlugin;
use self::spawning::SpawningPlugin;
use self::state::StatePlugin;

pub struct GameplayPlugin;

impl Plugin for GameplayPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            PrefabPlugin,
            GameplayApiPlugin,
            StatePlugin,
            SchedulePlugin,
            SpawningPlugin,
            CleanupPlugin,
            LifecyclePlugin,
            IntentPlugin,
        ));
    }
}
