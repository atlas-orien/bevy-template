pub mod player;

use bevy::prelude::*;

use self::player::PlayerSpritePlugin;

pub struct CharacterRenderPlugin;

impl Plugin for CharacterRenderPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PlayerSpritePlugin);
    }
}
