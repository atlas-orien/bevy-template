use bevy::prelude::*;

use crate::primitives::meshes::{StaticMesh3d, StaticMesh3dBundle};

#[derive(Component, Debug, Clone, Copy, Default, Eq, PartialEq)]
pub struct SphereItem3dMarker;

pub struct SphereItem3d {
    mesh: Handle<Mesh>,
    material: Handle<StandardMaterial>,
    transform: Transform,
}

impl SphereItem3d {
    pub fn new(
        mesh: Handle<Mesh>,
        material: Handle<StandardMaterial>,
        transform: Transform,
    ) -> Self {
        Self {
            mesh,
            material,
            transform,
        }
    }

    pub fn into_bundle(self) -> SphereItem3dBundle {
        SphereItem3dBundle {
            marker: SphereItem3dMarker,
            visual: StaticMesh3d::new(self.mesh, self.material, self.transform).into_bundle(),
        }
    }
}

#[derive(Bundle)]
pub struct SphereItem3dBundle {
    marker: SphereItem3dMarker,
    visual: StaticMesh3dBundle,
}
