use bevy::prelude::*;

#[derive(Component, Debug, Clone, Copy, Default, Eq, PartialEq)]
pub struct ExampleProp2d;

#[derive(Bundle)]
pub struct ExampleProp2dBundle {
    pub marker: ExampleProp2d,
    pub sprite: Sprite,
    pub transform: Transform,
}

impl ExampleProp2dBundle {
    pub fn new(color: Color, size: Vec2, translation: Vec3) -> Self {
        Self {
            marker: ExampleProp2d,
            sprite: Sprite {
                color,
                custom_size: Some(size),
                ..default()
            },
            transform: Transform::from_translation(translation),
        }
    }
}
