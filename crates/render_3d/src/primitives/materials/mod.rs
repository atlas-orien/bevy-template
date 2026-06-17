mod plugin;

use bevy::prelude::*;

pub use plugin::Materials3dPrimitivePlugin;

#[derive(Debug, Clone)]
pub struct StandardSurface3d {
    base_color: Color,
    perceptual_roughness: f32,
    metallic: f32,
}

impl StandardSurface3d {
    pub fn new(base_color: Color) -> Self {
        Self {
            base_color,
            perceptual_roughness: 0.65,
            metallic: 0.0,
        }
    }

    pub fn with_roughness(mut self, perceptual_roughness: f32) -> Self {
        self.perceptual_roughness = perceptual_roughness;
        self
    }

    pub fn with_metallic(mut self, metallic: f32) -> Self {
        self.metallic = metallic;
        self
    }

    pub fn into_material(self) -> StandardMaterial {
        StandardMaterial {
            base_color: self.base_color,
            perceptual_roughness: self.perceptual_roughness,
            metallic: self.metallic,
            ..default()
        }
    }
}
