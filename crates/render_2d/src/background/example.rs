use bevy::prelude::*;

#[derive(Component, Debug, Clone, Copy, Default, Eq, PartialEq)]
pub struct ExampleBackground2d;

#[derive(Component, Debug, Clone, Copy, Default, PartialEq)]
pub struct ExampleParallaxBackgroundLayer2d {
    pub speed: Vec2,
}

#[derive(Bundle)]
pub struct ExampleBackground2dBundle {
    pub marker: ExampleBackground2d,
    pub parallax: ExampleParallaxBackgroundLayer2d,
    pub sprite: Sprite,
    pub transform: Transform,
}

impl ExampleBackground2dBundle {
    pub fn new(color: Color, size: Vec2, z: f32, parallax_speed: Vec2) -> Self {
        Self {
            marker: ExampleBackground2d,
            parallax: ExampleParallaxBackgroundLayer2d {
                speed: parallax_speed,
            },
            sprite: Sprite {
                color,
                custom_size: Some(size),
                ..default()
            },
            transform: Transform::from_xyz(0.0, 0.0, z),
        }
    }
}
