pub mod character;

use bevy::prelude::*;

pub use character::{Character2dRender, Character2dRenderBundle, Character2dSprite};

pub struct CharacterRenderPlugin;

impl Plugin for CharacterRenderPlugin {
    fn build(&self, _app: &mut App) {}
}
