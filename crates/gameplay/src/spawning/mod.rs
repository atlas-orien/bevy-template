use bevy::prelude::*;
use ecs::characters::player::PlayerBundle;

use crate::flow::GameplayState;

#[derive(Component)]
pub struct GameplayEntity;

pub struct SpawningPlugin;

impl Plugin for SpawningPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameplayState::Playing), spawn_player)
            .add_systems(OnExit(GameplayState::Playing), despawn_gameplay_entities);
    }
}

fn spawn_player(mut commands: Commands) {
    commands.spawn((PlayerBundle::default(), GameplayEntity));
}

fn despawn_gameplay_entities(
    mut commands: Commands,
    entities: Query<Entity, With<GameplayEntity>>,
) {
    for entity in &entities {
        commands.entity(entity).despawn();
    }
}
