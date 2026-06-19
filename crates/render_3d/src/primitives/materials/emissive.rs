use bevy::prelude::*;

use super::MaterialSurface3d;

impl MaterialSurface3d {
    pub fn with_emissive_color(self, emissive: LinearRgba) -> Self {
        self.set_emissive(emissive)
    }

    pub fn with_emissive_strength(self, color: LinearRgba, strength: f32) -> Self {
        self.set_emissive(LinearRgba::rgb(
            color.red * strength,
            color.green * strength,
            color.blue * strength,
        ))
    }

    pub fn with_emissive_exposure_weight(self, weight: f32) -> Self {
        self.set_emissive_exposure_weight(weight)
    }
}
