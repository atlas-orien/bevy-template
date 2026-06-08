use bevy::prelude::*;
use render_2d::camera::MainCamera2dBundle;
use render_2d::environment::{Background2dBundle, Ground2dBundle};

pub fn spawn_main_camera_2d_prefab(commands: &mut Commands) -> Entity {
    commands.spawn(MainCamera2dBundle::new()).id()
}

pub fn spawn_demo_background_2d_prefab(commands: &mut Commands) -> [Entity; 2] {
    let background = commands
        .spawn(Background2dBundle::new(
            Color::srgb(0.10, 0.13, 0.16),
            Vec2::new(1280.0, 720.0),
            -10.0,
        ))
        .id();
    let ground = commands
        .spawn(Ground2dBundle::new(
            Color::srgb(0.22, 0.38, 0.24),
            Vec2::new(1280.0, 80.0),
            Vec3::new(0.0, -250.0, -5.0),
        ))
        .id();

    [background, ground]
}
