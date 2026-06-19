use bevy::prelude::*;
use bevy::render::render_resource::Face;

#[derive(Debug, Clone)]
pub struct MaterialSurface3d {
    base_color: Option<Color>,
    base_color_texture: Option<Handle<Image>>,
    normal_map_texture: Option<Handle<Image>>,
    metallic_roughness_texture: Option<Handle<Image>>,
    emissive: Option<LinearRgba>,
    emissive_texture: Option<Handle<Image>>,
    emissive_exposure_weight: f32,
    occlusion_texture: Option<Handle<Image>>,
    perceptual_roughness: f32,
    metallic: f32,
    alpha_mode: AlphaMode,
    double_sided: bool,
    cull_mode: Option<Face>,
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
            emissive: None,
            emissive_texture: None,
            emissive_exposure_weight: 0.0,
            occlusion_texture: None,
            perceptual_roughness: 0.65,
            metallic: 0.0,
            alpha_mode: AlphaMode::Opaque,
            double_sided: false,
            cull_mode: Some(Face::Back),
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
            emissive: surface.emissive.unwrap_or(LinearRgba::BLACK),
            emissive_texture: surface.emissive_texture,
            emissive_exposure_weight: surface.emissive_exposure_weight,
            occlusion_texture: surface.occlusion_texture,
            perceptual_roughness: surface.perceptual_roughness,
            metallic: surface.metallic,
            alpha_mode: surface.alpha_mode,
            double_sided: surface.double_sided,
            cull_mode: surface.cull_mode,
            unlit: surface.unlit,
            ..default()
        }
    }
}

impl MaterialSurface3d {
    pub(super) fn set_emissive(mut self, emissive: LinearRgba) -> Self {
        self.emissive = Some(emissive);
        self
    }

    pub(super) fn set_emissive_exposure_weight(mut self, weight: f32) -> Self {
        self.emissive_exposure_weight = weight;
        self
    }

    pub(super) fn set_alpha_mode(mut self, alpha_mode: AlphaMode) -> Self {
        self.alpha_mode = alpha_mode;
        self
    }

    pub(super) fn set_double_sided(mut self, double_sided: bool) -> Self {
        self.double_sided = double_sided;
        self
    }

    pub(super) fn set_cull_mode(mut self, cull_mode: Option<Face>) -> Self {
        self.cull_mode = cull_mode;
        self
    }
}
