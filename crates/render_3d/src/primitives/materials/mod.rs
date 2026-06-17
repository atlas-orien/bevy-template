mod plugin;
pub mod presets;

use bevy::prelude::*;

pub use plugin::Materials3dPrimitivePlugin;
pub use presets::UvCheckerMaterial3d;

#[derive(Debug, Clone)]
pub struct StandardSurface3d {
    base_color: Color,
    base_color_texture: Option<Handle<Image>>,
    normal_map_texture: Option<Handle<Image>>,
    metallic_roughness_texture: Option<Handle<Image>>,
    emissive_texture: Option<Handle<Image>>,
    occlusion_texture: Option<Handle<Image>>,
    perceptual_roughness: f32,
    metallic: f32,
    unlit: bool,
}

impl StandardSurface3d {
    pub fn new(base_color: Color) -> Self {
        Self {
            base_color,
            base_color_texture: None,
            normal_map_texture: None,
            metallic_roughness_texture: None,
            emissive_texture: None,
            occlusion_texture: None,
            perceptual_roughness: 0.65,
            metallic: 0.0,
            unlit: false,
        }
    }

    pub fn with_base_color_texture(mut self, texture: Handle<Image>) -> Self {
        self.base_color_texture = Some(texture);
        self
    }

    pub fn with_normal_map_texture(mut self, texture: Handle<Image>) -> Self {
        self.normal_map_texture = Some(texture);
        self
    }

    pub fn with_metallic_roughness_texture(mut self, texture: Handle<Image>) -> Self {
        self.metallic_roughness_texture = Some(texture);
        self
    }

    pub fn with_emissive_texture(mut self, texture: Handle<Image>) -> Self {
        self.emissive_texture = Some(texture);
        self
    }

    pub fn with_occlusion_texture(mut self, texture: Handle<Image>) -> Self {
        self.occlusion_texture = Some(texture);
        self
    }

    pub fn with_roughness(mut self, perceptual_roughness: f32) -> Self {
        self.perceptual_roughness = perceptual_roughness;
        self
    }

    pub fn with_metallic(mut self, metallic: f32) -> Self {
        self.metallic = metallic;
        self
    }

    pub fn unlit(mut self) -> Self {
        self.unlit = true;
        self
    }

    pub fn into_material(self) -> StandardMaterial {
        StandardMaterial {
            base_color: self.base_color,
            base_color_texture: self.base_color_texture,
            normal_map_texture: self.normal_map_texture,
            metallic_roughness_texture: self.metallic_roughness_texture,
            emissive_texture: self.emissive_texture,
            occlusion_texture: self.occlusion_texture,
            perceptual_roughness: self.perceptual_roughness,
            metallic: self.metallic,
            unlit: self.unlit,
            ..default()
        }
    }
}
