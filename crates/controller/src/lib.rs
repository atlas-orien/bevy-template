pub mod ai;
pub mod gamepad;
pub mod keyboard;
pub mod network;
pub mod script;

pub use error::Result;

use bevy::prelude::*;

use self::keyboard::KeyboardControllerPlugin;

pub struct ControllerPlugin;

impl Plugin for ControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(KeyboardControllerPlugin);
    }
}
