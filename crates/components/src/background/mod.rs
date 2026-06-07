use bevy::prelude::*;

#[derive(Component, Debug, Clone, Copy)]
pub struct BackgroundLayer {
    pub order: i32,
}

pub struct BackgroundPlugin;

impl Plugin for BackgroundPlugin {
    fn build(&self, _app: &mut App) {}
}
