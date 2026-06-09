pub mod main_camera;
pub mod systems;

use bevy::prelude::*;

pub use main_camera::{MainCamera2d, MainCamera2dBundle};

use self::systems::spawn_main_camera_2d_system;

pub struct Camera2dPlugin;

impl Plugin for Camera2dPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_main_camera_2d_system);
    }
}
