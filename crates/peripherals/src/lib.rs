pub mod gamepad;
pub mod keyboard;
pub mod mouse;
pub mod ui;

pub use error::Result;

use bevy::prelude::*;

pub struct PeripheralsPlugin;

impl Plugin for PeripheralsPlugin {
    fn build(&self, _app: &mut App) {}
}
