//! Demo 光感表现：用高亮 sprite 表达 glow。

use bevy::prelude::*;

const DEMO_GLOW_SIZE: Vec2 = Vec2::splat(96.0);
const DEMO_GLOW_Z: f32 = 2.0;
const DEMO_GLOW_COLOR: Color = Color::srgba(0.35, 0.78, 1.0, 0.34);

#[derive(Component, Debug, Clone, Copy, Default, Eq, PartialEq)]
pub(super) struct DemoGlow2dMarker;

#[derive(Bundle)]
pub struct DemoGlow2d {
    marker: DemoGlow2dMarker,
    sprite: Sprite,
    transform: Transform,
}

impl DemoGlow2d {
    pub fn new(translation: Vec3) -> Self {
        Self {
            marker: DemoGlow2dMarker,
            sprite: Sprite {
                color: DEMO_GLOW_COLOR,
                custom_size: Some(DEMO_GLOW_SIZE),
                ..default()
            },
            transform: Transform::from_xyz(translation.x, translation.y, DEMO_GLOW_Z),
        }
    }
}
