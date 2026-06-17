//! Demo 金属 PBR 材质预设。

use bevy::prelude::*;

use crate::primitives::materials::StandardSurface3d;

const DEMO_METAL_BASE_COLOR: &str = "3d/materials/demo-metal/base-color.png";
const DEMO_METAL_NORMAL: &str = "3d/materials/demo-metal/normal.png";
const DEMO_METAL_OCCLUSION_ROUGH_METAL: &str = "3d/materials/demo-metal/occlusion-rough-metal.png";

pub struct DemoMetalMaterial3d;

impl DemoMetalMaterial3d {
    pub fn material(asset_server: &AssetServer) -> StandardMaterial {
        let occlusion_rough_metal = asset_server.load(DEMO_METAL_OCCLUSION_ROUGH_METAL);

        StandardSurface3d::new(Color::WHITE)
            .with_base_color_texture(asset_server.load(DEMO_METAL_BASE_COLOR))
            .with_normal_map_texture(asset_server.load(DEMO_METAL_NORMAL))
            .with_occlusion_texture(occlusion_rough_metal.clone())
            .with_metallic_roughness_texture(occlusion_rough_metal)
            .with_roughness(1.0)
            .with_metallic(1.0)
            .into_material()
    }
}
