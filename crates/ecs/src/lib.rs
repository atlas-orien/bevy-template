pub mod background;
pub mod characters;
pub mod items;
pub mod world;

pub use error::Result;

use bevy::prelude::*;

use self::background::BackgroundPlugin;
use self::characters::CharactersPlugin;
use self::items::ItemsPlugin;
use self::world::WorldPlugin;

pub struct EcsPlugin;

impl Plugin for EcsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((WorldPlugin, BackgroundPlugin, CharactersPlugin, ItemsPlugin));
    }
}
