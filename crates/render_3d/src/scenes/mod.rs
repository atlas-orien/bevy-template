use bevy::prelude::*;

#[derive(Component)]
pub struct Scene3dEntity;

pub struct Scenes3dPlugin;

impl Plugin for Scenes3dPlugin {
    fn build(&self, _app: &mut App) {}
}
