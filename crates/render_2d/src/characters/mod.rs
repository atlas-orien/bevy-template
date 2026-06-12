pub mod demo_player;
pub mod example;
mod plugin;

pub use demo_player::{
    DemoNpcSprite2d, DemoNpcSprite2dBundle, DemoPlayerSprite2d, DemoPlayerSprite2dBundle,
};
pub use example::{ExampleCharacter2d, ExampleCharacter2dBundle};
pub use plugin::CharacterRenderPlugin;
