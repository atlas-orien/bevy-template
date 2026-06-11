use bevy::prelude::*;
use intent::movement::{MovementIntentQuery, set_movement_intent};
use prefab::identity::{GameplayEntityIdEntities, find_gameplay_entity};
use prefab::lifecycle::{GameplaySessionEntities, despawn_gameplay_prefabs};

use super::channel::{RuntimeRequestInbox, RuntimeUpdateSender, drain_runtime_requests_into};
use super::{RuntimeRequestMessage, RuntimeUpdateMessage};
use crate::spawning::runtime::spawn_runtime_prefab;
use crate::state::AppState;

pub fn forward_manager_requests_system(
    inbox: Option<Res<RuntimeRequestInbox>>,
    mut requests: MessageWriter<RuntimeRequestMessage>,
) {
    let Some(inbox) = inbox else {
        return;
    };

    drain_runtime_requests_into(&inbox, &mut requests);
}

pub fn consume_gameplay_requests_system(
    mut commands: Commands,
    mut requests: MessageMutator<RuntimeRequestMessage>,
    mut next_state: ResMut<NextState<AppState>>,
    update_sender: Option<Res<RuntimeUpdateSender>>,
    gameplay_session_entities: GameplaySessionEntities,
    gameplay_id_entities: GameplayEntityIdEntities,
    mut movement_intents: MovementIntentQuery,
) {
    for request in requests.read() {
        match request {
            RuntimeRequestMessage::SpawnPrefab(request) => {
                if let Some(prefab) = request.prefab.take() {
                    let registration = request.registration;
                    let entity = spawn_runtime_prefab(&mut commands, prefab);
                    commands
                        .entity(entity)
                        .insert(registration.gameplay_entity_id);
                    submit_update(
                        &update_sender,
                        RuntimeUpdateMessage::EntityRegistered(registration),
                    );
                }
            }
            RuntimeRequestMessage::DespawnEntity(id) => {
                if let Some(entity) = find_gameplay_entity(*id, &gameplay_id_entities) {
                    commands.entity(entity).try_despawn();
                    submit_update(
                        &update_sender,
                        RuntimeUpdateMessage::entity_unregistered(*id),
                    );
                }
            }
            RuntimeRequestMessage::ClearSession => {
                for (_, id) in gameplay_id_entities.iter() {
                    submit_update(
                        &update_sender,
                        RuntimeUpdateMessage::entity_unregistered(*id),
                    );
                }
                despawn_gameplay_prefabs(&mut commands, &gameplay_session_entities);
            }
            RuntimeRequestMessage::ChangeState(state) => {
                next_state.set(*state);
            }
            RuntimeRequestMessage::SetMovementIntent { id, target } => {
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
        submit_update(&update_sender, RuntimeUpdateMessage::entity_registered(*id));
    }
}

fn submit_update(update_sender: &Option<Res<RuntimeUpdateSender>>, update: RuntimeUpdateMessage) {
    if let Some(update_sender) = update_sender {
        let _ = update_sender.send(update);
    }
}
