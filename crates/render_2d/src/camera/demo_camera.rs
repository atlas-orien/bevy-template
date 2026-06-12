use bevy::prelude::*;

#[derive(Component, Debug, Clone, Copy, Default, Eq, PartialEq)]
pub struct DemoWorldCamera2d;

#[derive(Bundle)]
pub struct DemoWorldCamera2dBundle {
    pub camera_2d: Camera2d,
    pub camera: Camera,
    pub marker: DemoWorldCamera2d,
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
            marker: DemoWorldCamera2d,
            transform: Transform::from_xyz(0.0, 0.0, 1000.0),
        }
    }
}
