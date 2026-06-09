use bevy::prelude::*;
use intent::movement::{MovementIntentQuery, set_movement_intent};
use prefab::identity::{GameplayEntityIdEntities, find_gameplay_entity};
use prefab::lifecycle::{GameplaySessionEntities, despawn_gameplay_prefabs};

use crate::state::AppState;

use super::manager::GameplayRequestInbox;
use super::request::GameplayRequest;

pub fn forward_manager_requests_system(
    inbox: Option<Res<GameplayRequestInbox>>,
    mut requests: MessageWriter<GameplayRequest>,
) {
    let Some(inbox) = inbox else {
        return;
    };

    inbox.drain_into(&mut requests);
}

pub fn consume_gameplay_requests_system(
    mut commands: Commands,
    mut requests: MessageMutator<GameplayRequest>,
    mut next_state: ResMut<NextState<AppState>>,
    gameplay_session_entities: GameplaySessionEntities,
    gameplay_id_entities: GameplayEntityIdEntities,
    mut movement_intents: MovementIntentQuery,
) {
    for request in requests.read() {
        match request {
            GameplayRequest::SpawnPrefab(prefab) => {
                if let Some(prefab) = prefab.take() {
                    prefab.spawn_boxed(&mut commands);
                }
            }
            GameplayRequest::DespawnEntity(id) => {
                if let Some(entity) = find_gameplay_entity(*id, &gameplay_id_entities) {
                    commands.entity(entity).try_despawn();
                }
            }
            GameplayRequest::ClearSession => {
                despawn_gameplay_prefabs(&mut commands, &gameplay_session_entities);
            }
            GameplayRequest::ChangeState(state) => {
                next_state.set(*state);
            }
            GameplayRequest::SetMovementIntent { id, target } => {
                if let Some(entity) = find_gameplay_entity(*id, &gameplay_id_entities) {
                    let _ = set_movement_intent(entity, *target, &mut movement_intents);
                }
            }
        }
    }
}
