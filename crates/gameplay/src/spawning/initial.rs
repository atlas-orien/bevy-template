use bevy::prelude::*;
use catalog::demo::{demo_ground, demo_player};
use prefab::lifecycle::GameplaySessionEntity;
use prefab::world_2d::demo_level::{
    DemoBackgroundPrefab, DemoLandmarkPrefab, DemoRockPrefab, DemoSensorZonePrefab,
};
use render_2d::camera::DemoWorldCamera2d;

use super::plan::GameplaySpawnPlan;

pub fn default_gameplay_spawn_plan(asset_server: &AssetServer) -> GameplaySpawnPlan {
    GameplaySpawnPlan::new()
        .with(DemoBackgroundPrefab)
        .with(demo_ground(asset_server))
        .with(demo_player(Vec2::new(0.0, 96.0), asset_server))
        .with(DemoRockPrefab::new(Vec2::new(-220.0, 94.0)))
        .with(DemoRockPrefab::new(Vec2::new(260.0, 94.0)))
        .with(DemoLandmarkPrefab::new(
            Vec2::new(-900.0, 156.0),
            Color::srgb(0.95, 0.22, 0.18),
        ))
        .with(DemoLandmarkPrefab::new(
            Vec2::new(0.0, 156.0),
            Color::srgb(0.95, 0.82, 0.18),
        ))
        .with(DemoLandmarkPrefab::new(
            Vec2::new(900.0, 156.0),
            Color::srgb(0.18, 0.72, 0.95),
        ))
        .with(DemoSensorZonePrefab::new(Vec2::new(140.0, 98.0)))
}

pub fn spawn_initial_gameplay_plan_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut demo_session_started: MessageWriter<prefab::demo_events::DemoSessionStartedEvent>,
) {
    commands.spawn((DemoWorldCamera2d::default(), GameplaySessionEntity));

    for prefab in default_gameplay_spawn_plan(&asset_server).into_prefabs() {
        prefab.spawn_boxed(&mut commands);
    }

    demo_session_started.write(prefab::demo_events::DemoSessionStartedEvent);

    info!("Initial gameplay spawn plan completed.");
}
