//! User-editable 3D presentation content, cameras, scenes, and render systems.

pub mod animation;
pub mod camera;
pub mod characters;
pub mod debug;
pub mod effects;
pub mod environment;
pub mod items;
pub mod lighting;
pub mod materials;
pub mod models;
pub mod overlays;
pub mod particles;
pub mod props;
pub mod scenes;

pub use error::Result;

use bevy::prelude::*;

use self::animation::Animation3dPlugin;
use self::camera::Camera3dPlugin;
use self::characters::Characters3dPlugin;
use self::debug::Debug3dPlugin;
use self::effects::Effects3dPlugin;
use self::environment::Environment3dPlugin;
use self::items::Items3dPlugin;
use self::lighting::Lighting3dPlugin;
use self::materials::Materials3dPlugin;
use self::models::Models3dPlugin;
use self::overlays::Overlays3dPlugin;
use self::particles::Particles3dPlugin;
use self::props::Props3dPlugin;
use self::scenes::Scenes3dPlugin;

pub struct Render3dPlugin;

impl Plugin for Render3dPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(Camera3dPlugin)
            .add_plugins(Models3dPlugin)
            .add_plugins(Materials3dPlugin)
            .add_plugins(Animation3dPlugin)
            .add_plugins(Lighting3dPlugin)
            .add_plugins(Environment3dPlugin)
            .add_plugins(Scenes3dPlugin)
            .add_plugins(Characters3dPlugin)
            .add_plugins(Items3dPlugin)
            .add_plugins(Props3dPlugin)
            .add_plugins(Effects3dPlugin)
            .add_plugins(Particles3dPlugin)
            .add_plugins(Overlays3dPlugin)
            .add_plugins(Debug3dPlugin);
    }
}
