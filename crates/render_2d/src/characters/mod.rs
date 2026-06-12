pub mod demo_player;
pub mod example;

pub use demo_player::{
    DemoNpcSprite2d, DemoNpcSprite2dBundle, DemoPlayerSprite2d, DemoPlayerSprite2dBundle,
};
pub use example::{ExampleCharacter2d, ExampleCharacter2dBundle};

use bevy::prelude::*;

pub struct CharacterRenderPlugin;

impl Plugin for CharacterRenderPlugin {
    fn build(&self, _app: &mut App) {}
}
