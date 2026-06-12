pub mod demo_player_atlas;
pub mod example;

use bevy::prelude::*;

pub use demo_player_atlas::{DEMO_PLAYER_FRAME_SIZE, demo_player_atlas_layout};

pub struct AtlasesPlugin;

impl Plugin for AtlasesPlugin {
    fn build(&self, _app: &mut App) {}
}
