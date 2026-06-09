use bevy::prelude::*;

#[derive(Component, Debug, Clone, Copy, Default)]
pub struct MainCamera2d;

#[derive(Bundle, Default)]
pub struct MainCamera2dBundle {
    pub camera: Camera2d,
    pub marker: MainCamera2d,
}

impl MainCamera2dBundle {
    pub fn new() -> Self {
        Self::default()
    }
}
