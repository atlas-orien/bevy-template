use bevy::prelude::*;

#[derive(Component)]
pub struct Main3dCamera;

pub struct Camera3dPlugin;

impl Plugin for Camera3dPlugin {
    fn build(&self, _app: &mut App) {}
}
