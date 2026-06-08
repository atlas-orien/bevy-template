use bevy::prelude::*;
use ecs::components::world::gameplay::GameplayEntity;

use crate::state::AppState;

pub struct CleanupPlugin;

impl Plugin for CleanupPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnExit(AppState::Playing), despawn_gameplay_entities);
    }
}

fn despawn_gameplay_entities(
    mut commands: Commands,
    entities: Query<Entity, With<GameplayEntity>>,
) {
    for entity in &entities {
        commands.entity(entity).despawn();
    }
}
