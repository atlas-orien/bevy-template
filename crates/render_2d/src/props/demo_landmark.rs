//! Demo 地标 sprite 表现。

use bevy::prelude::*;

const DEMO_LANDMARK_SIZE: Vec2 = Vec2::new(28.0, 150.0);

#[derive(Component, Debug, Clone, Copy, Default, Eq, PartialEq)]
struct DemoLandmark2dMarker;

#[derive(Bundle)]
pub struct DemoLandmark2d {
    marker: DemoLandmark2dMarker,
    sprite: Sprite,
    transform: Transform,
}

impl DemoLandmark2d {
    pub fn new(translation: Vec3, color: Color) -> Self {
        Self {
            marker: DemoLandmark2dMarker,
            sprite: Sprite {
                color,
                custom_size: Some(DEMO_LANDMARK_SIZE),
                ..default()
            },
            transform: Transform::from_translation(translation),
        }
    }
}
