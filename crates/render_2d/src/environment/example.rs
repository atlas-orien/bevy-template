use bevy::prelude::*;

#[derive(Component, Debug, Clone, Copy, Default, Eq, PartialEq)]
pub struct ExampleEnvironmentDecoration2d;

#[derive(Component, Debug, Clone, Copy, Default, PartialEq)]
pub struct ExampleAmbientMotion2d {
    pub drift: Vec2,
}

#[derive(Bundle)]
pub struct ExampleEnvironmentDecoration2dBundle {
    pub marker: ExampleEnvironmentDecoration2d,
    pub ambient_motion: ExampleAmbientMotion2d,
    pub sprite: Sprite,
    pub transform: Transform,
}

impl ExampleEnvironmentDecoration2dBundle {
    pub fn new(color: Color, size: Vec2, translation: Vec3, drift: Vec2) -> Self {
        Self {
            marker: ExampleEnvironmentDecoration2d,
            ambient_motion: ExampleAmbientMotion2d { drift },
            sprite: Sprite {
                color,
                custom_size: Some(size),
                ..default()
            },
            transform: Transform::from_translation(translation),
        }
    }
}
