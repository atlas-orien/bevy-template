use bevy::prelude::*;

#[derive(Component, Debug, Clone, Copy, Default, Eq, PartialEq, Ord, PartialOrd)]
pub struct RenderZIndex2d(pub i32);
