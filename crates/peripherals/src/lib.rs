pub mod gamepad;
pub mod keyboard;
pub mod local_input;
pub mod mouse;

pub use error::Result;
pub use local_input::{LocalInputAction, LocalInputContext};

use bevy::prelude::*;

use self::keyboard::emit_keyboard_ui_navigation_system;

pub struct PeripheralsPlugin;

impl Plugin for PeripheralsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreUpdate, emit_keyboard_ui_navigation_system);
    }
}
