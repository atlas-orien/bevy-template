use bevy::prelude::*;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct ShaderAsset {
    path: &'static str,
}

impl ShaderAsset {
    pub const fn new(path: &'static str) -> Self {
        Self { path }
    }

    pub fn load(self, asset_server: &AssetServer) -> Handle<Shader> {
        asset_server.load(self.path)
    }
}
