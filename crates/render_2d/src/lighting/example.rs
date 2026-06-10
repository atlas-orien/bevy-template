use bevy::prelude::*;

#[derive(Component, Debug, Clone, Copy, Default, Eq, PartialEq)]
pub struct ExampleGlowLayer2d;

#[derive(Component, Debug, Clone, Copy, PartialEq)]
pub struct ExampleFakeShadow2d {
    pub offset: Vec2,
    pub opacity: f32,
}

#[derive(Bundle)]
pub struct ExampleFakeShadow2dBundle {
    pub marker: ExampleGlowLayer2d,
    pub shadow: ExampleFakeShadow2d,
    pub sprite: Sprite,
    pub transform: Transform,
}

impl ExampleFakeShadow2dBundle {
    pub fn new(color: Color, size: Vec2, translation: Vec3, offset: Vec2, opacity: f32) -> Self {
        Self {
            marker: ExampleGlowLayer2d,
            shadow: ExampleFakeShadow2d { offset, opacity },
            sprite: Sprite {
                color,
                custom_size: Some(size),
                ..default()
            },
            transform: Transform::from_translation(translation),
        }
    }
}
