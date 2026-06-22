use bevy::prelude::*;
use helper::assets::GltfAsset;

use super::systems::DemoFox3dAnimationState;
use crate::capabilities::animation::AnimationClip3d;

#[derive(Component, Debug, Clone)]
pub struct DemoFox3dAnimationSet {
    pub idle: AnimationClip3d,
    pub walk: AnimationClip3d,
    pub run: AnimationClip3d,
}

impl DemoFox3dAnimationSet {
    pub fn from_gltf_model(
        asset_server: &AssetServer,
        animation_graphs: &mut Assets<AnimationGraph>,
        model: GltfAsset,
    ) -> Self {
        Self {
            idle: AnimationClip3d::from_clip(
                model.load_animation(asset_server, 0),
                animation_graphs,
            ),
            walk: AnimationClip3d::from_clip(
                model.load_animation(asset_server, 1),
                animation_graphs,
            ),
            run: AnimationClip3d::from_clip(
                model.load_animation(asset_server, 2),
                animation_graphs,
            ),
        }
    }

    pub fn clip(&self, state: DemoFox3dAnimationState) -> AnimationClip3d {
        match state {
            DemoFox3dAnimationState::Idle => self.idle.clone(),
            DemoFox3dAnimationState::Walk => self.walk.clone(),
            DemoFox3dAnimationState::Run => self.run.clone(),
        }
    }
}
