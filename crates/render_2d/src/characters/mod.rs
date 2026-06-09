use bevy::prelude::*;

#[derive(Component, Debug, Clone, Copy, Default)]
pub struct Character2dRender;

#[derive(Bundle)]
pub struct Character2dRenderBundle {
    pub marker: Character2dRender,
    pub sprite: Sprite,
}

impl Character2dRenderBundle {
    pub fn new(color: Color, size: Vec2) -> Self {
        Self {
            marker: Character2dRender,
            sprite: Sprite {
                color,
                custom_size: Some(size),
                ..default()
            },
        }
    }
}

pub struct CharacterRenderPlugin;

impl Plugin for CharacterRenderPlugin {
    fn build(&self, _app: &mut App) {}
}
