use bevy::prelude::*;

#[derive(Component, Debug, Clone, Copy, Default, Eq, PartialEq)]
pub struct Model3dMarker;

#[derive(Debug, Clone)]
pub struct Model3d {
    scene: Handle<Scene>,
}

impl Model3d {
    pub fn new(scene: Handle<Scene>) -> Self {
        Self { scene }
    }

    pub fn into_bundle(self, transform: Transform) -> Model3dBundle {
        Model3dBundle {
            marker: Model3dMarker,
            scene: SceneRoot(self.scene),
            transform,
            visibility: Visibility::default(),
        }
    }
}

#[derive(Bundle)]
pub struct Model3dBundle {
    marker: Model3dMarker,
    scene: SceneRoot,
    transform: Transform,
    visibility: Visibility,
}
