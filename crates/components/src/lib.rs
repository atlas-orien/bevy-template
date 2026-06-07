pub mod base;
pub mod characters;
pub mod events;
pub mod items;
pub mod resources;
pub mod ui;
pub mod world;

pub use error::Result;

use bevy::prelude::*;

use self::characters::CharactersPlugin;
use self::items::ItemsPlugin;
use self::world::WorldPlugin;

pub struct ComponentsPlugin;

impl Plugin for ComponentsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((WorldPlugin, CharactersPlugin, ItemsPlugin));
    }
}
