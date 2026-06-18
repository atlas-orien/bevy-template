use bevy::prelude::*;
use render_3d::products::props::{DemoCapsule3d, DemoCapsule3dBundle};

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
    visual: DemoCapsule3dBundle,
}

impl Prefab for DemoPreviewCapsule3dPrefab {
    fn spawn(self, commands: &mut Commands) -> Entity {
        commands
            .spawn(DemoPreviewCapsule3dBundle {
                marker: DemoPreviewCapsule3dMarker,
                visual: DemoCapsule3d::new(self.mesh, self.material).into_bundle(),
            })
            .id()
    }
}
