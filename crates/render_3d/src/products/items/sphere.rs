use bevy::prelude::*;

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
            mesh: Mesh3d(self.mesh),
            material: MeshMaterial3d(self.material),
            transform: self.transform,
            visibility: Visibility::default(),
        }
    }
}

#[derive(Bundle)]
pub struct SphereItem3dBundle {
    marker: SphereItem3dMarker,
    mesh: Mesh3d,
    material: MeshMaterial3d<StandardMaterial>,
    transform: Transform,
    visibility: Visibility,
}
