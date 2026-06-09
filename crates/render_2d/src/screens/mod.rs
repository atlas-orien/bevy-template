pub mod clear_color;

use bevy::prelude::*;

use self::clear_color::setup_screen_clear_color_system;

pub struct ScreensPlugin;

impl Plugin for ScreensPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_screen_clear_color_system);
    }
}
