use bevy::prelude::*;

use crate::capabilities::animation::{AnimationPlayback3d, AnimationPlayback3dBundle};

#[derive(Debug, Clone)]
pub struct Scene3d {
    scene: Handle<Scene>,
}

#[derive(Debug, Clone)]
pub struct AnimatedScene3d {
    scene: Scene3d,
    playback: AnimationPlayback3d,
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

    pub fn with_animation(self, playback: AnimationPlayback3d) -> AnimatedScene3d {
        AnimatedScene3d {
            scene: self,
            playback,
        }
    }

    pub fn into_bundle(self) -> Scene3dBundle {
        Scene3dBundle {
            marker: Scene3dEntityMarker,
            scene: SceneRoot(self.scene),
        }
    }
}

impl AnimatedScene3d {
    pub fn new(scene: Handle<Scene>, playback: AnimationPlayback3d) -> Self {
        Scene3d::new(scene).with_animation(playback)
    }

    pub fn scene(&self) -> &Scene3d {
        &self.scene
    }

    pub fn playback(&self) -> &AnimationPlayback3d {
        &self.playback
    }

    pub fn into_bundle(self) -> AnimatedScene3dBundle {
        AnimatedScene3dBundle {
            scene: self.scene.into_bundle(),
            playback: self.playback.into_bundle(),
        }
    }
}

#[derive(Bundle)]
pub struct Scene3dBundle {
    marker: Scene3dEntityMarker,
    scene: SceneRoot,
}

#[derive(Bundle)]
pub struct AnimatedScene3dBundle {
    scene: Scene3dBundle,
    playback: AnimationPlayback3dBundle,
}

use super::Scene3dEntityMarker;
