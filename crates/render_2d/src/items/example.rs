use bevy::prelude::*;

#[derive(Component, Debug, Clone, Copy, Default, Eq, PartialEq)]
pub struct ExampleItem2d;

#[derive(Component, Debug, Clone, Copy, Default, Eq, PartialEq)]
pub struct ExamplePickupItem2d;

#[derive(Bundle)]
pub struct ExampleItem2dBundle {
    pub marker: ExampleItem2d,
    pub pickup_marker: ExamplePickupItem2d,
    pub sprite: Sprite,
    pub transform: Transform,
}

impl ExampleItem2dBundle {
    pub fn pickup(color: Color, size: Vec2, translation: Vec3) -> Self {
        Self {
            marker: ExampleItem2d,
            pickup_marker: ExamplePickupItem2d,
            sprite: Sprite {
                color,
                custom_size: Some(size),
                ..default()
            },
            transform: Transform::from_translation(translation),
        }
    }
}
