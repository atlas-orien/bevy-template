use bevy::prelude::*;
use ecs::components::world::gameplay::GameplayEntity;
use prefab::characters::player::PlayerPrefabBundle;

use crate::state::AppState;

pub struct SpawningPlugin;

impl Plugin for SpawningPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Playing), spawn_player);
    }
}

fn spawn_player(mut commands: Commands) {
    commands.spawn((
        PlayerPrefabBundle::at_position(Vec3::new(0.0, -180.0, 10.0)),
        GameplayEntity,
    ));
}
