use bevy::prelude::*;

use crate::primitives::meshes::{StaticMesh3d, StaticMesh3dBundle};

pub struct DemoCube3d {
    mesh: Handle<Mesh>,
    material: Handle<StandardMaterial>,
}

impl DemoCube3d {
    pub fn new(mesh: Handle<Mesh>, material: Handle<StandardMaterial>) -> Self {
        Self { mesh, material }
    }

    pub fn into_bundle(self) -> DemoCube3dBundle {
        DemoCube3dBundle {
            visual: StaticMesh3d::new(
                self.mesh,
                self.material,
                Transform::from_xyz(-1.8, 0.6, 0.0).with_rotation(Quat::from_rotation_y(0.45)),
            )
            .into_bundle(),
        }
    }
}

#[derive(Bundle)]
pub struct DemoCube3dBundle {
    visual: StaticMesh3dBundle,
}
