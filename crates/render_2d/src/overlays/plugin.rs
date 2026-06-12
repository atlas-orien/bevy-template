use bevy::prelude::*;

use super::demo_health_bar::demo_health_bar_system;

pub struct OverlaysPlugin;

impl Plugin for OverlaysPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, demo_health_bar_system);
    }
}
