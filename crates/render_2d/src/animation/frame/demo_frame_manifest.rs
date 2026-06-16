//! Demo sprite sheet frame manifest loaded from `.frames.ron` assets.

use std::collections::BTreeMap;
use std::io::{Error as IoError, ErrorKind};

use bevy::{
    asset::{AssetLoader, LoadContext, io::Reader},
    prelude::*,
    reflect::TypePath,
};
use error::Result;
use serde::Deserialize;

#[derive(Asset, Debug, Clone, TypePath)]
pub struct DemoFrameManifest2d {
    pub image: Handle<Image>,
    pub frame_size: UVec2,
    pub columns: u32,
    pub rows: u32,
    pub clips: BTreeMap<String, DemoFrameClip2d>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DemoFrameClip2d {
    pub frames: Vec<usize>,
    pub fps: f32,
    pub repeat: bool,
}

#[derive(Default, TypePath)]
pub struct DemoFrameManifestLoader2d;

#[derive(Component, Debug, Clone, PartialEq)]
pub struct DemoFrameManifestHandle2d(pub Handle<DemoFrameManifest2d>);

#[derive(Deserialize)]
struct DemoFrameManifestRon {
    image: String,
    frame_size: (u32, u32),
    columns: u32,
    rows: u32,
    clips: BTreeMap<String, DemoFrameClipRon>,
}

#[derive(Deserialize)]
struct DemoFrameClipRon {
    frames: Vec<usize>,
    fps: f32,
    repeat: bool,
}

impl DemoFrameManifest2d {
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

    pub fn clip(&self, name: &str) -> Option<&DemoFrameClip2d> {
        self.clips.get(name)
    }
}

impl From<DemoFrameClipRon> for DemoFrameClip2d {
    fn from(value: DemoFrameClipRon) -> Self {
        Self {
            frames: value.frames,
            fps: value.fps,
            repeat: value.repeat,
        }
    }
}

impl AssetLoader for DemoFrameManifestLoader2d {
    type Asset = DemoFrameManifest2d;
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
        let manifest: DemoFrameManifestRon = ron::de::from_bytes(&bytes)
            .map_err(|error| IoError::new(ErrorKind::InvalidData, error))?;
        let clips = manifest
            .clips
            .into_iter()
            .map(|(name, clip)| (name, clip.into()))
            .collect();

        Ok(DemoFrameManifest2d {
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
        let manifest = DemoFrameManifest2d {
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
