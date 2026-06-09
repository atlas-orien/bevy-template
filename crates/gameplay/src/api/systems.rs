use bevy::prelude::*;
use intent::movement::{MovementIntentQuery, set_movement_intent};
use prefab::identity::{GameplayEntityIdEntities, find_gameplay_entity};
use prefab::lifecycle::{GameplaySessionEntities, despawn_gameplay_prefabs};

use crate::state::AppState;

use super::channel::{GameplayRequestInbox, GameplayUpdate, GameplayUpdateSender};
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
    update_sender: Option<Res<GameplayUpdateSender>>,
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
                    submit_update(
                        &update_sender,
                        GameplayUpdate::EntityUnregistered { id: *id },
                    );
                }
            }
            GameplayRequest::ClearSession => {
                for (_, id) in gameplay_id_entities.iter() {
                    submit_update(
                        &update_sender,
                        GameplayUpdate::EntityUnregistered { id: *id },
                    );
                }
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

pub fn sync_gameplay_entities_system(
    update_sender: Option<Res<GameplayUpdateSender>>,
    gameplay_id_entities: GameplayEntityIdEntities,
) {
    for (_, id) in gameplay_id_entities.iter() {
        submit_update(&update_sender, GameplayUpdate::EntityRegistered { id: *id });
    }
}

fn submit_update(update_sender: &Option<Res<GameplayUpdateSender>>, update: GameplayUpdate) {
    if let Some(update_sender) = update_sender {
        let _ = update_sender.submit(update);
    }
}
