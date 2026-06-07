use bevy::prelude::*;
use components::characters::player::PlayerBundle;

use crate::flow::AppState;

#[derive(Component)]
pub struct GameplayEntity;

pub struct SpawningPlugin;

impl Plugin for SpawningPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Playing), spawn_player)
            .add_systems(OnExit(AppState::Playing), despawn_gameplay_entities);
    }
}

fn spawn_player(mut commands: Commands) {
    commands.spawn((
        PlayerBundle {
            transform: Transform::from_xyz(0.0, -180.0, 10.0),
            ..default()
        },
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
