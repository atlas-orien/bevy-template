use bevy::prelude::*;
use render_3d::primitives::meshes::{StaticMesh3d, StaticMesh3dBundle};

use crate::Prefab;

#[derive(Component, Debug, Clone, Copy, Default, Eq, PartialEq)]
pub struct DemoPreviewCube3dMarker;

pub struct DemoPreviewCube3dPrefab {
    mesh: Handle<Mesh>,
    material: Handle<StandardMaterial>,
}

impl DemoPreviewCube3dPrefab {
    pub fn new(mesh: Handle<Mesh>, material: Handle<StandardMaterial>) -> Self {
        Self { mesh, material }
    }
}

#[derive(Bundle)]
struct DemoPreviewCube3dBundle {
    marker: DemoPreviewCube3dMarker,
    visual: StaticMesh3dBundle,
}

impl Prefab for DemoPreviewCube3dPrefab {
    fn spawn(self, commands: &mut Commands) -> Entity {
        commands
            .spawn(DemoPreviewCube3dBundle {
                marker: DemoPreviewCube3dMarker,
                visual: StaticMesh3d::new(
                    self.mesh,
                    self.material,
                    Transform::from_xyz(-1.8, 0.6, 0.0).with_rotation(Quat::from_rotation_y(0.45)),
                )
                .into_bundle(),
            })
            .id()
    }
}
