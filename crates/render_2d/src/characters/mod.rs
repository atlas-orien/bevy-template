pub mod demo_player;
mod plugin;

pub use demo_player::{
    DemoNpcSprite2d, DemoNpcSprite2dBundle, DemoPlayerSprite2d, DemoPlayerSprite2dBundle,
    DemoPlayerSpriteAtlasReady2d, prepare_demo_player_sprite_atlas_system,
};
pub use plugin::CharacterRenderPlugin;
