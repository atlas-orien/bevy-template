pub mod character;

use bevy::prelude::*;

pub use character::{Character, DemoPlayerControlled};

pub struct CharactersPlugin;

impl Plugin for CharactersPlugin {
    fn build(&self, _app: &mut App) {}
}
