use bevy::prelude::*;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct ImageAsset {
    path: &'static str,
}

impl ImageAsset {
    pub const fn new(path: &'static str) -> Self {
        Self { path }
    }

    pub fn load(self, asset_server: &AssetServer) -> Handle<Image> {
        asset_server.load(self.path)
    }
}
