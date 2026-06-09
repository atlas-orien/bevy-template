pub mod appearance;
pub mod camera;
pub mod characters;
pub mod geometry;
pub mod ordering;
pub mod screens;
pub mod sprite;
pub mod transform;
pub mod ui;

pub use error::Result;

use bevy::prelude::*;

use self::camera::Camera2dPlugin;
use self::characters::CharacterRenderPlugin;
use self::screens::ScreensPlugin;
use self::ui::UiPlugin;

pub struct Render2dPlugin;

impl Plugin for Render2dPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            ScreensPlugin,
            Camera2dPlugin,
            CharacterRenderPlugin,
            UiPlugin,
        ));
    }
}
