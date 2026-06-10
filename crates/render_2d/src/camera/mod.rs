pub mod example;

use bevy::prelude::*;

use self::example::spawn_example_camera_2d_system;

pub struct Camera2dPlugin;

impl Plugin for Camera2dPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_example_camera_2d_system);
    }
}
