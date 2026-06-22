use bevy::prelude::*;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct GltfAsset {
    path: &'static str,
}

impl GltfAsset {
    pub const fn new(path: &'static str) -> Self {
        Self { path }
    }

    pub fn load_scene(self, asset_server: &AssetServer, scene_index: usize) -> Handle<Scene> {
        asset_server.load(GltfAssetLabel::Scene(scene_index).from_asset(self.path))
    }

    pub fn load_animation(
        self,
        asset_server: &AssetServer,
        animation_index: usize,
    ) -> Handle<AnimationClip> {
        asset_server.load(GltfAssetLabel::Animation(animation_index).from_asset(self.path))
    }
}

