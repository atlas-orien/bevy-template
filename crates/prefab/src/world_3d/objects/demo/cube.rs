use bevy::prelude::*;
use render_3d::products::props::{DemoCube3d, DemoCube3dBundle};

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
    visual: DemoCube3dBundle,
}

impl Prefab for DemoPreviewCube3dPrefab {
    fn spawn(self, commands: &mut Commands) -> Entity {
        commands
            .spawn(DemoPreviewCube3dBundle {
                marker: DemoPreviewCube3dMarker,
                visual: DemoCube3d::new(self.mesh, self.material).into_bundle(),
            })
            .id()
    }
}
