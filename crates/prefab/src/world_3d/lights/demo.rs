//! Demo 3D light prefab.

use bevy::prelude::*;
use render_3d::primitives::lights::SunLight3dBundle;

use crate::Prefab;

#[derive(Component, Debug, Clone, Copy, Default, Eq, PartialEq)]
pub struct DemoPreviewLights3dMarker;

#[derive(Default)]
pub struct DemoPreviewLights3dPrefab;

#[derive(Bundle)]
struct DemoPreviewLights3dBundle {
    marker: DemoPreviewLights3dMarker,
    sunlight: SunLight3dBundle,
}

impl Default for DemoPreviewLights3dBundle {
    fn default() -> Self {
        Self {
            marker: DemoPreviewLights3dMarker,
            sunlight: SunLight3dBundle::default(),
        }
    }
}

impl Prefab for DemoPreviewLights3dPrefab {
    fn spawn(self, commands: &mut Commands) -> Entity {
        commands.spawn(DemoPreviewLights3dBundle::default()).id()
    }
}
