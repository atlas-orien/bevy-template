use bevy::prelude::*;

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
            mesh: Mesh3d(self.mesh),
            material: MeshMaterial3d(self.material),
            transform: Transform::from_translation(Vec3::new(0.0, 0.72, 0.0)),
            visibility: Visibility::default(),
        }
    }
}

#[derive(Bundle)]
pub struct DemoSphere3dBundle {
    mesh: Mesh3d,
    material: MeshMaterial3d<StandardMaterial>,
    transform: Transform,
    visibility: Visibility,
}
