use bevy::prelude::*;

use super::layered::layered_background_parallax_system;

pub struct BackgroundPlugin;

impl Plugin for BackgroundPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, layered_background_parallax_system);
    }
}
