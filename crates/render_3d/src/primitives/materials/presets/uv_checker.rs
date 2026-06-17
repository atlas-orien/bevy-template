//! UV checker 材质预设，用于验证 3D mesh 的 UV 和表面走向。

use bevy::prelude::*;

use crate::primitives::materials::StandardSurface3d;

const UV_CHECKER_TEXTURE: &str = "3d/textures/demo_uv_checker_bw.png";

pub struct UvCheckerMaterial3d;

impl UvCheckerMaterial3d {
    pub fn material(asset_server: &AssetServer) -> StandardMaterial {
        StandardSurface3d::new(Color::WHITE)
            .with_base_color_texture(asset_server.load(UV_CHECKER_TEXTURE))
            .with_roughness(0.72)
            .with_metallic(0.0)
            .into_material()
    }
}
