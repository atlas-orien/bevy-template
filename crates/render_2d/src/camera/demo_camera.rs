use bevy::camera::visibility::RenderLayers;
use bevy::prelude::*;

use super::DemoCameraFollow;

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
                order: 0,
                ..default()
            },
            render_layers: RenderLayers::layer(0),
            marker: DemoWorldCamera2d,
            follow: DemoCameraFollow::default(),
            transform: Transform::from_xyz(0.0, 0.0, 1000.0),
        }
    }
}
