use bevy::prelude::*;
use catalog::demo::{
    DemoBackground, DemoBgm, DemoGround, DemoLandmark, DemoPlayer, DemoRock, DemoSensorZone,
    WorldCamera2d,
};

use super::plan::GameplaySpawnPlan;

pub fn default_gameplay_spawn_plan(asset_server: &AssetServer) -> GameplaySpawnPlan {
    GameplaySpawnPlan::new()
        .with(WorldCamera2d::prefab())
        .with(DemoBackground::prefab())
        .with(DemoGround::prefab(asset_server))
        .with(DemoPlayer::at(Vec2::new(0.0, 96.0)).prefab(asset_server))
        .with(DemoRock::at(Vec2::new(-220.0, 94.0)).prefab())
        .with(DemoRock::at(Vec2::new(260.0, 94.0)).prefab())
        .with(DemoLandmark::new(Vec2::new(-900.0, 156.0), Color::srgb(0.95, 0.22, 0.18)).prefab())
        .with(DemoLandmark::new(Vec2::new(0.0, 156.0), Color::srgb(0.95, 0.82, 0.18)).prefab())
        .with(DemoLandmark::new(Vec2::new(900.0, 156.0), Color::srgb(0.18, 0.72, 0.95)).prefab())
        .with(DemoSensorZone::at(Vec2::new(140.0, 98.0)).prefab())
}

pub fn spawn_initial_gameplay_plan_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut demo_session_started: MessageWriter<prefab::demo_events::DemoSessionStartedEvent>,
) {
    commands.insert_resource(DemoBgm::resource());

    for prefab in default_gameplay_spawn_plan(&asset_server).into_prefabs() {
        prefab.spawn_boxed(&mut commands);
    }

    demo_session_started.write(prefab::demo_events::DemoSessionStartedEvent);

    info!("Initial gameplay spawn plan completed.");
}
