//! 3D primitives 的开发预览入口。

use bevy::asset::AssetPlugin;
use bevy::prelude::*;
use bevy::window::{Window, WindowPlugin};
use catalog::world_3d::{
    DemoPreviewCamera3d, DemoPreviewCapsule3d, DemoPreviewCube3d, DemoPreviewFloor3d,
    DemoPreviewLights3d, DemoPreviewSphere3d,
};
use prefab::Prefab;
use render_3d::Render3dPlugin;

pub fn run() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(AssetPlugin {
                    file_path: "../../assets".to_string(),
                    ..default()
                })
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Dev Preview - 3D Primitives".to_string(),
                        resolution: (1280, 720).into(),
                        ..default()
                    }),
                    ..default()
                }),
        )
        .add_plugins(Render3dPlugin)
        .insert_resource(GlobalAmbientLight {
            color: Color::WHITE,
            brightness: 0.0,
            affects_lightmapped_meshes: true,
        })
        .add_systems(Startup, spawn_3d_primitives_preview_system)
        .run();
}

fn spawn_3d_primitives_preview_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    DemoPreviewCamera3d::prefab().spawn(&mut commands);
    DemoPreviewLights3d::prefab().spawn(&mut commands);
    DemoPreviewFloor3d::prefab(&mut meshes, &mut materials).spawn(&mut commands);
    DemoPreviewCube3d::prefab(&mut meshes, &mut materials).spawn(&mut commands);
    DemoPreviewSphere3d::prefab(&asset_server, &mut meshes, &mut materials).spawn(&mut commands);
    DemoPreviewCapsule3d::prefab(&mut meshes, &mut materials).spawn(&mut commands);
}
