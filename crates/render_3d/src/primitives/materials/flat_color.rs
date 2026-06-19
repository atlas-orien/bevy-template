use bevy::prelude::*;

use super::MaterialSurface3d;

impl MaterialSurface3d {
    pub fn flat_color(base_color: Color) -> Self {
        Self::empty().with_base_color(base_color)
    }
}
