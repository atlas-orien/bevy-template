use bevy::prelude::*;

#[derive(Component)]
pub struct Scene3dEntity;

pub struct Scene3dPlugin;

impl Plugin for Scene3dPlugin {
    fn build(&self, _app: &mut App) {}
}
