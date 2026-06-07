pub mod player;

use bevy::prelude::*;

use self::player::PlayerPlugin;

pub struct CharactersPlugin;

impl Plugin for CharactersPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PlayerPlugin);
    }
}
