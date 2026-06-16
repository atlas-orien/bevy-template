//! Demo 2D material 表现：可组合到 Mesh2d 实体上的 ColorMaterial。

use bevy::prelude::*;

#[derive(Bundle)]
pub struct DemoColorMaterial2d {
    material: MeshMaterial2d<ColorMaterial>,
}

impl DemoColorMaterial2d {
    pub fn new(material: Handle<ColorMaterial>) -> Self {
        Self {
            material: MeshMaterial2d(material),
        }
    }
}
