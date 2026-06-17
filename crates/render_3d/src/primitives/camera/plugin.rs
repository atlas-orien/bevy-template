use bevy::prelude::*;

use super::presets::{follow_camera_3d_system, orbit_camera_3d_system};

pub struct Camera3dContentPlugin;

impl Plugin for Camera3dContentPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (follow_camera_3d_system, orbit_camera_3d_system));
    }
}
