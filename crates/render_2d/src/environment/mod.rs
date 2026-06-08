use bevy::prelude::*;

#[derive(Component, Debug, Clone, Copy, Default)]
pub struct Background2d;

#[derive(Component, Debug, Clone, Copy, Default)]
pub struct Ground2d;

#[derive(Bundle)]
pub struct Background2dBundle {
    pub sprite: Sprite,
    pub transform: Transform,
    pub marker: Background2d,
}

impl Background2dBundle {
    pub fn new(color: Color, size: Vec2, z: f32) -> Self {
        Self {
            sprite: Sprite::from_color(color, size),
            transform: Transform::from_xyz(0.0, 0.0, z),
            marker: Background2d,
        }
    }
}

#[derive(Bundle)]
pub struct Ground2dBundle {
    pub sprite: Sprite,
    pub transform: Transform,
    pub marker: Ground2d,
}

impl Ground2dBundle {
    pub fn new(color: Color, size: Vec2, translation: Vec3) -> Self {
        Self {
            sprite: Sprite::from_color(color, size),
            transform: Transform::from_translation(translation),
            marker: Ground2d,
        }
    }
}
