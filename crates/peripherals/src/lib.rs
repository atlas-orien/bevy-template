pub mod gamepad;
pub mod keyboard;
pub mod mouse;

pub use error::Result;

use bevy::prelude::*;

use self::keyboard::emit_keyboard_ui_navigation_system;

pub struct PeripheralsPlugin;

impl Plugin for PeripheralsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreUpdate, emit_keyboard_ui_navigation_system);
    }
}
