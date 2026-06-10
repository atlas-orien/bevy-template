use bevy::{prelude::*, sprite::Text2d};

#[derive(Component, Debug, Clone, Copy, Default, Eq, PartialEq)]
pub struct ExampleWorldText2d;

#[derive(Component, Debug, Clone, Copy, Default, PartialEq)]
pub struct ExampleFloatingText2d {
    pub velocity: Vec2,
    pub seconds_remaining: f32,
}

#[derive(Bundle)]
pub struct ExampleFloatingText2dBundle {
    pub marker: ExampleWorldText2d,
    pub floating: ExampleFloatingText2d,
    pub text: Text2d,
    pub transform: Transform,
}

impl ExampleFloatingText2dBundle {
    pub fn new(
        text: impl Into<String>,
        translation: Vec3,
        velocity: Vec2,
        seconds_remaining: f32,
    ) -> Self {
        Self {
            marker: ExampleWorldText2d,
            floating: ExampleFloatingText2d {
                velocity,
                seconds_remaining,
            },
            text: Text2d::new(text),
            transform: Transform::from_translation(translation),
        }
    }
}
