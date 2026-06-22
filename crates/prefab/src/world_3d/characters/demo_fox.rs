use bevy::prelude::*;
use render_3d::products::characters::DemoFox3d;

use crate::Prefab;

pub struct DemoFox3dPrefab {
    visual: DemoFox3d,
}

impl DemoFox3dPrefab {
    pub fn new(visual: DemoFox3d) -> Self {
        Self { visual }
    }
}

#[derive(Component, Debug, Clone, Copy, Default, Eq, PartialEq)]
pub struct DemoFox3dPrefabMarker;

#[derive(Bundle)]
struct DemoFox3dPrefabBundle {
    marker: DemoFox3dPrefabMarker,
    transform: Transform,
    visibility: Visibility,
}

impl Prefab for DemoFox3dPrefab {
    fn spawn(self, commands: &mut Commands) -> Entity {
        commands
            .spawn(DemoFox3dPrefabBundle {
                marker: DemoFox3dPrefabMarker,
                transform: Transform::default(),
                visibility: Visibility::default(),
            })
            .with_children(|parent| {
                self.visual.spawn(parent);
            })
            .id()
    }
}
