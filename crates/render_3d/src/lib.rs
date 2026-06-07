pub mod camera;
pub mod scene;
pub mod ui;

pub use error::Result;

use bevy::prelude::*;

use self::camera::Camera3dPlugin;
use self::scene::Scene3dPlugin;
use self::ui::Ui3dPlugin;

pub struct Render3dPlugin;

impl Plugin for Render3dPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((Scene3dPlugin, Camera3dPlugin, Ui3dPlugin));
    }
}
