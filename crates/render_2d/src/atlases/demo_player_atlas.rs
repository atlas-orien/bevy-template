//! Demo 角色 sprite sheet 的 atlas layout 配置。

use bevy::prelude::*;

pub const DEMO_PLAYER_FRAME_SIZE: UVec2 = UVec2::new(24, 24);

pub fn demo_player_atlas_layout() -> TextureAtlasLayout {
    TextureAtlasLayout::from_grid(DEMO_PLAYER_FRAME_SIZE, 7, 1, None, None)
}
