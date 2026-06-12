//! Gameplay state flow, scheduling, and public request API.

pub mod api;
pub mod cleanup;
pub mod control;
pub mod interaction;
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
use self::state::StatePlugin;

pub struct GameplayPlugin {
    request_inbox: Option<api::RuntimeRequestInbox>,
    update_sender: Option<api::RuntimeUpdateSender>,
}

impl GameplayPlugin {
    pub fn new(
        request_inbox: api::RuntimeRequestInbox,
        update_sender: api::RuntimeUpdateSender,
    ) -> Self {
        Self {
            request_inbox: Some(request_inbox),
            update_sender: Some(update_sender),
        }
    }

    pub fn without_external_manager() -> Self {
        Self {
            request_inbox: None,
            update_sender: None,
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
        if let Some(update_sender) = self.update_sender.clone() {
            app.insert_resource(update_sender);
        }

        app.add_plugins((
            PrefabPlugin,
            GameplayApiPlugin,
            StatePlugin,
            SchedulePlugin,
            CleanupPlugin,
            LifecyclePlugin,
            IntentPlugin,
        ));
    }
}
