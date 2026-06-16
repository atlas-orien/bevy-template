//! 通用 texture atlas sprite primitive。

mod plugin;

use bevy::prelude::*;

use crate::primitives::markers::AtlasSprite2dMarker;

pub use plugin::AtlasesPlugin;

#[derive(Bundle)]
pub struct AtlasSprite2d {
    marker: AtlasSprite2dMarker,
    sprite: Sprite,
    transform: Transform,
    visibility: Visibility,
}

impl AtlasSprite2d {
    pub fn new(
        image: Handle<Image>,
        layout: Handle<TextureAtlasLayout>,
        index: usize,
        size: Option<Vec2>,
        translation: Vec3,
    ) -> Self {
        Self {
            marker: AtlasSprite2dMarker,
            sprite: Sprite {
                image,
                texture_atlas: Some(TextureAtlas { layout, index }),
                custom_size: size,
                ..default()
            },
            transform: Transform::from_translation(translation),
            visibility: Visibility::default(),
        }
    }
}
