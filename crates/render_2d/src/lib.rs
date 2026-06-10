pub mod animation;
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

use self::animation::Animation2dPlugin;
use self::appearance::Appearance2dPlugin;
use self::camera::Camera2dPlugin;
use self::characters::CharacterRenderPlugin;
use self::geometry::Geometry2dPlugin;
use self::ordering::Ordering2dPlugin;
use self::screens::ScreensPlugin;
use self::sprite::Sprite2dPlugin;
use self::ui::UiPlugin;

pub struct Render2dPlugin;

impl Plugin for Render2dPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            ScreensPlugin,
            Camera2dPlugin,
            Appearance2dPlugin,
            Geometry2dPlugin,
            Ordering2dPlugin,
            Sprite2dPlugin,
            Animation2dPlugin,
            CharacterRenderPlugin,
            UiPlugin,
        ));
    }
}
