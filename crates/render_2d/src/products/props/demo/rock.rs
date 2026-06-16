//! Demo 岩石 sprite 表现。

use bevy::prelude::*;

const DEMO_ROCK_COLOR: Color = Color::srgb(0.45, 0.48, 0.52);
const DEMO_ROCK_SIZE: Vec2 = Vec2::new(44.0, 30.0);

#[derive(Component, Debug, Clone, Copy, Default, Eq, PartialEq)]
struct DemoRock2dMarker;

#[derive(Bundle)]
pub struct DemoRock2d {
    marker: DemoRock2dMarker,
    sprite: Sprite,
    transform: Transform,
}

impl DemoRock2d {
    pub fn new(translation: Vec3) -> Self {
        Self {
            marker: DemoRock2dMarker,
            sprite: Sprite {
                color: DEMO_ROCK_COLOR,
                custom_size: Some(DEMO_ROCK_SIZE),
                ..default()
            },
            transform: Transform::from_translation(translation),
        }
    }
}
