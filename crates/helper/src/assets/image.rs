use bevy::image::ImageLoaderSettings;
use bevy::prelude::*;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum ImageColorSpace {
    Srgb,
    Linear,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct ImageAsset {
    path: &'static str,
    color_space: ImageColorSpace,
}

impl ImageAsset {
    pub const fn srgb(path: &'static str) -> Self {
        Self {
            path,
            color_space: ImageColorSpace::Srgb,
        }
    }

    pub const fn linear(path: &'static str) -> Self {
        Self {
            path,
            color_space: ImageColorSpace::Linear,
        }
    }

    pub fn load(self, asset_server: &AssetServer) -> Handle<Image> {
        match self.color_space {
            ImageColorSpace::Srgb => asset_server.load(self.path),
            ImageColorSpace::Linear => asset_server
                .load_with_settings(self.path, |settings: &mut ImageLoaderSettings| {
                    settings.is_srgb = false
                }),
        }
    }
}
