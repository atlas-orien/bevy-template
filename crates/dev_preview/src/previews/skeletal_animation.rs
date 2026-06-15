//! Demo skeletal animation 的开发预览入口。

use bevy::asset::AssetPlugin;
use bevy::prelude::*;
use bevy::window::{Window, WindowPlugin};
use ecs::EcsPlugin;
use gameplay::api::GameplayApiPlugin;
use prefab::Prefab;
use prefab::world_2d::demo_level::DemoSkeletonPrefab;
use render_2d::Render2dPlugin;
use render_2d::camera::DemoWorldCamera2dBundle;

pub fn run() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(AssetPlugin {
                    file_path: "../../assets".to_string(),
                    ..default()
                })
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Dev Preview - Skeletal Animation".to_string(),
                        resolution: (960, 540).into(),
                        ..default()
                    }),
                    ..default()
                }),
        )
        .add_plugins(EcsPlugin)
        .add_plugins(Render2dPlugin)
        .add_plugins(GameplayApiPlugin)
        .add_systems(Startup, spawn_skeletal_animation_preview_system)
        .run();
}

fn spawn_skeletal_animation_preview_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(DemoWorldCamera2dBundle::default());

    DemoSkeletonPrefab::new(
        Vec2::new(0.0, -48.0),
        asset_server.load("2d/static/props/demo-skeletal-bone/demo-skeletal-bone.png"),
        asset_server.load("2d/static/props/demo-skeletal-joint/demo-skeletal-joint.png"),
    )
    .spawn(&mut commands);
}
