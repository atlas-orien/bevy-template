use bevy::prelude::*;

#[derive(Component, Debug, Clone, Copy, PartialEq)]
pub struct RenderOpacity2d(pub f32);

impl Default for RenderOpacity2d {
    fn default() -> Self {
        Self(1.0)
    }
}
