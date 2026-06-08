use bevy::prelude::*;

#[derive(Component, Debug, Clone, Copy, Default)]
pub struct MainCamera2d;

pub struct Camera2dPlugin;

impl Plugin for Camera2dPlugin {
    fn build(&self, _app: &mut App) {}
}

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
