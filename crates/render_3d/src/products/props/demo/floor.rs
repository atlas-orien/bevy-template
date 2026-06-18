use bevy::prelude::*;

use crate::primitives::meshes::{StaticMesh3d, StaticMesh3dBundle};

pub struct DemoFloor3d {
    mesh: Handle<Mesh>,
    material: Handle<StandardMaterial>,
}

impl DemoFloor3d {
    pub fn new(mesh: Handle<Mesh>, material: Handle<StandardMaterial>) -> Self {
        Self { mesh, material }
    }

    pub fn into_bundle(self) -> DemoFloor3dBundle {
        DemoFloor3dBundle {
            visual: StaticMesh3d::at(self.mesh, self.material, Vec3::ZERO).into_bundle(),
        }
    }
}

#[derive(Bundle)]
pub struct DemoFloor3dBundle {
    visual: StaticMesh3dBundle,
}
