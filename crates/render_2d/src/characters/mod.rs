pub mod example;

pub use example::{ExampleCharacter2d, ExampleCharacter2dBundle};

use bevy::prelude::*;

pub struct CharacterRenderPlugin;

impl Plugin for CharacterRenderPlugin {
    fn build(&self, _app: &mut App) {}
}
