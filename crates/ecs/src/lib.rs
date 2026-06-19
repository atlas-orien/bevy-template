//! Core ECS data, resources, events, and world-rule systems.

pub mod components;
pub mod events;
pub mod resources;
pub mod systems;

pub use error::Result;

use bevy::prelude::*;

use self::events::EventsPlugin;
use self::resources::ResourcesPlugin;

pub struct EcsPlugin;

impl Plugin for EcsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((ResourcesPlugin, EventsPlugin));
    }
}
