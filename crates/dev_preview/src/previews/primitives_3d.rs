//! 3D primitives 的开发预览入口。

use bevy::prelude::*;
use bevy::window::{Window, WindowPlugin};
use catalog::world_3d::{DemoPreviewCamera3d, DemoPreviewItems3d, DemoPreviewLights3d};
use prefab::Prefab;
use render_3d::Render3dPlugin;

pub fn run() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Dev Preview - 3D Primitives".to_string(),
                resolution: (1280, 720).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(Render3dPlugin)
        .add_systems(Startup, spawn_3d_primitives_preview_system)
        .run();
}

fn spawn_3d_primitives_preview_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    DemoPreviewCamera3d::prefab().spawn(&mut commands);
    DemoPreviewLights3d::prefab().spawn(&mut commands);
    DemoPreviewItems3d::prefab(&mut meshes, &mut materials).spawn(&mut commands);
}
