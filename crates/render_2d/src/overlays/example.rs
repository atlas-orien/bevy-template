use bevy::prelude::*;

#[derive(Component, Debug, Clone, Copy, Default, Eq, PartialEq)]
pub struct ExampleWorldOverlay2d;

#[derive(Component, Debug, Clone, Copy, PartialEq)]
pub struct ExampleHealthBarOverlay2d {
    pub width: f32,
    pub height: f32,
    pub y_offset: f32,
}

#[derive(Bundle)]
pub struct ExampleHealthBarOverlay2dBundle {
    pub marker: ExampleWorldOverlay2d,
    pub health_bar: ExampleHealthBarOverlay2d,
    pub sprite: Sprite,
    pub transform: Transform,
}

impl ExampleHealthBarOverlay2dBundle {
    pub fn new(width: f32, height: f32, y_offset: f32, color: Color) -> Self {
        Self {
            marker: ExampleWorldOverlay2d,
            health_bar: ExampleHealthBarOverlay2d {
                width,
                height,
                y_offset,
            },
            sprite: Sprite {
                color,
                custom_size: Some(Vec2::new(width, height)),
                ..default()
            },
            transform: Transform::from_xyz(0.0, y_offset, 0.0),
        }
    }
}
