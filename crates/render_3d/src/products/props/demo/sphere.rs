use bevy::prelude::*;

use crate::primitives::meshes::{StaticMesh3d, StaticMesh3dBundle};

pub struct DemoSphere3d {
    mesh: Handle<Mesh>,
    material: Handle<StandardMaterial>,
}

impl DemoSphere3d {
    pub fn new(mesh: Handle<Mesh>, material: Handle<StandardMaterial>) -> Self {
        Self { mesh, material }
    }

    pub fn into_bundle(self) -> DemoSphere3dBundle {
        DemoSphere3dBundle {
            visual: StaticMesh3d::at(self.mesh, self.material, Vec3::new(0.0, 0.72, 0.0))
                .into_bundle(),
        }
    }
}

#[derive(Bundle)]
pub struct DemoSphere3dBundle {
    visual: StaticMesh3dBundle,
}
