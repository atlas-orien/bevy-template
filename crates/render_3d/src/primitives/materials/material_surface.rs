use bevy::prelude::*;

#[derive(Debug, Clone)]
pub struct MaterialSurface3d {
    base_color: Option<Color>,
    base_color_texture: Option<Handle<Image>>,
    normal_map_texture: Option<Handle<Image>>,
    metallic_roughness_texture: Option<Handle<Image>>,
    emissive_texture: Option<Handle<Image>>,
    occlusion_texture: Option<Handle<Image>>,
    perceptual_roughness: f32,
    metallic: f32,
    unlit: bool,
}

impl MaterialSurface3d {
    pub fn empty() -> Self {
        Self::default()
    }

    pub fn with_base_color(mut self, base_color: Color) -> Self {
        self.base_color = Some(base_color);
        self
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

    pub fn with_occlusion_roughness_metallic_texture(mut self, texture: Handle<Image>) -> Self {
        self.occlusion_texture = Some(texture.clone());
        self.metallic_roughness_texture = Some(texture);
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
}

impl Default for MaterialSurface3d {
    fn default() -> Self {
        Self {
            base_color: None,
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
}

impl From<MaterialSurface3d> for StandardMaterial {
    fn from(surface: MaterialSurface3d) -> Self {
        Self {
            base_color: surface.base_color.unwrap_or(Color::WHITE),
            base_color_texture: surface.base_color_texture,
            normal_map_texture: surface.normal_map_texture,
            metallic_roughness_texture: surface.metallic_roughness_texture,
            emissive_texture: surface.emissive_texture,
            occlusion_texture: surface.occlusion_texture,
            perceptual_roughness: surface.perceptual_roughness,
            metallic: surface.metallic,
            unlit: surface.unlit,
            ..default()
        }
    }
}
