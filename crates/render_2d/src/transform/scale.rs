use bevy::prelude::*;

#[derive(Component, Debug, Clone, Copy, PartialEq)]
pub struct RenderScale2d(pub Vec2);

impl Default for RenderScale2d {
    fn default() -> Self {
        Self(Vec2::ONE)
    }
}
