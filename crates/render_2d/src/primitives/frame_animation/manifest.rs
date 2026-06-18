//! 通用 sprite sheet frame manifest，加载自 `.frames.ron` 资源。

use std::collections::BTreeMap;

use bevy::{
    asset::{AssetLoader, LoadContext, io::Reader},
    prelude::*,
    reflect::TypePath,
};
use error::Result;
use helper::assets::manifests::{FrameClipManifest, FrameManifest};

#[derive(Asset, Debug, Clone, TypePath)]
pub struct FrameAnimationManifest2d {
    pub image: Handle<Image>,
    pub frame_size: UVec2,
    pub columns: u32,
    pub rows: u32,
    pub clips: BTreeMap<String, FrameAnimationClip2d>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FrameAnimationClip2d {
    pub frames: Vec<usize>,
    pub fps: f32,
    pub repeat: bool,
}

#[derive(Default, TypePath)]
pub struct FrameAnimationManifestLoader2d;

#[derive(Component, Debug, Clone, PartialEq)]
pub struct FrameAnimationHandle2d(pub Handle<FrameAnimationManifest2d>);

pub type FrameAnimationLoader2d = FrameAnimationManifestLoader2d;

impl FrameAnimationManifest2d {
    pub fn atlas_layout(&self) -> TextureAtlasLayout {
        let mut layout =
            TextureAtlasLayout::new_empty(self.frame_size * UVec2::new(self.columns, self.rows));
        for row in 0..self.rows {
            for column in 0..self.columns {
                let min = self.frame_size * UVec2::new(column, row);
                layout.textures.push(URect {
                    min,
                    max: min + self.frame_size,
                });
            }
        }
        layout
    }

    pub fn clip(&self, name: &str) -> Option<&FrameAnimationClip2d> {
        self.clips.get(name)
    }
}

impl From<FrameClipManifest> for FrameAnimationClip2d {
    fn from(value: FrameClipManifest) -> Self {
        Self {
            frames: value.frames,
            fps: value.fps,
            repeat: value.repeat,
        }
    }
}

impl AssetLoader for FrameAnimationManifestLoader2d {
    type Asset = FrameAnimationManifest2d;
    type Settings = ();
    type Error = error::GameError;

    async fn load(
        &self,
        reader: &mut dyn Reader,
        _settings: &Self::Settings,
        load_context: &mut LoadContext<'_>,
    ) -> Result<Self::Asset> {
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).await?;
        let manifest = FrameManifest::from_bytes(&bytes)?;
        let clips = manifest
            .clips
            .into_iter()
            .map(|(name, clip)| (name, clip.into()))
            .collect();

        Ok(FrameAnimationManifest2d {
            image: load_context.load(manifest.image),
            frame_size: UVec2::new(manifest.frame_size.0, manifest.frame_size.1),
            columns: manifest.columns,
            rows: manifest.rows,
            clips,
        })
    }

    fn extensions(&self) -> &[&str] {
        &["frames.ron"]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn manifest_builds_atlas_layout_from_resource_data() {
        let manifest = FrameAnimationManifest2d {
            image: Handle::default(),
            frame_size: UVec2::new(24, 24),
            columns: 7,
            rows: 1,
            clips: BTreeMap::new(),
        };

        let layout = manifest.atlas_layout();

        assert_eq!(layout.textures.len(), 7);
        assert_eq!(layout.size, UVec2::new(168, 24));
    }
}
