use bevy::prelude::*;

#[derive(Component, Debug, Clone, Copy, Default, Eq, PartialEq)]
pub struct ExampleEffect2d;

#[derive(Component, Debug, Clone, Copy, Default, PartialEq)]
pub struct ExampleEffectLifetime2d {
    pub seconds_remaining: f32,
}

#[derive(Bundle)]
pub struct ExampleEffect2dBundle {
    pub marker: ExampleEffect2d,
    pub lifetime: ExampleEffectLifetime2d,
    pub sprite: Sprite,
    pub transform: Transform,
}

impl ExampleEffect2dBundle {
    pub fn new(color: Color, size: Vec2, translation: Vec3, seconds_remaining: f32) -> Self {
        Self {
            marker: ExampleEffect2d,
            lifetime: ExampleEffectLifetime2d { seconds_remaining },
            sprite: Sprite {
                color,
                custom_size: Some(size),
                ..default()
            },
            transform: Transform::from_translation(translation),
        }
    }
}
