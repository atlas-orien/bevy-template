use bevy::prelude::*;

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
            mesh: Mesh3d(self.mesh),
            material: MeshMaterial3d(self.material),
            transform: Transform::from_xyz(-1.8, 0.6, 0.0)
                .with_rotation(Quat::from_rotation_y(0.45)),
            visibility: Visibility::default(),
        }
    }
}

#[derive(Bundle)]
pub struct DemoCube3dBundle {
    mesh: Mesh3d,
    material: MeshMaterial3d<StandardMaterial>,
    transform: Transform,
    visibility: Visibility,
}
