pub mod animation;
pub mod camera;
pub mod characters;
pub mod screens;
pub mod ui;

#[cfg(test)]
mod tests;

pub use error::Result;

use bevy::prelude::*;

use self::animation::Animation2dPlugin;
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
            Animation2dPlugin,
            CharacterRenderPlugin,
            UiPlugin,
        ));
    }
}
