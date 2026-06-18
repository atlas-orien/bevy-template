use bevy::prelude::*;
use render_3d::primitives::meshes::{StaticMesh3d, StaticMesh3dBundle};

use crate::Prefab;

#[derive(Component, Debug, Clone, Copy, Default, Eq, PartialEq)]
pub struct DemoPreviewSphere3dMarker;

pub struct DemoPreviewSphere3dPrefab {
    mesh: Handle<Mesh>,
    material: Handle<StandardMaterial>,
}

impl DemoPreviewSphere3dPrefab {
    pub fn new(mesh: Handle<Mesh>, material: Handle<StandardMaterial>) -> Self {
        Self { mesh, material }
    }
}

#[derive(Bundle)]
struct DemoPreviewSphere3dBundle {
    marker: DemoPreviewSphere3dMarker,
    visual: StaticMesh3dBundle,
}

impl Prefab for DemoPreviewSphere3dPrefab {
    fn spawn(self, commands: &mut Commands) -> Entity {
        commands
            .spawn(DemoPreviewSphere3dBundle {
                marker: DemoPreviewSphere3dMarker,
                visual: StaticMesh3d::at(self.mesh, self.material, Vec3::new(0.0, 0.72, 0.0))
                    .into_bundle(),
            })
            .id()
    }
}
