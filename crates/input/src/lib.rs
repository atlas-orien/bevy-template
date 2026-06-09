pub mod gameplay_api;
pub mod local;

pub use error::Result;

use bevy::prelude::*;

use self::local::keyboard_movement_input_system;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, keyboard_movement_input_system);
    }
}
