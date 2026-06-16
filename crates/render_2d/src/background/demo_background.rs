//! Demo 视差背景层 bundle 与视差偏移系统。

use bevy::prelude::*;

use crate::camera::DemoWorldCamera2dMarker;

#[derive(Component, Debug, Clone, Copy, Default, Eq, PartialEq)]
pub(super) struct DemoBackgroundLayer2dMarker;

#[derive(Component, Debug, Clone, Copy, Default, PartialEq)]
pub(super) struct DemoParallaxBackgroundLayer2d {
    speed: Vec2,
}

pub struct DemoBackgroundLayer2d {
    color: Color,
    size: Vec2,
    z: f32,
    parallax_speed: Vec2,
}

impl DemoBackgroundLayer2d {
    pub fn new(color: Color, size: Vec2, z: f32, parallax_speed: Vec2) -> Self {
        Self {
            color,
            size,
            z,
            parallax_speed,
        }
    }

    pub fn into_bundle(self) -> impl Bundle {
        (
            DemoBackgroundLayer2dMarker,
            DemoParallaxBackgroundLayer2d {
                speed: self.parallax_speed,
            },
            Sprite {
                color: self.color,
                custom_size: Some(self.size),
                ..default()
            },
            Transform::from_xyz(0.0, 0.0, self.z),
        )
    }
}

pub(super) fn demo_parallax_background_system(
    camera: Query<
        &Transform,
        (
            With<DemoWorldCamera2dMarker>,
            Without<DemoBackgroundLayer2dMarker>,
        ),
    >,
    mut backgrounds: Query<
        (&DemoParallaxBackgroundLayer2d, &mut Transform),
        With<DemoBackgroundLayer2dMarker>,
    >,
) {
    let Ok(camera) = camera.single() else {
        return;
    };
    let camera_translation = camera.translation.truncate();

    for (parallax, mut transform) in &mut backgrounds {
        let offset = camera_translation * parallax.speed;
        transform.translation.x = offset.x;
        transform.translation.y = offset.y;
    }
}
