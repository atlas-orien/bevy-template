use bevy::prelude::*;

#[derive(Component, Debug, Clone, Copy, Default, Eq, PartialEq)]
pub struct ExampleCamera2d;

pub fn spawn_example_camera_2d_system(mut commands: Commands) {
    commands.spawn((Camera2d, ExampleCamera2d));
}
