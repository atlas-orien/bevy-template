pub mod animation;
pub mod background;
pub mod camera;
pub mod characters;
pub mod effects;
pub mod environment;
pub mod items;
pub mod particles;
pub mod props;
pub mod screens;
pub mod tilemap;
pub mod ui;

pub use error::Result;

use bevy::prelude::*;

use self::animation::Animation2dPlugin;
use self::background::BackgroundPlugin;
use self::camera::Camera2dPlugin;
use self::characters::CharacterRenderPlugin;
use self::effects::EffectsPlugin;
use self::environment::EnvironmentPlugin;
use self::items::ItemsPlugin;
use self::particles::ParticlesPlugin;
use self::props::PropsPlugin;
use self::screens::ScreensPlugin;
use self::tilemap::TilemapPlugin;
use self::ui::UiPlugin;

pub struct Render2dPlugin;

impl Plugin for Render2dPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            Camera2dPlugin,
            BackgroundPlugin,
            CharacterRenderPlugin,
            ItemsPlugin,
            PropsPlugin,
            TilemapPlugin,
            EnvironmentPlugin,
            EffectsPlugin,
            ParticlesPlugin,
            Animation2dPlugin,
            ScreensPlugin,
            UiPlugin,
        ));
    }
}
