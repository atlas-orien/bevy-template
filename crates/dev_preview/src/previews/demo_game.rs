//! Demo 游戏流程的开发预览入口。

use bevy::asset::AssetPlugin;
use bevy::prelude::*;
use bevy::window::{Window, WindowPlugin};
use external_runtime::config::ExternalRuntimeSettings;
use external_runtime::manager::ExternalRuntimeManager;
use external_runtime::runtime::{ExternalRuntime, ExternalRuntimeConfig};
use gameplay::GameplayPlugin;
use gameplay::api::{ManagerUpdateChannel, RuntimeRequestChannel};
use interaction::InteractionPlugin;
use peripherals::PeripheralsPlugin;

pub async fn run() {
    let runtime_requests = RuntimeRequestChannel::new();
    let manager_updates = ManagerUpdateChannel::new();

    let manager = ExternalRuntimeManager::new(runtime_requests.sender(), manager_updates.inbox());
    let runtime_config = ExternalRuntimeSettings::load("config/services.toml")
        .map(ExternalRuntimeConfig::from)
        .unwrap_or_else(|error| {
            eprintln!("failed to load config/services.toml: {error}; using default runtime config");
            ExternalRuntimeConfig::default()
        });
    let external_runtime = ExternalRuntime::spawn(runtime_config, manager);

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
        .add_plugins(GameplayPlugin::new(
            runtime_requests.inbox(),
            manager_updates.sender(),
        ))
        .run();

    external_runtime.shutdown().await;
}
