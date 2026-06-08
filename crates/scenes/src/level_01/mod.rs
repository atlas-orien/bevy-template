use bevy::prelude::*;
use prefab::world_2d::characters::player::spawn_player_2d_prefab;
use prefab::world_2d::environment::{spawn_demo_background_2d_prefab, spawn_main_camera_2d_prefab};

use crate::shared::SceneEntity;

pub fn spawn_level_01_scene(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let camera = spawn_main_camera_2d_prefab(&mut commands);
    commands.entity(camera).insert(SceneEntity);

    for entity in spawn_demo_background_2d_prefab(&mut commands) {
        commands.entity(entity).insert(SceneEntity);
    }

    let player = spawn_player_2d_prefab(
        &mut commands,
        &asset_server,
        &mut texture_atlas_layouts,
        Vec3::new(0.0, -180.0, 10.0),
    );
    commands.entity(player).insert(SceneEntity);
}
