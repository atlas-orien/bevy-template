//! Demo 世界相机 bundle 配置。

use bevy::camera::visibility::RenderLayers;
use bevy::prelude::*;

use super::DemoCameraFollow;

const DEMO_WORLD_CAMERA_ORDER: isize = 0;
const DEMO_WORLD_CAMERA_LAYER: usize = 0;
const DEMO_WORLD_CAMERA_Z: f32 = 1000.0;

#[derive(Component, Debug, Clone, Copy, Default, Eq, PartialEq)]
pub(crate) struct DemoWorldCamera2dMarker;

#[derive(Bundle)]
pub struct DemoWorldCamera2d {
    camera_2d: Camera2d,
    camera: Camera,
    render_layers: RenderLayers,
    marker: DemoWorldCamera2dMarker,
    follow: DemoCameraFollow,
    transform: Transform,
}

impl Default for DemoWorldCamera2d {
    fn default() -> Self {
        Self {
            camera_2d: Camera2d,
            camera: Camera {
                order: DEMO_WORLD_CAMERA_ORDER,
                ..default()
            },
            render_layers: RenderLayers::layer(DEMO_WORLD_CAMERA_LAYER),
            marker: DemoWorldCamera2dMarker,
            follow: DemoCameraFollow::default(),
            transform: Transform::from_xyz(0.0, 0.0, DEMO_WORLD_CAMERA_Z),
        }
    }
}
