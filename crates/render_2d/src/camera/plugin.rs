use bevy::prelude::*;

use super::follow::follow_camera_system;

pub struct Camera2dPlugin;

impl Plugin for Camera2dPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostUpdate, follow_camera_system);
    }
}
