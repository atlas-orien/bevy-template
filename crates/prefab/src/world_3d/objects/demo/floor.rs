use bevy::prelude::*;
use render_3d::primitives::meshes::{StaticMesh3d, StaticMesh3dBundle};

use crate::Prefab;

#[derive(Component, Debug, Clone, Copy, Default, Eq, PartialEq)]
pub struct DemoPreviewFloor3dMarker;

pub struct DemoPreviewFloor3dPrefab {
    mesh: Handle<Mesh>,
    material: Handle<StandardMaterial>,
}

impl DemoPreviewFloor3dPrefab {
    pub fn new(mesh: Handle<Mesh>, material: Handle<StandardMaterial>) -> Self {
        Self { mesh, material }
    }
}

#[derive(Bundle)]
struct DemoPreviewFloor3dBundle {
    marker: DemoPreviewFloor3dMarker,
    visual: StaticMesh3dBundle,
}

impl Prefab for DemoPreviewFloor3dPrefab {
    fn spawn(self, commands: &mut Commands) -> Entity {
        commands
            .spawn(DemoPreviewFloor3dBundle {
                marker: DemoPreviewFloor3dMarker,
                visual: StaticMesh3d::at(self.mesh, self.material, Vec3::ZERO).into_bundle(),
            })
            .id()
    }
}
