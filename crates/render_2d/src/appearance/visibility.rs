use bevy::prelude::*;

#[derive(Component, Debug, Clone, Copy, Eq, PartialEq)]
pub struct RenderVisibility2d(pub bool);

impl Default for RenderVisibility2d {
    fn default() -> Self {
        Self(true)
    }
}
