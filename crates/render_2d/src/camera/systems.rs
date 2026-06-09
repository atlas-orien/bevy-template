use bevy::prelude::*;

use super::MainCamera2dBundle;

pub fn spawn_main_camera_2d_system(mut commands: Commands) {
    commands.spawn(MainCamera2dBundle::new());
}
