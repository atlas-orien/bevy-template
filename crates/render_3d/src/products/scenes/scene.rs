use bevy::prelude::*;

#[derive(Debug, Clone)]
pub struct Scene3d {
    scene: Handle<Scene>,
}

impl Scene3d {
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

    pub fn into_bundle(self) -> Scene3dBundle {
        Scene3dBundle {
            marker: Scene3dEntityMarker,
            scene: SceneRoot(self.scene),
        }
    }
}

#[derive(Bundle)]
pub struct Scene3dBundle {
    marker: Scene3dEntityMarker,
    scene: SceneRoot,
}

use super::Scene3dEntityMarker;
