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

pub struct GameplayPlugin {
    request_inbox: Option<api::GameplayRequestInbox>,
}

impl GameplayPlugin {
    pub fn new(request_inbox: api::GameplayRequestInbox) -> Self {
        Self {
            request_inbox: Some(request_inbox),
        }
    }

    pub fn without_external_manager() -> Self {
        Self {
            request_inbox: None,
        }
    }
}

impl Default for GameplayPlugin {
    fn default() -> Self {
        Self::without_external_manager()
    }
}

impl Plugin for GameplayPlugin {
    fn build(&self, app: &mut App) {
        if let Some(request_inbox) = self.request_inbox.clone() {
            app.insert_resource(request_inbox);
        }

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
