use bevy::prelude::*;
use bevy::render::render_resource::Face;

use super::MaterialSurface3d;

impl MaterialSurface3d {
    pub fn with_alpha_mode(self, alpha_mode: AlphaMode) -> Self {
        self.set_alpha_mode(alpha_mode)
    }

    pub fn alpha_blend(self) -> Self {
        self.with_alpha_mode(AlphaMode::Blend)
    }

    pub fn alpha_mask(self, cutoff: f32) -> Self {
        self.with_alpha_mode(AlphaMode::Mask(cutoff))
    }

    pub fn double_sided(self) -> Self {
        self.set_double_sided(true).set_cull_mode(None)
    }

    pub fn with_cull_mode(self, cull_mode: Option<Face>) -> Self {
        self.set_cull_mode(cull_mode)
    }
}
