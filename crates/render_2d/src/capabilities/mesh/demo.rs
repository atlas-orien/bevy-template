//! Demo 2D mesh 表现：可组合 ColorMaterial 的 Mesh2d 实体。

use bevy::prelude::*;

#[derive(Bundle)]
pub struct DemoMesh2d {
    mesh: Mesh2d,
    transform: Transform,
}

impl DemoMesh2d {
    pub fn new(mesh: Handle<Mesh>, translation: Vec3) -> Self {
        Self {
            mesh: Mesh2d(mesh),
            transform: Transform::from_translation(translation),
        }
    }
}
