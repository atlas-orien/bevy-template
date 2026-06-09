use bevy::prelude::*;

#[derive(Component, Debug, Clone, Copy, Default, PartialEq)]
pub struct RenderRotation2d {
    pub radians: f32,
}
