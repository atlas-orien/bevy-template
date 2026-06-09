use bevy::prelude::*;

use crate::geometry::{RenderColor2d, RenderSize2d};

#[derive(Component, Debug, Clone, Copy, Default)]
pub struct Character2dRender;

#[derive(Component, Debug, Clone, Copy)]
pub struct Character2dSprite {
    pub color: RenderColor2d,
    pub size: RenderSize2d,
}

#[derive(Bundle)]
pub struct Character2dRenderBundle {
    pub marker: Character2dRender,
    pub sprite_config: Character2dSprite,
    pub sprite: Sprite,
}

impl Character2dRenderBundle {
    pub fn new(color: Color, size: Vec2) -> Self {
        Self {
            marker: Character2dRender,
            sprite_config: Character2dSprite {
                color: RenderColor2d(color),
                size: RenderSize2d(size),
            },
            sprite: Sprite {
                color,
                custom_size: Some(size),
                ..default()
            },
        }
    }
}
