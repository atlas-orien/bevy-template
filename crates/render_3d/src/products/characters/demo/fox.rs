use bevy::ecs::hierarchy::ChildSpawnerCommands;
use bevy::prelude::*;

use crate::capabilities::animation::AnimationPlayback3d;
use crate::capabilities::animation::demo::{
    DemoFox3dAnimationSet, DemoFox3dAnimationState, DemoFox3dAnimationStateSet,
};
use crate::products::scenes::{AnimatedScene3d, AnimatedScene3dBundle};

#[derive(Component, Debug, Clone, Copy, Default, Eq, PartialEq)]
pub struct DemoFox3dMarker;

#[derive(Debug, Clone)]
pub struct DemoFox3d {
    scene: Handle<Scene>,
    animations: DemoFox3dAnimationSet,
    initial_state: DemoFox3dAnimationState,
    transform: Transform,
}

impl DemoFox3d {
    pub fn new(
        scene: Handle<Scene>,
        animations: DemoFox3dAnimationSet,
        initial_state: DemoFox3dAnimationState,
        transform: Transform,
    ) -> Self {
        Self {
            scene,
            animations,
            initial_state,
            transform,
        }
    }

    pub fn spawn(self, parent: &mut ChildSpawnerCommands) {
        parent.spawn(self.into_bundle());
    }

    fn into_bundle(self) -> DemoFox3dBundle {
        let playback = AnimationPlayback3d::repeat(self.animations.clip(self.initial_state));

        DemoFox3dBundle {
            marker: DemoFox3dMarker,
            state: DemoFox3dAnimationStateSet::new(self.initial_state),
            animations: self.animations,
            scene: AnimatedScene3d::new(self.scene, playback).into_bundle(),
            transform: self.transform,
            visibility: Visibility::default(),
        }
    }
}

#[derive(Bundle)]
struct DemoFox3dBundle {
    marker: DemoFox3dMarker,
    state: DemoFox3dAnimationStateSet,
    animations: DemoFox3dAnimationSet,
    scene: AnimatedScene3dBundle,
    transform: Transform,
    visibility: Visibility,
}
