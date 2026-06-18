//! Demo 金属 PBR 材质预设。

use bevy::prelude::*;
use helper::assets::TextureAsset;

use crate::primitives::materials::StandardSurface3d;

const DEMO_METAL_BASE_COLOR: TextureAsset =
    TextureAsset::srgb("3d/materials/demo-metal/base-color.png");
const DEMO_METAL_NORMAL: TextureAsset = TextureAsset::linear("3d/materials/demo-metal/normal.png");
const DEMO_METAL_OCCLUSION_ROUGH_METAL: TextureAsset =
    TextureAsset::linear("3d/materials/demo-metal/occlusion-rough-metal.png");

pub struct DemoMetalMaterial3d;

impl DemoMetalMaterial3d {
    pub fn material(asset_server: &AssetServer) -> StandardMaterial {
        StandardSurface3d::new(Color::WHITE)
            .with_base_color_texture_asset(DEMO_METAL_BASE_COLOR, asset_server)
            .with_normal_map_texture_asset(DEMO_METAL_NORMAL, asset_server)
            .with_occlusion_roughness_metallic_texture_asset(
                DEMO_METAL_OCCLUSION_ROUGH_METAL,
                asset_server,
            )
            .with_roughness(1.0)
            .with_metallic(1.0)
            .into_material()
    }
}
