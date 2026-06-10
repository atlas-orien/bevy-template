use bevy::prelude::*;

#[derive(Component, Debug, Clone, Copy, Default, Eq, PartialEq)]
pub struct ExampleCharacter2d;

#[derive(Bundle)]
pub struct ExampleCharacter2dBundle {
    pub marker: ExampleCharacter2d,
    pub sprite: Sprite,
}

impl ExampleCharacter2dBundle {
    pub fn new(color: Color, size: Vec2) -> Self {
        Self {
            marker: ExampleCharacter2d,
            sprite: Sprite {
                color,
                custom_size: Some(size),
                ..default()
            },
        }
    }
}
