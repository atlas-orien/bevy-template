use bevy::prelude::*;

#[derive(Component, Debug, Clone, Copy, Default, Eq, PartialEq)]
pub struct ExampleDebugVisual2d;

#[derive(Component, Debug, Clone, Copy, PartialEq)]
pub struct ExampleDebugBounds2d {
    pub size: Vec2,
    pub color: Color,
}

#[derive(Bundle)]
pub struct ExampleDebugBounds2dBundle {
    pub marker: ExampleDebugVisual2d,
    pub bounds: ExampleDebugBounds2d,
    pub transform: Transform,
}

impl ExampleDebugBounds2dBundle {
    pub fn new(size: Vec2, color: Color, translation: Vec3) -> Self {
        Self {
            marker: ExampleDebugVisual2d,
            bounds: ExampleDebugBounds2d { size, color },
            transform: Transform::from_translation(translation),
        }
    }
}
