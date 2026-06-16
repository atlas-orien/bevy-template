use bevy::prelude::*;

use super::demo::demo_parallax_background_system;

pub struct BackgroundPlugin;

impl Plugin for BackgroundPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, demo_parallax_background_system);
    }
}
