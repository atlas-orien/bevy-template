use bevy::prelude::*;

use crate::primitives::meshes::{StaticMesh3d, StaticMesh3dBundle};

pub struct DemoCapsule3d {
    mesh: Handle<Mesh>,
    material: Handle<StandardMaterial>,
}

impl DemoCapsule3d {
    pub fn new(mesh: Handle<Mesh>, material: Handle<StandardMaterial>) -> Self {
        Self { mesh, material }
    }

    pub fn into_bundle(self) -> DemoCapsule3dBundle {
        DemoCapsule3dBundle {
            visual: StaticMesh3d::new(
                self.mesh,
                self.material,
                Transform::from_xyz(1.85, 0.98, 0.0).with_rotation(Quat::from_rotation_z(0.16)),
            )
            .into_bundle(),
        }
    }
}

#[derive(Bundle)]
pub struct DemoCapsule3dBundle {
    visual: StaticMesh3dBundle,
}
