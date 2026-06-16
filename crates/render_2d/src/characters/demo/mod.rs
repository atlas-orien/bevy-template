//! Demo 角色 2D 表现。

mod npc_sprite;
mod player_sprite;

pub use npc_sprite::DemoNpcSprite2d;
pub use player_sprite::DemoPlayerSprite2d;

pub(super) use player_sprite::prepare_demo_player_sprite_atlas_system;
