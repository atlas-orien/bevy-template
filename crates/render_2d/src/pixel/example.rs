use bevy::prelude::*;

#[derive(Component, Debug, Clone, Copy, Default, Eq, PartialEq)]
pub struct ExamplePixelArt2d;

#[derive(Component, Debug, Clone, Copy, PartialEq)]
pub struct ExamplePixelSnap2d {
    pub pixels_per_unit: f32,
}

#[derive(Bundle)]
pub struct ExamplePixelArtSprite2dBundle {
    pub marker: ExamplePixelArt2d,
    pub snap: ExamplePixelSnap2d,
    pub sprite: Sprite,
    pub transform: Transform,
}

impl ExamplePixelArtSprite2dBundle {
    pub fn new(color: Color, size: Vec2, translation: Vec3, pixels_per_unit: f32) -> Self {
        Self {
            marker: ExamplePixelArt2d,
            snap: ExamplePixelSnap2d { pixels_per_unit },
            sprite: Sprite {
                color,
                custom_size: Some(size),
                ..default()
            },
            transform: Transform::from_translation(translation),
        }
    }
}
