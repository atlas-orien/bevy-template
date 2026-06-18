use bevy::image::ImageLoaderSettings;
use bevy::prelude::*;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum TextureColorSpace {
    Srgb,
    Linear,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct TextureAsset {
    path: &'static str,
    color_space: TextureColorSpace,
}

impl TextureAsset {
    pub const fn srgb(path: &'static str) -> Self {
        Self {
            path,
            color_space: TextureColorSpace::Srgb,
        }
    }

    pub const fn linear(path: &'static str) -> Self {
        Self {
            path,
            color_space: TextureColorSpace::Linear,
        }
    }

    pub fn load(self, asset_server: &AssetServer) -> Handle<Image> {
        match self.color_space {
            TextureColorSpace::Srgb => asset_server.load(self.path),
            TextureColorSpace::Linear => asset_server
                .load_with_settings(self.path, |settings: &mut ImageLoaderSettings| {
                    settings.is_srgb = false
                }),
        }
    }
}
