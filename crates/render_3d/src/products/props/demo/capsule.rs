use bevy::prelude::*;

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
            mesh: Mesh3d(self.mesh),
            material: MeshMaterial3d(self.material),
            transform: Transform::from_xyz(1.85, 0.98, 0.0)
                .with_rotation(Quat::from_rotation_z(0.16)),
            visibility: Visibility::default(),
        }
    }
}

#[derive(Bundle)]
pub struct DemoCapsule3dBundle {
    mesh: Mesh3d,
    material: MeshMaterial3d<StandardMaterial>,
    transform: Transform,
    visibility: Visibility,
}
