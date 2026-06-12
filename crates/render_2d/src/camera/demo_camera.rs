use bevy::camera::visibility::RenderLayers;
use bevy::prelude::*;

use super::DemoCameraFollow;

const DEMO_WORLD_CAMERA_ORDER: isize = 0;
const DEMO_WORLD_CAMERA_LAYER: usize = 0;
const DEMO_WORLD_CAMERA_Z: f32 = 1000.0;

#[derive(Component, Debug, Clone, Copy, Default, Eq, PartialEq)]
pub struct DemoWorldCamera2d;

#[derive(Bundle)]
pub struct DemoWorldCamera2dBundle {
    pub camera_2d: Camera2d,
    pub camera: Camera,
    pub render_layers: RenderLayers,
    pub marker: DemoWorldCamera2d,
    pub follow: DemoCameraFollow,
    pub transform: Transform,
}

impl Default for DemoWorldCamera2dBundle {
    fn default() -> Self {
        Self {
            camera_2d: Camera2d,
            camera: Camera {
                order: DEMO_WORLD_CAMERA_ORDER,
                ..default()
            },
            render_layers: RenderLayers::layer(DEMO_WORLD_CAMERA_LAYER),
            marker: DemoWorldCamera2d,
            follow: DemoCameraFollow::default(),
            transform: Transform::from_xyz(0.0, 0.0, DEMO_WORLD_CAMERA_Z),
        }
    }
}
