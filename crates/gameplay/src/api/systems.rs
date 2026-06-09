use bevy::prelude::*;
use intent::movement::{MovementIntentQuery, set_movement_intent};
use prefab::identity::{GameplayEntityIdEntities, find_gameplay_entity};
use prefab::lifecycle::{GameplaySessionEntities, despawn_gameplay_prefabs};

use super::channel::{RuntimeRequestInbox, RuntimeUpdateSender, drain_runtime_requests_into};
use super::{RuntimeRequest, RuntimeUpdate};
use crate::state::AppState;

pub fn forward_manager_requests_system(
    inbox: Option<Res<RuntimeRequestInbox>>,
    mut requests: MessageWriter<RuntimeRequest>,
) {
    let Some(inbox) = inbox else {
        return;
    };

    drain_runtime_requests_into(&inbox, &mut requests);
}

pub fn consume_gameplay_requests_system(
    mut commands: Commands,
    mut requests: MessageMutator<RuntimeRequest>,
    mut next_state: ResMut<NextState<AppState>>,
    update_sender: Option<Res<RuntimeUpdateSender>>,
    gameplay_session_entities: GameplaySessionEntities,
    gameplay_id_entities: GameplayEntityIdEntities,
    mut movement_intents: MovementIntentQuery,
) {
    for request in requests.read() {
        match request {
            RuntimeRequest::SpawnPrefab(prefab) => {
                if let Some(prefab) = prefab.take() {
                    prefab.spawn_boxed(&mut commands);
                }
            }
            RuntimeRequest::DespawnEntity(id) => {
                if let Some(entity) = find_gameplay_entity(*id, &gameplay_id_entities) {
                    commands.entity(entity).try_despawn();
                    submit_update(
                        &update_sender,
                        RuntimeUpdate::EntityUnregistered { id: *id },
                    );
                }
            }
            RuntimeRequest::ClearSession => {
                for (_, id) in gameplay_id_entities.iter() {
                    submit_update(
                        &update_sender,
                        RuntimeUpdate::EntityUnregistered { id: *id },
                    );
                }
                despawn_gameplay_prefabs(&mut commands, &gameplay_session_entities);
            }
            RuntimeRequest::ChangeState(state) => {
                next_state.set(*state);
            }
            RuntimeRequest::SetMovementIntent { id, target } => {
                if let Some(entity) = find_gameplay_entity(*id, &gameplay_id_entities) {
                    let _ = set_movement_intent(entity, *target, &mut movement_intents);
                }
            }
        }
    }
}

pub fn sync_gameplay_entities_system(
    update_sender: Option<Res<RuntimeUpdateSender>>,
    gameplay_id_entities: GameplayEntityIdEntities,
) {
    for (_, id) in gameplay_id_entities.iter() {
        submit_update(&update_sender, RuntimeUpdate::EntityRegistered { id: *id });
    }
}

fn submit_update(update_sender: &Option<Res<RuntimeUpdateSender>>, update: RuntimeUpdate) {
    if let Some(update_sender) = update_sender {
        let _ = update_sender.send(update);
    }
}
