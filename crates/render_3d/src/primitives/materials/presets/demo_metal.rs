//! Demo 金属 PBR 材质预设。

use bevy::prelude::*;

use crate::primitives::materials::StandardSurface3d;
use crate::primitives::textures::TextureAsset3d;

const DEMO_METAL_BASE_COLOR: TextureAsset3d =
    TextureAsset3d::srgb("3d/materials/demo-metal/base-color.png");
const DEMO_METAL_NORMAL: TextureAsset3d =
    TextureAsset3d::linear("3d/materials/demo-metal/normal.png");
const DEMO_METAL_OCCLUSION_ROUGH_METAL: TextureAsset3d =
    TextureAsset3d::linear("3d/materials/demo-metal/occlusion-rough-metal.png");

pub struct DemoMetalMaterial3d;

impl DemoMetalMaterial3d {
    pub fn material(asset_server: &AssetServer) -> StandardMaterial {
        let occlusion_rough_metal = DEMO_METAL_OCCLUSION_ROUGH_METAL.load(asset_server);

        StandardSurface3d::new(Color::WHITE)
            .with_base_color_texture(DEMO_METAL_BASE_COLOR.load(asset_server))
            .with_normal_map_texture(DEMO_METAL_NORMAL.load(asset_server))
            .with_occlusion_texture(occlusion_rough_metal.clone())
            .with_metallic_roughness_texture(occlusion_rough_metal)
            .with_roughness(1.0)
            .with_metallic(1.0)
            .into_material()
    }
}
