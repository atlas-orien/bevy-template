use bevy::prelude::*;

#[derive(Component, Debug, Clone, Copy, PartialEq)]
pub enum RenderShape2d {
    Rect { size: Vec2 },
    Circle { radius: f32 },
}
