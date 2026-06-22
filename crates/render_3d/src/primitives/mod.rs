//! 通用 3D 表现 primitive。

pub mod camera;
pub mod lights;
pub mod materials;
pub mod meshes;
pub mod models;
pub mod transforms;

pub use camera::Camera3dContentPlugin;

use bevy::prelude::*;

pub struct Render3dPrimitivesPlugin;

impl Plugin for Render3dPrimitivesPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(Camera3dContentPlugin);
    }
}
