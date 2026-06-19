use bevy::prelude::*;

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
            mesh: Mesh3d(self.mesh),
            material: MeshMaterial3d(self.material),
            transform: Transform::from_translation(Vec3::ZERO),
            visibility: Visibility::default(),
        }
    }
}

#[derive(Bundle)]
pub struct DemoFloor3dBundle {
    mesh: Mesh3d,
    material: MeshMaterial3d<StandardMaterial>,
    transform: Transform,
    visibility: Visibility,
}
