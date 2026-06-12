use bevy::prelude::*;

use super::characters::CharactersPlugin;
use super::items::ItemsPlugin;
use super::world::WorldPlugin;

pub struct ComponentsPlugin;

impl Plugin for ComponentsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((WorldPlugin, CharactersPlugin, ItemsPlugin));
    }
}
