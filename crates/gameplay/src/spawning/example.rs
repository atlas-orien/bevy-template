use bevy::prelude::*;
use prefab::world_2d::characters::example::spawn_example_gabe_player_2d_system;

pub fn spawn_example_gameplay_plan_system(
    commands: Commands,
    asset_server: Res<AssetServer>,
    texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    spawn_example_gabe_player_2d_system(commands, asset_server, texture_atlas_layouts);
    info!("Example gameplay spawn plan completed.");
}
