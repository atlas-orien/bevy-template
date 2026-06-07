pub mod player;

pub use error::Result;

use bevy::prelude::*;

use self::player::PlayerControllerPlugin;

pub struct ControllerPlugin;

impl Plugin for ControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PlayerControllerPlugin);
    }
}
