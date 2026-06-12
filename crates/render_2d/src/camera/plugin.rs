use bevy::prelude::*;

use super::demo_camera_follow::demo_camera_follow_system;

pub struct Camera2dPlugin;

impl Plugin for Camera2dPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostUpdate, demo_camera_follow_system);
    }
}
