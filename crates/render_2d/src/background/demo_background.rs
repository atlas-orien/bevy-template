//! Demo 视差背景层 bundle 与视差偏移系统。

use bevy::prelude::*;

use crate::camera::DemoWorldCamera2d;

#[derive(Component, Debug, Clone, Copy, Default, Eq, PartialEq)]
pub struct DemoBackgroundLayer2d;

#[derive(Component, Debug, Clone, Copy, Default, PartialEq)]
pub struct DemoParallaxBackgroundLayer2d {
    pub speed: Vec2,
}

#[derive(Bundle)]
pub struct DemoBackgroundLayer2dBundle {
    pub marker: DemoBackgroundLayer2d,
    pub parallax: DemoParallaxBackgroundLayer2d,
    pub sprite: Sprite,
    pub transform: Transform,
}

impl DemoBackgroundLayer2dBundle {
    pub fn new(color: Color, size: Vec2, z: f32, parallax_speed: Vec2) -> Self {
        Self {
            marker: DemoBackgroundLayer2d,
            parallax: DemoParallaxBackgroundLayer2d {
                speed: parallax_speed,
            },
            sprite: Sprite {
                color,
                custom_size: Some(size),
                ..default()
            },
            transform: Transform::from_xyz(0.0, 0.0, z),
        }
    }
}

pub fn demo_parallax_background_system(
    camera: Query<&Transform, (With<DemoWorldCamera2d>, Without<DemoBackgroundLayer2d>)>,
    mut backgrounds: Query<
        (&DemoParallaxBackgroundLayer2d, &mut Transform),
        With<DemoBackgroundLayer2d>,
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
