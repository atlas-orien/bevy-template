use bevy::prelude::*;
use ecs::components::world::gameplay::GameplayEntity;
use prefab::characters::player::PlayerPrefabBundle;

use crate::flow::AppState;

pub struct SpawningPlugin;

impl Plugin for SpawningPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Playing), spawn_player)
            .add_systems(OnExit(AppState::Playing), despawn_gameplay_entities);
    }
}

fn spawn_player(mut commands: Commands) {
    commands.spawn((
        PlayerPrefabBundle::at_position(Vec3::new(0.0, -180.0, 10.0)),
        GameplayEntity,
    ));
}

fn despawn_gameplay_entities(
    mut commands: Commands,
    entities: Query<Entity, With<GameplayEntity>>,
) {
    for entity in &entities {
        commands.entity(entity).despawn();
    }
}
