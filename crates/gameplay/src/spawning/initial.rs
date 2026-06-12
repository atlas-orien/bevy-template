use bevy::{
    image::{ImageArrayLayout, ImageLoaderSettings},
    prelude::*,
};
use prefab::lifecycle::{GameplaySessionEntities, GameplaySessionEntity};
use prefab::world_2d::characters::DemoPlayerPrefab;
use prefab::world_2d::demo_level::{DemoBackgroundPrefab, DemoGroundPrefab};
use render_2d::atlases::demo_player_atlas_layout;
use render_2d::camera::DemoWorldCamera2dBundle;

use super::plan::GameplaySpawnPlan;

pub fn default_gameplay_spawn_plan(
    asset_server: &AssetServer,
    atlas_layouts: &mut Assets<TextureAtlasLayout>,
) -> GameplaySpawnPlan {
    let player_atlas_layout = atlas_layouts.add(demo_player_atlas_layout());

    GameplaySpawnPlan::new()
        .with(DemoBackgroundPrefab)
        .with(DemoGroundPrefab::new(asset_server.load_with_settings(
            "2d/static/tilemaps/demo_tileset.png",
            |settings: &mut ImageLoaderSettings| {
                settings.array_layout = Some(ImageArrayLayout::RowCount { rows: 4 });
            },
        )))
        .with(DemoPlayerPrefab::new(
            Vec2::new(0.0, 96.0),
            asset_server.load("2d/animated/characters/demo_player.png"),
            player_atlas_layout,
        ))
}

pub fn spawn_initial_gameplay_plan_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    session_entities: GameplaySessionEntities,
) {
    if !session_entities.is_empty() {
        info!("Initial gameplay spawn plan skipped; session already exists.");
        return;
    }

    commands.spawn((DemoWorldCamera2dBundle::default(), GameplaySessionEntity));

    for prefab in default_gameplay_spawn_plan(&asset_server, &mut atlas_layouts).into_prefabs() {
        prefab.spawn_boxed(&mut commands);
    }

    info!("Initial gameplay spawn plan completed.");
}
