pub mod components;
pub mod events;
pub mod resources;
pub mod systems;

pub use error::Result;

use bevy::prelude::*;

use self::components::ComponentsPlugin;

pub struct EcsPlugin;

impl Plugin for EcsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ComponentsPlugin);
    }
}
