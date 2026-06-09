use bevy::prelude::*;
use prefab::lifecycle::{GameplaySessionEntities, despawn_gameplay_prefabs};

use crate::state::AppState;

use super::request::GameplayRequest;

pub fn consume_gameplay_requests_system(
    mut commands: Commands,
    mut requests: MessageMutator<GameplayRequest>,
    mut next_state: ResMut<NextState<AppState>>,
    gameplay_session_entities: GameplaySessionEntities,
) {
    for request in requests.read() {
        match request {
            GameplayRequest::SpawnPrefab(prefab) => {
                if let Some(prefab) = prefab.take() {
                    prefab.spawn_boxed(&mut commands);
                }
            }
            GameplayRequest::DespawnEntity(entity) => {
                commands.entity(*entity).try_despawn();
            }
            GameplayRequest::ClearSession => {
                despawn_gameplay_prefabs(&mut commands, &gameplay_session_entities);
            }
            GameplayRequest::ChangeState(state) => {
                next_state.set(*state);
            }
        }
    }
}
