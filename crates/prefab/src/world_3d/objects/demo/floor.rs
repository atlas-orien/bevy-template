use bevy::prelude::*;
use render_3d::products::props::{DemoFloor3d, DemoFloor3dBundle};

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
    visual: DemoFloor3dBundle,
}

impl Prefab for DemoPreviewFloor3dPrefab {
    fn spawn(self, commands: &mut Commands) -> Entity {
        commands
            .spawn(DemoPreviewFloor3dBundle {
                marker: DemoPreviewFloor3dMarker,
                visual: DemoFloor3d::new(self.mesh, self.material).into_bundle(),
            })
            .id()
    }
}
