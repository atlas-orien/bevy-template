use bevy::prelude::*;

#[derive(Component, Debug, Clone, Copy, Default, Eq, PartialEq)]
pub struct StaticMesh3dMarker;

pub struct StaticMesh3d {
    mesh: Handle<Mesh>,
    material: Handle<StandardMaterial>,
    transform: Transform,
}

impl StaticMesh3d {
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

    pub fn at(mesh: Handle<Mesh>, material: Handle<StandardMaterial>, translation: Vec3) -> Self {
        Self::new(mesh, material, Transform::from_translation(translation))
    }

    pub fn into_bundle(self) -> StaticMesh3dBundle {
        StaticMesh3dBundle {
            marker: StaticMesh3dMarker,
            mesh: Mesh3d(self.mesh),
            material: MeshMaterial3d(self.material),
            transform: self.transform,
            visibility: Visibility::default(),
        }
    }
}

#[derive(Bundle)]
pub struct StaticMesh3dBundle {
    marker: StaticMesh3dMarker,
    mesh: Mesh3d,
    material: MeshMaterial3d<StandardMaterial>,
    transform: Transform,
    visibility: Visibility,
}
