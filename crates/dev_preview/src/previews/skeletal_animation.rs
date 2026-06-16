//! Demo skeletal animation 的开发预览入口。

use bevy::asset::AssetPlugin;
use bevy::prelude::*;
use bevy::window::{Window, WindowPlugin};
use catalog::demo::demo_skeleton;
use ecs::EcsPlugin;
use gameplay::api::GameplayApiPlugin;
use prefab::Prefab;
use render_2d::Render2dPlugin;
use render_2d::camera::FixedCamera2dBundle;

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
    commands.spawn(FixedCamera2dBundle::default());

    demo_skeleton(Vec2::new(0.0, -48.0), &asset_server).spawn(&mut commands);
}
