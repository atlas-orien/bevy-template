//! Demo 3D camera prefab.

use bevy::prelude::*;
use render_3d::primitives::camera::FixedCamera3dBundle;

use crate::Prefab;

#[derive(Component, Debug, Clone, Copy, Default, Eq, PartialEq)]
pub struct DemoPreviewCamera3dMarker;

#[derive(Default)]
pub struct DemoPreviewCamera3dPrefab;

#[derive(Bundle, Default)]
struct DemoPreviewCamera3dBundle {
    marker: DemoPreviewCamera3dMarker,
    camera: FixedCamera3dBundle,
}

impl Prefab for DemoPreviewCamera3dPrefab {
    fn spawn(self, commands: &mut Commands) -> Entity {
        commands.spawn(DemoPreviewCamera3dBundle::default()).id()
    }
}
