use bevy::prelude::*;

use super::MaterialSurface3d;

impl MaterialSurface3d {
    pub fn with_textured_pbr(
        self,
        base_color_texture: Handle<Image>,
        normal_map_texture: Handle<Image>,
        occlusion_roughness_metallic_texture: Handle<Image>,
    ) -> Self {
        self.with_base_color_texture(base_color_texture)
            .with_normal_map_texture(normal_map_texture)
            .with_occlusion_roughness_metallic_texture(occlusion_roughness_metallic_texture)
    }
}
