//! Demo 游戏流程的开发预览入口。

use bevy::asset::AssetPlugin;
use bevy::prelude::*;
use bevy::window::{Window, WindowPlugin};
use gameplay::GameplayPlugin;
use interaction::InteractionPlugin;
use peripherals::PeripheralsPlugin;

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
                        title: "Dev Preview - Demo Game".to_string(),
                        resolution: (1280, 720).into(),
                        ..default()
                    }),
                    ..default()
                }),
        )
        .add_plugins(PeripheralsPlugin)
        .add_plugins(InteractionPlugin)
        .add_plugins(GameplayPlugin::without_external_manager())
        .run();
}
