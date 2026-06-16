use bevy::prelude::*;

use super::demo::demo_pixel_snap_system;

pub struct PixelPlugin;

impl Plugin for PixelPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostUpdate, demo_pixel_snap_system);
    }
}
