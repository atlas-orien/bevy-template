use bevy::image::ImageLoaderSettings;
use bevy::prelude::*;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum TextureColorSpace3d {
    Srgb,
    Linear,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct TextureAsset3d {
    path: &'static str,
    color_space: TextureColorSpace3d,
}

impl TextureAsset3d {
    pub const fn srgb(path: &'static str) -> Self {
        Self {
            path,
            color_space: TextureColorSpace3d::Srgb,
        }
    }

    pub const fn linear(path: &'static str) -> Self {
        Self {
            path,
            color_space: TextureColorSpace3d::Linear,
        }
    }

    pub fn load(self, asset_server: &AssetServer) -> Handle<Image> {
        match self.color_space {
            TextureColorSpace3d::Srgb => asset_server.load(self.path),
            TextureColorSpace3d::Linear => asset_server
                .load_with_settings(self.path, |settings: &mut ImageLoaderSettings| {
                    settings.is_srgb = false
                }),
        }
    }
}
