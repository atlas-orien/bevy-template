use bevy::prelude::*;

#[derive(Component, Debug, Clone, Copy, Default, Eq, PartialEq)]
pub struct DemoPlayerSprite2d;

#[derive(Bundle)]
pub struct DemoPlayerSprite2dBundle {
    pub marker: DemoPlayerSprite2d,
    pub sprite: Sprite,
    pub transform: Transform,
}

impl Default for DemoPlayerSprite2dBundle {
    fn default() -> Self {
        Self {
            marker: DemoPlayerSprite2d,
            sprite: Sprite {
                color: Color::srgb(0.95, 0.72, 0.24),
                custom_size: Some(Vec2::new(28.0, 36.0)),
                ..default()
            },
            transform: Transform::from_xyz(0.0, 18.0, 4.0),
        }
    }
}
