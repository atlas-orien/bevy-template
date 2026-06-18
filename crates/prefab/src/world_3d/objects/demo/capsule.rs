use bevy::prelude::*;
use render_3d::primitives::meshes::{StaticMesh3d, StaticMesh3dBundle};

use crate::Prefab;

#[derive(Component, Debug, Clone, Copy, Default, Eq, PartialEq)]
pub struct DemoPreviewCapsule3dMarker;

pub struct DemoPreviewCapsule3dPrefab {
    mesh: Handle<Mesh>,
    material: Handle<StandardMaterial>,
}

impl DemoPreviewCapsule3dPrefab {
    pub fn new(mesh: Handle<Mesh>, material: Handle<StandardMaterial>) -> Self {
        Self { mesh, material }
    }
}

#[derive(Bundle)]
struct DemoPreviewCapsule3dBundle {
    marker: DemoPreviewCapsule3dMarker,
    visual: StaticMesh3dBundle,
}

impl Prefab for DemoPreviewCapsule3dPrefab {
    fn spawn(self, commands: &mut Commands) -> Entity {
        commands
            .spawn(DemoPreviewCapsule3dBundle {
                marker: DemoPreviewCapsule3dMarker,
                visual: StaticMesh3d::new(
                    self.mesh,
                    self.material,
                    Transform::from_xyz(1.85, 0.98, 0.0).with_rotation(Quat::from_rotation_z(0.16)),
                )
                .into_bundle(),
            })
            .id()
    }
}
