use bevy::prelude::*;

use crate::animation::frame::{DemoFrameAnimation2d, DemoPlayerAnimation2d};

const DEMO_PLAYER_SPRITE_SIZE: Vec2 = Vec2::new(48.0, 48.0);
const DEMO_PLAYER_SPRITE_TRANSLATION: Vec3 = Vec3::new(0.0, 18.0, 4.0);
const DEMO_NPC_SPRITE_COLOR: Color = Color::srgb(0.65, 0.42, 0.95);
const DEMO_NPC_SPRITE_SIZE: Vec2 = Vec2::new(30.0, 38.0);
const DEMO_NPC_SPRITE_TRANSLATION: Vec3 = Vec3::new(0.0, 18.0, 4.0);

#[derive(Component, Debug, Clone, Copy, Default, Eq, PartialEq)]
pub struct DemoPlayerSprite2d;

#[derive(Component, Debug, Clone, Copy, Default, Eq, PartialEq)]
pub struct DemoNpcSprite2d;

#[derive(Bundle)]
pub struct DemoPlayerSprite2dBundle {
    pub marker: DemoPlayerSprite2d,
    pub animation_marker: DemoPlayerAnimation2d,
    pub animation: DemoFrameAnimation2d,
    pub sprite: Sprite,
    pub transform: Transform,
}

impl DemoPlayerSprite2dBundle {
    pub fn new(image: Handle<Image>, atlas_layout: Handle<TextureAtlasLayout>) -> Self {
        Self {
            marker: DemoPlayerSprite2d,
            animation_marker: DemoPlayerAnimation2d,
            animation: DemoFrameAnimation2d::idle(),
            sprite: Sprite {
                image,
                texture_atlas: Some(TextureAtlas {
                    layout: atlas_layout,
                    index: 0,
                }),
                custom_size: Some(DEMO_PLAYER_SPRITE_SIZE),
                ..default()
            },
            transform: Transform::from_translation(DEMO_PLAYER_SPRITE_TRANSLATION),
        }
    }
}

#[derive(Bundle)]
pub struct DemoNpcSprite2dBundle {
    pub marker: DemoNpcSprite2d,
    pub sprite: Sprite,
    pub transform: Transform,
}

impl Default for DemoNpcSprite2dBundle {
    fn default() -> Self {
        Self {
            marker: DemoNpcSprite2d,
            sprite: Sprite {
                color: DEMO_NPC_SPRITE_COLOR,
                custom_size: Some(DEMO_NPC_SPRITE_SIZE),
                ..default()
            },
            transform: Transform::from_translation(DEMO_NPC_SPRITE_TRANSLATION),
        }
    }
}
