use bevy::camera::visibility::RenderLayers;
use bevy::prelude::*;

use crate::camera::SceneCamera2d;

const FIXED_CAMERA_2D_ORDER: isize = 0;
const FIXED_CAMERA_2D_LAYER: usize = 0;
const FIXED_CAMERA_2D_Z: f32 = 1000.0;

#[derive(Bundle)]
pub struct FixedCamera2dBundle {
    camera_2d: Camera2d,
    camera: Camera,
    render_layers: RenderLayers,
    marker: SceneCamera2d,
    transform: Transform,
}

impl Default for FixedCamera2dBundle {
    fn default() -> Self {
        Self {
            camera_2d: Camera2d,
            camera: Camera {
                order: FIXED_CAMERA_2D_ORDER,
                ..default()
            },
            render_layers: RenderLayers::layer(FIXED_CAMERA_2D_LAYER),
            marker: SceneCamera2d,
            transform: Transform::from_xyz(0.0, 0.0, FIXED_CAMERA_2D_Z),
        }
    }
}
