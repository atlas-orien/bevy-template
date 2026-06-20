use bevy::prelude::*;

#[derive(Component, Debug, Clone, Copy, Default, Eq, PartialEq)]
pub struct SceneModel3dMarker;

#[derive(Debug, Clone)]
pub struct SceneModel3d {
    scene: Handle<Scene>,
}

impl SceneModel3d {
    pub fn new(scene: Handle<Scene>) -> Self {
        Self { scene }
    }

    pub fn scene(&self) -> &Handle<Scene> {
        &self.scene
    }

    pub fn with_scene(mut self, scene: Handle<Scene>) -> Self {
        self.scene = scene;
        self
    }

    pub fn into_bundle(self, transform: Transform) -> SceneModel3dBundle {
        SceneModel3dBundle {
            marker: SceneModel3dMarker,
            scene: SceneRoot(self.scene),
            transform,
            visibility: Visibility::default(),
        }
    }
}

#[derive(Bundle)]
pub struct SceneModel3dBundle {
    marker: SceneModel3dMarker,
    scene: SceneRoot,
    transform: Transform,
    visibility: Visibility,
}
