//! Demo NPC sprite 表现。

use bevy::prelude::*;

const DEMO_NPC_SPRITE_COLOR: Color = Color::srgb(0.65, 0.42, 0.95);
const DEMO_NPC_SPRITE_SIZE: Vec2 = Vec2::new(30.0, 38.0);
const DEMO_NPC_SPRITE_TRANSLATION: Vec3 = Vec3::new(0.0, 18.0, 4.0);

#[derive(Component, Debug, Clone, Copy, Default, Eq, PartialEq)]
struct DemoNpcSprite2dMarker;

#[derive(Bundle)]
pub struct DemoNpcSprite2d {
    marker: DemoNpcSprite2dMarker,
    sprite: Sprite,
    transform: Transform,
}

impl Default for DemoNpcSprite2d {
    fn default() -> Self {
        Self {
            marker: DemoNpcSprite2dMarker,
            sprite: Sprite {
                color: DEMO_NPC_SPRITE_COLOR,
                custom_size: Some(DEMO_NPC_SPRITE_SIZE),
                ..default()
            },
            transform: Transform::from_translation(DEMO_NPC_SPRITE_TRANSLATION),
        }
    }
}
