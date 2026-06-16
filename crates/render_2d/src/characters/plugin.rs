use bevy::prelude::*;

use super::demo_player::prepare_demo_player_sprite_atlas_system;

pub struct CharacterRenderPlugin;

impl Plugin for CharacterRenderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, prepare_demo_player_sprite_atlas_system);
    }
}
