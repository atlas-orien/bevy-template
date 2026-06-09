use bevy::prelude::*;

#[derive(Component, Debug, Clone, Copy, Default, Eq, PartialEq)]
pub struct RenderFlip2d {
    pub x: bool,
    pub y: bool,
}
