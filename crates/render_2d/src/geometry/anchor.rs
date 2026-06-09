use bevy::prelude::*;

#[derive(Component, Debug, Clone, Copy, Default, PartialEq)]
pub enum RenderAnchor2d {
    #[default]
    Center,
    TopLeft,
    BottomLeft,
    Custom(Vec2),
}
