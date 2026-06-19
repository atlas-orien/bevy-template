//! 通用 3D 表现 primitive。

pub mod camera;
pub mod lights;
pub mod materials;
pub mod meshes;
pub mod transforms;

pub use camera::Camera3dContentPlugin;
pub use lights::Lights3dPlugin;
pub use meshes::Meshes3dPlugin;

use bevy::prelude::*;

pub struct Render3dPrimitivesPlugin;

impl Plugin for Render3dPrimitivesPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((Camera3dContentPlugin, Lights3dPlugin, Meshes3dPlugin));
    }
}
